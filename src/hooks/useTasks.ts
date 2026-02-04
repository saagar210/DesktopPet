import { useState, useEffect, useCallback } from "react";
import { invokeMaybe, invokeOr } from "../lib/tauri";
import type { Task } from "../store/types";

export function useTasks() {
  const [tasks, setTasks] = useState<Task[]>([]);

  useEffect(() => {
    invokeOr<Task[]>("get_tasks", undefined, []).then(setTasks);
  }, []);

  const addTask = useCallback(async (title: string) => {
    const task = await invokeMaybe<Task>("add_task", { title });
    if (!task) return null;
    setTasks((prev) => [...prev, task]);
    return task;
  }, []);

  const toggleTask = useCallback(async (taskId: string) => {
    const updated = await invokeMaybe<Task[]>("toggle_task", { taskId });
    if (!updated) return;
    setTasks(updated);
  }, []);

  const deleteTask = useCallback(async (taskId: string) => {
    const updated = await invokeMaybe<Task[]>("delete_task", { taskId });
    if (!updated) return;
    setTasks(updated);
  }, []);

  return { tasks, addTask, toggleTask, deleteTask };
}
