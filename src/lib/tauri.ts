import { invoke } from "@tauri-apps/api/core";
import { listen, type EventCallback } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const noop = () => {};

function warn(tag: string, error: unknown) {
  console.warn(`[${tag}]`, error);
}

export async function invokeOr<T>(
  cmd: string,
  args: Record<string, unknown> | undefined,
  fallback: T,
  tag = cmd
) {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    warn(tag, error);
    return fallback;
  }
}

export async function invokeMaybe<T>(
  cmd: string,
  args?: Record<string, unknown>,
  tag = cmd
) {
  try {
    return await invoke<T>(cmd, args);
  } catch (error) {
    warn(tag, error);
    return null;
  }
}

export function invokeQuiet(cmd: string, args?: Record<string, unknown>, tag = cmd) {
  try {
    invoke(cmd, args).catch((error) => warn(tag, error));
  } catch (error) {
    warn(tag, error);
  }
}

export async function listenSafe<T>(
  event: string,
  handler: EventCallback<T>,
  tag = `listen:${event}`
) {
  try {
    return await listen<T>(event, handler);
  } catch (error) {
    warn(tag, error);
    return noop;
  }
}

export function startDraggingSafe() {
  try {
    getCurrentWindow()
      .startDragging()
      .catch((error) => warn("startDragging", error));
  } catch (error) {
    warn("startDragging", error);
  }
}

// Achievement commands
export async function getAchievements() {
  return invokeOr("get_achievements", undefined, [], "achievements:get");
}

export async function getAchievementStats() {
  return invokeOr("get_achievement_stats", undefined, {}, "achievements:stats");
}

export async function checkAchievementProgress() {
  return invokeMaybe<string[]>("check_achievement_progress", undefined, "achievements:check");
}

export async function checkTimeAchievement(completionHour: number) {
  return invokeMaybe<string[]>(
    "check_time_achievement",
    { completionHour },
    "achievements:check-time"
  );
}
