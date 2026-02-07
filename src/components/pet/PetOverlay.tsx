import { useEffect, useState, useCallback, useRef } from "react";
import { EVENT_PET_STATE_CHANGED } from "../../lib/events";
import { invokeMaybe, invokeOr, listenSafe, startDraggingSafe } from "../../lib/tauri";
import type { PetState } from "../../store/types";
import { PetCharacter } from "./PetCharacter";

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
  const clickTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

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

    let cancelled = false;
    let unlisten = () => {};
    listenSafe<PetState>(EVENT_PET_STATE_CHANGED, (event) => {
      setPet(event.payload);
    }).then((fn) => {
      if (cancelled) {
        fn();
        return;
      }
      unlisten = fn;
    });

    return () => {
      cancelled = true;
      unlisten();
    };
  }, []);

  const handleClick = useCallback(() => {
    setAnimOverride("clicked");
    if (clickTimeoutRef.current) {
      clearTimeout(clickTimeoutRef.current);
    }
    clickTimeoutRef.current = setTimeout(() => setAnimOverride(null), 400);
    // Quick pat interaction on click
    void invokeMaybe<PetState>("pet_interact", { action: "pet" });
  }, []);

  const handleDrag = useCallback(() => {
    startDraggingSafe();
  }, []);

  useEffect(
    () => () => {
      if (clickTimeoutRef.current) {
        clearTimeout(clickTimeoutRef.current);
      }
    },
    []
  );

  const currentAnim = animOverride ?? pet.animationState;
  const animClass = `anim-${currentAnim}`;
  const skinClass =
    pet.skin === "neon"
      ? "saturate-150 brightness-110"
      : pet.skin === "pixel"
        ? "contrast-125"
        : pet.skin === "plush"
          ? "brightness-95 saturate-75"
          : "";

  return (
    <div
      className="w-full h-full flex items-center justify-center cursor-grab active:cursor-grabbing"
      onClick={handleClick}
      onMouseDown={handleDrag}
    >
      <div className={`${animClass} ${skinClass}`}>
        <PetCharacter stage={pet.currentStage} accessories={pet.accessories} />
      </div>
    </div>
  );
}
