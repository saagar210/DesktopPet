import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { PetPanel } from "../PetPanel";
import type { PetState } from "../../../store/types";

const pet: PetState = {
  currentStage: 1,
  animationState: "idle",
  accessories: [],
  totalPomodoros: 6,
  speciesId: "penguin",
  evolutionThresholds: [0, 5, 15],
  mood: "content",
  energy: 80,
  hunger: 20,
  cleanliness: 80,
  affection: 55,
  personality: "balanced",
  evolutionPath: "companion",
  skin: "classic",
  scene: "meadow",
  lastInteraction: null,
  lastCareUpdateAt: new Date().toISOString(),
};

describe("PetPanel smoke flow", () => {
  it("captures photo booth card on demand", async () => {
    const user = userEvent.setup();
    const onCaptureCard = vi.fn(async () => "Pet card saved.");

    render(
      <PetPanel
        pet={pet}
        events={[]}
        activeQuest={null}
        interactionVerbs={[
          { id: "pet", label: "Pat" },
          { id: "feed", label: "Feed" },
          { id: "play", label: "Play" },
          { id: "nap", label: "Nap" },
          { id: "clean", label: "Clean" },
          { id: "train", label: "Train" },
        ]}
        onInteract={vi.fn()}
        onCaptureCard={onCaptureCard}
        onRollEvent={vi.fn()}
        onResolveEvent={vi.fn()}
      />
    );

    await user.click(screen.getByRole("button", { name: "Photo Booth: Save Pet Card" }));
    expect(onCaptureCard).toHaveBeenCalledTimes(1);
    await screen.findByText("Pet card saved.");
  });
});
