import { useState, useEffect } from "react";
import { invokeOr, listenSafe } from "../lib/tauri";
import type { DailyGoal } from "../store/types";

export function useGoals() {
  const [goals, setGoals] = useState<DailyGoal[]>([]);

  useEffect(() => {
    invokeOr<DailyGoal[]>("get_daily_goals", undefined, []).then(setGoals);

    let unlisten = () => {};
    listenSafe<DailyGoal[]>("goals-changed", (event) => {
      setGoals(event.payload);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten();
    };
  }, []);

  return { goals };
}
