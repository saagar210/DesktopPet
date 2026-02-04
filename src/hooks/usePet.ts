import { useCallback, useState, useEffect } from "react";
import { EVOLUTION_THRESHOLDS } from "../lib/constants";
import { EVENT_PET_STATE_CHANGED } from "../lib/events";
import { invokeMaybe, invokeOr, listenSafe } from "../lib/tauri";
import type { PetState } from "../store/types";

const defaultPet: PetState = {
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
};

export function usePet() {
  const [pet, setPet] = useState<PetState>(defaultPet);

  useEffect(() => {
    invokeOr<PetState>("get_pet_state", undefined, defaultPet).then(setPet);

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

  const interact = useCallback(async (action: string) => {
    const updated = await invokeMaybe<PetState>("pet_interact", { action });
    if (!updated) return null;
    setPet(updated);
    return updated;
  }, []);

  const setCustomization = useCallback(async (skin?: string, scene?: string) => {
    const updated = await invokeMaybe<PetState>("set_pet_customization", { skin, scene });
    if (!updated) return null;
    setPet(updated);
    return updated;
  }, []);

  return {
    pet,
    stageName,
    progressToNext,
    stageProgress,
    stageSpan,
    interact,
    setCustomization,
  };
}
