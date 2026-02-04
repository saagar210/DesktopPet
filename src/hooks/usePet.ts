import { useState, useEffect } from "react";
import { EVOLUTION_THRESHOLDS } from "../lib/constants";
import { invokeOr, listenSafe } from "../lib/tauri";
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
    invokeOr<PetState>("get_pet_state", undefined, defaultPet).then(setPet);

    let unlisten = () => {};
    listenSafe<PetState>("pet-state-changed", (event) => {
      setPet(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten();
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
