import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Task } from "../store/types";

export function useTasks() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    invoke<Task[]>("get_tasks").then(setTasks);
  }, []);

  const addTask = useCallback(async (title: string) => {
    const task = await invoke<Task>("add_task", { title });
    setTasks((prev) => [...prev, task]);
    return task;
  }, []);

  const toggleTask = useCallback(async (taskId: string) => {
    const updated = await invoke<Task[]>("toggle_task", { taskId });
    setTasks(updated);
  }, []);

  const deleteTask = useCallback(async (taskId: string) => {
    const updated = await invoke<Task[]>("delete_task", { taskId });
    setTasks(updated);
  }, []);

  return { tasks, addTask, toggleTask, deleteTask };
}
