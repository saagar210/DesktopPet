import { useCallback, useEffect, useState } from "react";
import { invokeMaybe, invokeOr } from "../lib/tauri";
import type { PetEvent, PetQuest } from "../store/types";

export function usePetEvents() {
  const [events, setEvents] = useState<PetEvent[]>([]);
  const [activeQuest, setActiveQuest] = useState<PetQuest | null>(null);

  const refresh = useCallback(() => {
    Promise.all([
      invokeOr<PetEvent[]>("get_pet_events", undefined, []),
      invokeOr<PetQuest | null>("get_pet_active_quest", undefined, null),
    ]).then(([latestEvents, quest]) => {
      setEvents(latestEvents);
      setActiveQuest(quest);
    });
  }, []);

  useEffect(() => {
    refresh();
  }, [refresh]);

  const rollEvent = useCallback(async () => {
    const event = await invokeMaybe<PetEvent>("roll_pet_event");
    if (!event) return null;
    refresh();
    return event;
  }, [refresh]);

  const resolveEvent = useCallback(async (eventId: string) => {
    const updated = await invokeMaybe<PetEvent[]>("resolve_pet_event", { eventId });
    if (!updated) return events;
    setEvents(updated);
    return updated;
  }, [events]);

  return { events, activeQuest, refresh, rollEvent, resolveEvent };
}
