import { useState } from "react";
import type { Task } from "../../store/types";

interface Props {
  tasks: Task[];
  onAdd: (title: string) => void;
  onToggle: (id: string) => void;
  onDelete: (id: string) => void;
}

export function TaskList({ tasks, onAdd, onToggle, onDelete }: Props) {
  const [input, setInput] = useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const title = input.trim();
    if (!title) return;
    onAdd(title);
    setInput("");
  };

  return (
    <div className="flex flex-col gap-3">
      <form onSubmit={handleSubmit} className="flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder="Add a task..."
          className="flex-1 px-3 py-2 text-sm border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-300"
        />
        <button
          type="submit"
          className="px-3 py-2 bg-blue-500 text-white text-sm rounded-lg hover:bg-blue-600 transition-colors"
        >
          Add
        </button>
      </form>

      {tasks.length === 0 ? (
        <p className="text-sm text-gray-400 text-center py-4">No tasks yet</p>
      ) : (
        <ul className="flex flex-col gap-1">
          {tasks.map((task) => (
            <li
              key={task.id}
              className="flex items-center gap-2 p-2 bg-white rounded-lg border border-gray-100"
            >
              <button
                onClick={() => onToggle(task.id)}
                className={`w-5 h-5 rounded border-2 flex items-center justify-center flex-shrink-0 transition-colors ${
                  task.completed
                    ? "bg-green-500 border-green-500 text-white"
                    : "border-gray-300 hover:border-blue-400"
                }`}
              >
                {task.completed && (
                  <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
                    <path d="M2 6L5 9L10 3" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
                  </svg>
                )}
              </button>
              <span
                className={`flex-1 text-sm ${task.completed ? "text-gray-400 line-through" : "text-gray-700"}`}
              >
                {task.title}
              </span>
              <button
                onClick={() => onDelete(task.id)}
                className="text-gray-300 hover:text-red-400 transition-colors"
              >
                <svg width="14" height="14" viewBox="0 0 14 14" fill="none">
                  <path d="M3 3L11 11M11 3L3 11" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" />
                </svg>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
