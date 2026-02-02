import type { DailyGoal } from "../../store/types";
import { ProgressBar } from "../shared/ProgressBar";

interface Props {
  goals: DailyGoal[];
}

export function GoalsList({ goals }: Props) {
  if (goals.length === 0) {
    return <p className="text-sm text-gray-400 text-center py-4">Loading goals...</p>;
  }

  return (
    <div className="flex flex-col gap-3">
      {goals.map((goal) => {
        const done = goal.progress >= goal.target;
        return (
          <div key={goal.id} className="p-3 bg-white rounded-lg border border-gray-100">
            <div className="flex items-center justify-between mb-1">
              <span className={`text-sm ${done ? "text-green-600 line-through" : "text-gray-700"}`}>
                {done ? "\u2713 " : ""}
                {goal.description}
              </span>
              <span className="text-xs text-gray-400">
                {goal.progress}/{goal.target}
              </span>
            </div>
            <ProgressBar
              value={goal.progress}
              max={goal.target}
              color={done ? "bg-green-500" : "bg-blue-400"}
            />
          </div>
        );
      })}
    </div>
  );
}
