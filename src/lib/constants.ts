export const TIMER_PRESETS = {
  short: { work: 15 * 60, break: 5 * 60, label: "15 / 5" },
  standard: { work: 25 * 60, break: 5 * 60, label: "25 / 5" },
  long: { work: 50 * 60, break: 10 * 60, label: "50 / 10" },
} as const;

export type TimerPreset = keyof typeof TIMER_PRESETS;

export const DEFAULT_PRESET: TimerPreset = "standard";

export const COINS_PER_POMODORO = 10;

export const EVOLUTION_THRESHOLDS = {
  stage1: 5,
  stage2: 15,
} as const;

export const SHOP_ITEMS = [
  { id: "party_hat", name: "Party Hat", cost: 30, icon: "🎩" },
  { id: "bow_tie", name: "Bow Tie", cost: 20, icon: "🎀" },
  { id: "sunglasses", name: "Sunglasses", cost: 25, icon: "🕶️" },
  { id: "scarf", name: "Scarf", cost: 35, icon: "🧣" },
  { id: "apple", name: "Apple", cost: 5, icon: "🍎" },
  { id: "cookie", name: "Cookie", cost: 10, icon: "🍪" },
] as const;

export type ShopItemId = (typeof SHOP_ITEMS)[number]["id"];

export const ANIMATION_STATES = [
  "idle",
  "working",
  "break",
  "celebrating",
  "evolving",
  "clicked",
] as const;

export type AnimationState = (typeof ANIMATION_STATES)[number];

export const DEFAULT_DAILY_GOALS = [
  { id: "pomodoros", description: "Complete 4 pomodoros", target: 4 },
  { id: "breaks", description: "Take 3 breaks", target: 3 },
  { id: "tasks", description: "Complete 2 tasks", target: 2 },
  { id: "focus_minutes", description: "Focus for 60 minutes", target: 60 },
] as const;
