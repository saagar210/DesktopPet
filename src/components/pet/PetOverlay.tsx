import { useEffect, useState, useCallback, useRef } from "react";
import { EVENT_PET_STATE_CHANGED, EVENT_SETTINGS_CHANGED } from "../../lib/events";
import { listenForChillSignals } from "../../lib/chill";
import { invokeMaybe, invokeOr, listenSafe, startDraggingSafe } from "../../lib/tauri";
import type { PetState, Settings } from "../../store/types";
import { PetCharacter } from "./PetCharacter";

export function PetOverlay() {
  const [pet, setPet] = useState<PetState>({
    currentStage: 0,
    animationState: "idle",
    accessories: [],
    totalPomodoros: 0,
    speciesId: "penguin",
    evolutionThresholds: [0, 5, 15],
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
  const [settings, setSettings] = useState<Pick<
    Settings,
    "animationBudget" | "contextAwareChillEnabled" | "focusModeEnabled"
  >>({
    animationBudget: "medium",
    contextAwareChillEnabled: true,
    focusModeEnabled: false,
  });
  const [isContextChilled, setIsContextChilled] = useState(false);
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
        speciesId: "penguin",
        evolutionThresholds: [0, 5, 15],
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

  useEffect(() => {
    invokeOr<Settings>(
      "get_settings",
      undefined,
      {
        timerPreset: "standard",
        notificationsEnabled: true,
        toastNotificationsEnabled: false,
        trayBadgeEnabled: true,
        notificationWhitelist: ["session_complete", "guardrail_alert"],
        soundsEnabled: false,
        soundVolume: 0.7,
        quietModeEnabled: true,
        focusModeEnabled: false,
        animationBudget: "medium",
        contextAwareChillEnabled: true,
        chillOnFullscreen: true,
        chillOnMeetings: true,
        chillOnHeavyTyping: true,
        meetingHosts: ["zoom.us", "meet.google.com", "teams.microsoft.com"],
        heavyTypingThresholdCpm: 220,
        enabledSeasonalPacks: [],
        uiTheme: "sunrise",
        petSkin: "classic",
        petScene: "meadow",
        focusGuardrailsEnabled: false,
        focusGuardrailsWorkOnly: true,
        focusAllowlist: [],
        focusBlocklist: [],
      }
    ).then((loaded) =>
      setSettings({
        animationBudget: loaded.animationBudget,
        contextAwareChillEnabled: loaded.contextAwareChillEnabled,
        focusModeEnabled: loaded.focusModeEnabled,
      })
    );

    let cancelled = false;
    let unlisten = () => {};
    listenSafe<Settings>(EVENT_SETTINGS_CHANGED, (event) => {
      setSettings({
        animationBudget: event.payload.animationBudget,
        contextAwareChillEnabled: event.payload.contextAwareChillEnabled,
        focusModeEnabled: event.payload.focusModeEnabled,
      });
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

  useEffect(() => {
    return listenForChillSignals((signals) => {
      setIsContextChilled(
        signals.fullscreen || signals.heavyTyping || signals.meeting || signals.focusMode
      );
    });
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

  const shouldChill = settings.contextAwareChillEnabled && isContextChilled;
  const currentAnim = shouldChill ? "idle" : animOverride ?? pet.animationState;
  const animClass = `anim-${currentAnim}`;
  const budgetClass = `anim-budget-${settings.animationBudget}`;
  const skinClass =
    pet.skin === "neon"
      ? "saturate-150 brightness-110"
      : pet.skin === "pixel"
        ? "contrast-125"
        : pet.skin === "plush"
          ? "brightness-95 saturate-75"
          : "";
  const accessoryBehaviorClasses = [
    pet.accessories.includes("scarf") ? "behavior-scarf" : "",
    pet.accessories.includes("sunglasses") ? "behavior-sunglasses" : "",
    pet.accessories.includes("bow_tie") ? "behavior-bowtie" : "",
    pet.accessories.includes("party_hat") ? "behavior-partyhat" : "",
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <div
      className="w-full h-full flex items-center justify-center cursor-grab active:cursor-grabbing"
      onClick={handleClick}
      onMouseDown={handleDrag}
    >
      <div
        className={`${animClass} ${budgetClass} ${skinClass} ${shouldChill ? "chill-dim" : ""}`}
      >
        <div className={accessoryBehaviorClasses}>
          <PetCharacter
            stage={pet.currentStage}
            accessories={pet.accessories}
            speciesId={pet.speciesId}
          />
        </div>
      </div>
    </div>
  );
}
