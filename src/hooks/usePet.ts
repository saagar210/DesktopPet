import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { EVOLUTION_THRESHOLDS } from "../lib/constants";
import type { PetState } from "../store/types";

const defaultPet: PetState = {
  currentStage: 0,
  animationState: "idle",
  accessories: [],
  totalPomodoros: 0,
};

export function usePet() {
  const [pet, setPet] = useState<PetState>(defaultPet);

  useEffect(() => {
    invoke<PetState>("get_pet_state").then(setPet);

    const unlisten = listen<PetState>("pet-state-changed", (event) => {
      setPet(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const nextThreshold =
    pet.currentStage === 0
      ? EVOLUTION_THRESHOLDS.stage1
      : pet.currentStage === 1
        ? EVOLUTION_THRESHOLDS.stage2
        : null;

  const prevThreshold = pet.currentStage === 1 ? EVOLUTION_THRESHOLDS.stage1 : 0;

  const stageSpan = nextThreshold !== null ? nextThreshold - prevThreshold : 0;
  const stageProgress = nextThreshold !== null ? pet.totalPomodoros - prevThreshold : 0;

  const progressToNext = stageSpan > 0 ? stageProgress / stageSpan : 1;

  const stageName =
    pet.currentStage === 0 ? "Blob" : pet.currentStage === 1 ? "Buddy" : "Champion";

  return { pet, stageName, progressToNext, stageProgress, stageSpan };
}
