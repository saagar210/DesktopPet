import { useEffect, useState, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
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
  });
  const [animOverride, setAnimOverride] = useState<string | null>(null);
  const [particles, setParticles] = useState<Particle[]>([]);
  const particleId = useRef(0);

  useEffect(() => {
    invoke<PetState>("get_pet_state").then(setPet);

    const unlisten = listen<PetState>("pet-state-changed", (event) => {
      setPet(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // Idle particle spawner
  const currentAnim = animOverride ?? pet.animationState;
  useEffect(() => {
    if (currentAnim !== "idle") {
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
      setTimeout(() => {
        setParticles((prev) => prev.filter((pp) => pp.id !== id));
      }, 2000);
    }, 800);

    return () => clearInterval(interval);
  }, [currentAnim]);

  const handleClick = useCallback(() => {
    setAnimOverride("clicked");
    setTimeout(() => setAnimOverride(null), 400);
  }, []);

  const handleDrag = useCallback(() => {
    getCurrentWindow().startDragging();
  }, []);

  const animClass = `anim-${currentAnim}`;

  return (
    <div
      className="w-full h-full flex items-center justify-center cursor-grab active:cursor-grabbing relative"
      onClick={handleClick}
      onMouseDown={handleDrag}
    >
      <div className={animClass}>
        <PetCharacter stage={pet.currentStage} accessories={pet.accessories} />
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
