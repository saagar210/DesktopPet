import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { DailyGoal } from "../store/types";

export function useGoals() {
  const [goals, setGoals] = useState<DailyGoal[]>([]);

  useEffect(() => {
    invoke<DailyGoal[]>("get_daily_goals").then(setGoals);

    const unlisten = listen<DailyGoal[]>("goals-changed", (event) => {
      setGoals(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return { goals };
}
