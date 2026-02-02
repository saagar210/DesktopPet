export function generateId(): string {
  return crypto.randomUUID();
}

export function todayDateString(): string {
  return new Date().toISOString().slice(0, 10);
}

export function formatTime(totalSeconds: number): string {
  const m = Math.floor(totalSeconds / 60);
  const s = totalSeconds % 60;
  return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}
