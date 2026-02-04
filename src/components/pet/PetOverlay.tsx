import { useEffect, useState, useCallback, useRef, type MouseEvent } from "react";
import { EVENT_PET_STATE_CHANGED } from "../../lib/events";
import { invokeMaybe, invokeOr, listenSafe, startDraggingSafe } from "../../lib/tauri";
import { usePetEvents } from "../../hooks/usePetEvents";
import type { PetState } from "../../store/types";
import { PetCharacter } from "./PetCharacter";

interface Particle {
  id: number;
  x: number;
  y: number;
  char: string;
}

const PARTICLE_CHARS = ["\u2728", "\u2b50", "\u00b7", "\u2022"];

export function PetOverlay() {
  const [pet, setPet] = useState<PetState>({
    currentStage: 0,
    animationState: "idle",
    accessories: [],
    totalPomodoros: 0,
    mood: "content",
    energy: 80,
    hunger: 20,
    cleanliness: 80,
    affection: 50,
    personality: "balanced",
    evolutionPath: "undetermined",
    skin: "classic",
    scene: "meadow",
    lastInteraction: null,
    lastCareUpdateAt: new Date().toISOString(),
  });
  const [animOverride, setAnimOverride] = useState<string | null>(null);
  const [particles, setParticles] = useState<Particle[]>([]);
  const { events, activeQuest, refresh: refreshEvents, rollEvent, resolveEvent } = usePetEvents();
  const particleId = useRef(0);
  const particleTimeouts = useRef<Set<ReturnType<typeof setTimeout>>>(new Set());
  const clickTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const clearParticleTimeouts = useCallback(() => {
    particleTimeouts.current.forEach((timeoutId) => clearTimeout(timeoutId));
    particleTimeouts.current.clear();
  }, []);

  useEffect(() => {
    invokeOr<PetState>(
      "get_pet_state",
      undefined,
      {
        currentStage: 0,
        animationState: "idle",
        accessories: [],
        totalPomodoros: 0,
        mood: "content",
        energy: 80,
        hunger: 20,
        cleanliness: 80,
        affection: 50,
        personality: "balanced",
        evolutionPath: "undetermined",
        skin: "classic",
        scene: "meadow",
        lastInteraction: null,
        lastCareUpdateAt: new Date().toISOString(),
      }
    ).then(setPet);

    let unlisten = () => {};
    listenSafe<PetState>(EVENT_PET_STATE_CHANGED, (event) => {
      setPet(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten();
    };
  }, []);

  // Idle particle spawner
  const currentAnim = animOverride ?? pet.animationState;
  useEffect(() => {
    if (currentAnim !== "idle") {
      clearParticleTimeouts();
      setParticles([]);
      return;
    }

    const interval = setInterval(() => {
      const id = particleId.current++;
      const p: Particle = {
        id,
        x: 140 + Math.random() * 120,
        y: 120 + Math.random() * 80,
        char: PARTICLE_CHARS[Math.floor(Math.random() * PARTICLE_CHARS.length)],
      };
      setParticles((prev) => [...prev.slice(-5), p]);
      const timeoutId = setTimeout(() => {
        setParticles((prev) => prev.filter((pp) => pp.id !== id));
        particleTimeouts.current.delete(timeoutId);
      }, 2000);
      particleTimeouts.current.add(timeoutId);
    }, 800);

    return () => {
      clearInterval(interval);
      clearParticleTimeouts();
    };
  }, [clearParticleTimeouts, currentAnim]);

  const handleClick = useCallback(() => {
    setAnimOverride("clicked");
    if (clickTimeoutRef.current) {
      clearTimeout(clickTimeoutRef.current);
    }
    clickTimeoutRef.current = setTimeout(() => setAnimOverride(null), 400);
  }, []);

  const handleDrag = useCallback((event: MouseEvent<HTMLDivElement>) => {
    const target = event.target as HTMLElement;
    if (target.closest("button")) return;
    startDraggingSafe();
  }, []);

  const animClass = `anim-${currentAnim}`;
  const activeEvent = events.find((event) => !event.resolved);
  const sceneClass =
    pet.scene === "forest"
      ? "bg-gradient-to-b from-green-100 to-emerald-200"
      : pet.scene === "space"
        ? "bg-gradient-to-b from-slate-900 to-indigo-950"
        : pet.scene === "cozy_room"
          ? "bg-gradient-to-b from-amber-100 to-orange-200"
          : "bg-gradient-to-b from-sky-100 to-cyan-200";
  const skinClass =
    pet.skin === "neon"
      ? "saturate-150 brightness-110"
      : pet.skin === "pixel"
        ? "contrast-125"
        : pet.skin === "plush"
          ? "brightness-95 saturate-75"
          : "";

  const interact = useCallback(async (action: string) => {
    const updated = await invokeMaybe<PetState>("pet_interact", { action });
    if (updated) {
      setPet(updated);
      setAnimOverride(updated.animationState);
      if (clickTimeoutRef.current) {
        clearTimeout(clickTimeoutRef.current);
      }
      clickTimeoutRef.current = setTimeout(() => setAnimOverride(null), 450);
      refreshEvents();
    }
  }, [refreshEvents]);

  useEffect(
    () => () => {
      if (clickTimeoutRef.current) {
        clearTimeout(clickTimeoutRef.current);
      }
      clearParticleTimeouts();
    },
    [clearParticleTimeouts]
  );

  return (
    <div
      className={`w-full h-full flex items-center justify-center cursor-grab active:cursor-grabbing relative overflow-hidden ${sceneClass}`}
      onClick={handleClick}
      onMouseDown={handleDrag}
    >
      <div className={`relative z-10 ${animClass} ${skinClass}`}>
        <PetCharacter stage={pet.currentStage} accessories={pet.accessories} />
      </div>
      <div className="absolute top-2 left-2 right-2 z-20 px-2 py-1 rounded-md bg-white/70 backdrop-blur text-[10px] text-gray-700 flex items-center justify-between">
        <span>Mood: {pet.mood}</span>
        <span>
          Energy {pet.energy}% • {pet.evolutionPath}
        </span>
      </div>
      {activeEvent && (
        <div className="absolute top-10 left-2 right-2 z-20 px-2 py-1 rounded-md bg-white/75 backdrop-blur text-[10px] text-gray-700">
          <span className="font-medium capitalize">{activeEvent.kind}:</span> {activeEvent.description}
        </div>
      )}
      {activeQuest && (
        <div className="absolute top-[68px] left-2 right-2 z-20 px-2 py-1 rounded-md bg-indigo-50/90 border border-indigo-200 text-[10px] text-indigo-900">
          <div className="font-medium">{activeQuest.title}</div>
          <div>
            {activeQuest.completedSessions}/{activeQuest.targetSessions} sessions • +{activeQuest.rewardCoins} coins
          </div>
        </div>
      )}
      <div className="absolute top-[106px] left-2 right-2 z-20 px-2 py-1 rounded-md bg-white/80 backdrop-blur text-[10px] text-gray-700 grid grid-cols-2 gap-x-2 gap-y-1">
        <span>Energy {pet.energy}%</span>
        <span>Hunger {pet.hunger}%</span>
        <span>Clean {pet.cleanliness}%</span>
        <span>Bond {pet.affection}%</span>
      </div>
      <div className="absolute bottom-2 left-2 right-2 z-20 flex gap-1">
        {[
          { action: "pet", label: "Pat" },
          { action: "feed", label: "Feed" },
          { action: "play", label: "Play" },
          { action: "nap", label: "Nap" },
          { action: "clean", label: "Clean" },
          { action: "train", label: "Train" },
          { action: "quest", label: "Quest" },
        ].map((item) => (
          <button
            key={item.action}
            type="button"
            onMouseDown={(event) => event.stopPropagation()}
            onClick={(event) => {
              event.stopPropagation();
              if (item.action === "quest") {
                void rollEvent();
                return;
              }
              void interact(item.action);
            }}
            className="flex-1 rounded-md bg-white/80 hover:bg-white text-[10px] text-gray-700 px-1 py-1"
          >
            {item.label}
          </button>
        ))}
      </div>
      <div className="absolute bottom-12 left-2 right-2 z-20 flex flex-col gap-1">
        {events.slice(0, 3).map((entry) => (
          <div
            key={entry.id}
            className="px-2 py-1 rounded-md bg-black/15 text-[10px] text-white flex items-center justify-between gap-2"
          >
            <span className={entry.resolved ? "opacity-60 line-through" : ""}>
              {entry.description}
            </span>
            {!entry.resolved && (
              <button
                type="button"
                onMouseDown={(event) => event.stopPropagation()}
                onClick={(event) => {
                  event.stopPropagation();
                  void resolveEvent(entry.id);
                }}
                className="rounded px-1 py-0.5 bg-white/80 text-[9px] text-gray-700"
              >
                done
              </button>
            )}
          </div>
        ))}
      </div>
      {particles.map((p) => (
        <span
          key={p.id}
          className="idle-particle text-xs"
          style={{
            left: p.x,
            top: p.y,
            opacity: 0.7,
          }}
        >
          {p.char}
        </span>
      ))}
    </div>
  );
}
