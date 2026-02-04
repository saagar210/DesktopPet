import { useState, useEffect, useRef, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TIMER_PRESETS, DEFAULT_PRESET } from "../lib/constants";
import { invokeQuiet, listenSafe } from "../lib/tauri";
import type { TimerPreset } from "../lib/constants";

type TimerPhase = "idle" | "work" | "break" | "celebrating";

interface PomodoroState {
  phase: TimerPhase;
  secondsLeft: number;
  totalSeconds: number;
  sessionId: string | null;
  sessionsCompleted: number;
  preset: TimerPreset;
}

export function usePomodoro() {
  const [state, setState] = useState<PomodoroState>({
    phase: "idle",
    secondsLeft: TIMER_PRESETS[DEFAULT_PRESET].work,
    totalSeconds: TIMER_PRESETS[DEFAULT_PRESET].work,
    sessionId: null,
    sessionsCompleted: 0,
    preset: DEFAULT_PRESET,
  });
  const [paused, setPaused] = useState(false);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const clearTimer = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  }, []);

  const setPreset = useCallback(
    (preset: TimerPreset) => {
      if (state.phase !== "idle") return;
      setState((s) => ({
        ...s,
        preset,
        secondsLeft: TIMER_PRESETS[preset].work,
        totalSeconds: TIMER_PRESETS[preset].work,
      }));
    },
    [state.phase]
  );

  const start = useCallback(async () => {
    const p = TIMER_PRESETS[state.preset];
    try {
      const result = await invoke<{ id: string }>("start_pomodoro", {
        workDuration: p.work,
        breakDuration: p.break,
      });
      setState((s) => ({
        ...s,
        phase: "work",
        secondsLeft: p.work,
        totalSeconds: p.work,
        sessionId: result.id,
      }));
      setPaused(false);
    } catch (e) {
      console.error("[start_pomodoro]", e);
    }
  }, [state.preset]);

  const pause = useCallback(() => setPaused(true), []);
  const resume = useCallback(() => setPaused(false), []);

  const reset = useCallback(() => {
    clearTimer();
    const p = TIMER_PRESETS[state.preset];
    invokeQuiet("set_pet_animation", { animation: "idle" });
    setState((s) => ({
      ...s,
      phase: "idle",
      secondsLeft: p.work,
      totalSeconds: p.work,
      sessionId: null,
    }));
    setPaused(false);
  }, [clearTimer, state.preset]);

  // Tick timer
  useEffect(() => {
    clearTimer();
    if (state.phase === "idle" || state.phase === "celebrating" || paused) return;

    intervalRef.current = setInterval(() => {
      setState((prev) => {
        if (prev.secondsLeft <= 1) {
          if (prev.phase === "work") {
            // Work complete → celebrate (Rust awards coins + updates pet + tracks goals)
            if (prev.sessionId) {
              invokeQuiet("complete_pomodoro", { sessionId: prev.sessionId });
            }
            return {
              ...prev,
              phase: "celebrating" as TimerPhase,
              secondsLeft: 0,
              sessionsCompleted: prev.sessionsCompleted + 1,
            };
          } else if (prev.phase === "break") {
            // Break complete → idle
            invokeQuiet("set_pet_animation", { animation: "idle" });
            const p = TIMER_PRESETS[prev.preset];
            return {
              ...prev,
              phase: "idle" as TimerPhase,
              secondsLeft: p.work,
              totalSeconds: p.work,
              sessionId: null,
            };
          }
        }
        return { ...prev, secondsLeft: prev.secondsLeft - 1 };
      });
    }, 1000);

    return clearTimer;
  }, [state.phase, paused, clearTimer]);

  // Auto-transition: celebrating → break after 3s
  useEffect(() => {
    if (state.phase !== "celebrating") return;
    const timeout = setTimeout(() => {
      const p = TIMER_PRESETS[state.preset];
      invokeQuiet("set_pet_animation", { animation: "break" });
      // Track break goal (absolute count — each completed work session starts a break)
      invokeQuiet("update_goal_progress", {
        goalId: "breaks",
        progress: state.sessionsCompleted,
      });
      setState((s) => ({
        ...s,
        phase: "break",
        secondsLeft: p.break,
        totalSeconds: p.break,
      }));
    }, 3000);
    return () => clearTimeout(timeout);
  }, [state.phase, state.preset, state.sessionsCompleted]);

  // Listen for tray start
  useEffect(() => {
    let unlisten = () => {};
    listenSafe("tray-start-pomodoro", () => {
      if (state.phase === "idle") {
        start();
      }
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten();
    };
  }, [state.phase, start]);

  return {
    ...state,
    paused,
    start,
    pause,
    resume,
    reset,
    setPreset,
  };
}
