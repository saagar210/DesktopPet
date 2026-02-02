import { formatTime } from "../../lib/utils";

interface Props {
  phase: string;
  secondsLeft: number;
  totalSeconds: number;
  sessionsCompleted: number;
  paused: boolean;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onReset: () => void;
}

export function TimerDisplay({
  phase,
  secondsLeft,
  totalSeconds,
  sessionsCompleted,
  paused,
  onStart,
  onPause,
  onResume,
  onReset,
}: Props) {
  const radius = 70;
  const circumference = 2 * Math.PI * radius;
  const progress = totalSeconds > 0 ? secondsLeft / totalSeconds : 0;
  const dashOffset = circumference * (1 - progress);

  const ringColor =
    phase === "work"
      ? "stroke-red-400"
      : phase === "break"
        ? "stroke-green-400"
        : phase === "celebrating"
          ? "stroke-amber-400"
          : "stroke-gray-300";

  const phaseLabel =
    phase === "work"
      ? "Focus"
      : phase === "break"
        ? "Break"
        : phase === "celebrating"
          ? "Nice!"
          : "Ready";

  return (
    <div className="flex flex-col items-center gap-4">
      {/* Circular timer */}
      <div className="relative">
        <svg width="180" height="180" viewBox="0 0 180 180">
          {/* Background ring */}
          <circle
            cx="90"
            cy="90"
            r={radius}
            fill="none"
            stroke="#E5E7EB"
            strokeWidth="8"
          />
          {/* Progress ring */}
          <circle
            cx="90"
            cy="90"
            r={radius}
            fill="none"
            className={ringColor}
            strokeWidth="8"
            strokeLinecap="round"
            strokeDasharray={circumference}
            strokeDashoffset={dashOffset}
            transform="rotate(-90 90 90)"
            style={{ transition: "stroke-dashoffset 0.5s ease" }}
          />
        </svg>
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <span className="text-3xl font-mono font-bold text-gray-800">
            {formatTime(secondsLeft)}
          </span>
          <span className="text-sm text-gray-500">{phaseLabel}</span>
        </div>
      </div>

      {/* Controls */}
      <div className="flex gap-2">
        {phase === "idle" && (
          <button
            onClick={onStart}
            className="px-6 py-2 bg-red-500 text-white rounded-full font-medium hover:bg-red-600 transition-colors"
          >
            Start
          </button>
        )}
        {(phase === "work" || phase === "break") && !paused && (
          <button
            onClick={onPause}
            className="px-6 py-2 bg-gray-500 text-white rounded-full font-medium hover:bg-gray-600 transition-colors"
          >
            Pause
          </button>
        )}
        {(phase === "work" || phase === "break") && paused && (
          <button
            onClick={onResume}
            className="px-6 py-2 bg-green-500 text-white rounded-full font-medium hover:bg-green-600 transition-colors"
          >
            Resume
          </button>
        )}
        {phase !== "idle" && (
          <button
            onClick={onReset}
            className="px-6 py-2 bg-gray-200 text-gray-700 rounded-full font-medium hover:bg-gray-300 transition-colors"
          >
            Reset
          </button>
        )}
      </div>

      {/* Session counter */}
      <div className="text-sm text-gray-500">
        Sessions today: <span className="font-semibold text-gray-700">{sessionsCompleted}</span>
      </div>
    </div>
  );
}
