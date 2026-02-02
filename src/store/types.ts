import type { AnimationState, ShopItemId, TimerPreset } from "../lib/constants";

export interface PetState {
  currentStage: number;
  animationState: AnimationState;
  accessories: ShopItemId[];
  totalPomodoros: number;
}

export interface PomodoroSession {
  id: string;
  startedAt: string;
  completedAt: string | null;
  workDuration: number;
  breakDuration: number;
}

export interface CoinBalance {
  total: number;
  spent: number;
}

export interface Task {
  id: string;
  title: string;
  completed: boolean;
  createdAt: string;
}

export interface DailyGoal {
  id: string;
  description: string;
  target: number;
  progress: number;
  date: string;
}

export interface Settings {
  timerPreset: TimerPreset;
}

export interface AppState {
  pet: PetState;
  coins: CoinBalance;
  tasks: Task[];
  goals: DailyGoal[];
  sessions: PomodoroSession[];
  settings: Settings;
}
