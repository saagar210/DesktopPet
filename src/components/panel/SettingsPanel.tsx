import { TIMER_PRESETS } from "../../lib/constants";
import type { TimerPreset } from "../../lib/constants";

interface Props {
  preset: TimerPreset;
  onSetPreset: (p: TimerPreset) => void;
  disabled: boolean;
}

export function SettingsPanel({ preset, onSetPreset, disabled }: Props) {
  const presets = Object.entries(TIMER_PRESETS) as [TimerPreset, (typeof TIMER_PRESETS)[TimerPreset]][];

  return (
    <div className="flex flex-col gap-4">
      <h3 className="text-sm font-medium text-gray-600">Timer Mode</h3>
      <div className="flex flex-col gap-2">
        {presets.map(([key, val]) => (
          <button
            key={key}
            onClick={() => onSetPreset(key)}
            disabled={disabled}
            className={`p-3 rounded-lg border text-left transition-colors ${
              preset === key
                ? "bg-blue-50 border-blue-300"
                : disabled
                  ? "bg-gray-50 border-gray-100 text-gray-400"
                  : "bg-white border-gray-100 hover:border-blue-200"
            }`}
          >
            <div className="text-sm font-medium text-gray-700">{val.label}</div>
            <div className="text-xs text-gray-400">
              {val.work / 60}min work / {val.break / 60}min break
            </div>
          </button>
        ))}
      </div>
      {disabled && (
        <p className="text-xs text-gray-400">
          Timer mode can only be changed when the timer is idle.
        </p>
      )}
    </div>
  );
}
