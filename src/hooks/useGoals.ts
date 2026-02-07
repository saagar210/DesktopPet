import { useState, useEffect } from "react";
import { EVENT_GOALS_CHANGED } from "../lib/events";
import { invokeOr, listenSafe } from "../lib/tauri";
import type { DailyGoal } from "../store/types";

export function useGoals() {
  const [goals, setGoals] = useState<DailyGoal[]>([]);

  useEffect(() => {
    invokeOr<DailyGoal[]>("get_daily_goals", undefined, []).then(setGoals);

    let cancelled = false;
    let unlisten = () => {};
    listenSafe<DailyGoal[]>(EVENT_GOALS_CHANGED, (event) => {
      setGoals(event.payload);
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

  return { goals };
}
