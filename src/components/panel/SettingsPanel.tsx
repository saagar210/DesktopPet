import { useEffect, useState } from "react";
import { TIMER_PRESETS } from "../../lib/constants";
import type { TimerPreset } from "../../lib/constants";
import type { FocusGuardrailEvent, FocusGuardrailsStatus, Settings } from "../../store/types";

interface Props {
  preset: TimerPreset;
  settings: Settings;
  onSetPreset: (p: TimerPreset) => void;
  onSetNotificationsEnabled: (enabled: boolean) => void;
  onSetSoundsEnabled: (enabled: boolean) => void;
  onSetSoundVolume: (volume: number) => void;
  onSetFocusGuardrailsEnabled: (enabled: boolean) => void;
  onSetFocusGuardrailsWorkOnly: (enabled: boolean) => void;
  onSetFocusAllowlist: (hosts: string[]) => void;
  onSetFocusBlocklist: (hosts: string[]) => void;
  onEvaluateGuardrails: (phase: string, hosts: string[]) => void;
  onInterveneGuardrails: (phase: string, hosts: string[]) => void;
  guardrailStatus: FocusGuardrailsStatus | null;
  guardrailEvents: FocusGuardrailEvent[];
  disabled: boolean;
}

export function SettingsPanel({
  preset,
  settings,
  onSetPreset,
  onSetNotificationsEnabled,
  onSetSoundsEnabled,
  onSetSoundVolume,
  onSetFocusGuardrailsEnabled,
  onSetFocusGuardrailsWorkOnly,
  onSetFocusAllowlist,
  onSetFocusBlocklist,
  onEvaluateGuardrails,
  onInterveneGuardrails,
  guardrailStatus,
  guardrailEvents,
  disabled,
}: Props) {
  const presets = Object.entries(TIMER_PRESETS) as [TimerPreset, (typeof TIMER_PRESETS)[TimerPreset]][];
  const [hostPreview, setHostPreview] = useState(settings.focusBlocklist.join(", "));

  useEffect(() => {
    setHostPreview(settings.focusBlocklist.join(", "));
  }, [settings.focusBlocklist]);

  return (
    <div className="flex flex-col gap-4">
      <h3 className="text-sm font-medium" style={{ color: "var(--muted-color)" }}>Timer Mode</h3>
      <div className="flex flex-col gap-2">
        {presets.map(([key, val]) => (
          <button
            key={key}
            onClick={() => onSetPreset(key)}
            disabled={disabled}
            className={`p-3 rounded-lg border text-left transition-opacity ${disabled ? "opacity-60" : ""}`}
            style={{
              backgroundColor:
                preset === key
                  ? "var(--accent-soft)"
                  : "var(--card-bg)",
              borderColor:
                preset === key
                  ? "color-mix(in srgb, var(--accent-color) 35%, white)"
                  : "var(--border-color)",
              color: disabled ? "var(--muted-color)" : "var(--text-color)",
            }}
          >
            <div className="text-sm font-medium">{val.label}</div>
            <div className="text-xs" style={{ color: "var(--muted-color)" }}>
              {val.work / 60}min work / {val.break / 60}min break
            </div>
          </button>
        ))}
      </div>
      {disabled && (
        <p className="text-xs" style={{ color: "var(--muted-color)" }}>
          Timer mode can only be changed when the timer is idle.
        </p>
      )}

      <div className="pt-2 border-t flex flex-col gap-3" style={{ borderColor: "var(--border-color)" }}>
        <h3 className="text-sm font-medium" style={{ color: "var(--muted-color)" }}>Alerts</h3>
        <label className="flex items-center justify-between gap-3 text-sm">
          <span style={{ color: "var(--text-color)" }}>Desktop notifications</span>
          <input
            type="checkbox"
            checked={settings.notificationsEnabled}
            onChange={(event) => onSetNotificationsEnabled(event.target.checked)}
          />
        </label>
        <label className="flex items-center justify-between gap-3 text-sm">
          <span style={{ color: "var(--text-color)" }}>Sound cues</span>
          <input
            type="checkbox"
            checked={settings.soundsEnabled}
            onChange={(event) => onSetSoundsEnabled(event.target.checked)}
          />
        </label>
        <label className="flex flex-col gap-1 text-sm">
          <span className={!settings.soundsEnabled ? "opacity-50" : ""} style={{ color: "var(--text-color)" }}>
            Sound volume ({Math.round(settings.soundVolume * 100)}%)
          </span>
          <input
            type="range"
            min={0}
            max={1}
            step={0.05}
            value={settings.soundVolume}
            onChange={(event) => onSetSoundVolume(Number(event.target.value))}
            disabled={!settings.soundsEnabled}
          />
        </label>
      </div>

      <div className="pt-2 border-t flex flex-col gap-3" style={{ borderColor: "var(--border-color)" }}>
        <h3 className="text-sm font-medium" style={{ color: "var(--muted-color)" }}>Focus Guardrails</h3>
        <label className="flex items-center justify-between gap-3 text-sm">
          <span style={{ color: "var(--text-color)" }}>Enable guardrails</span>
          <input
            type="checkbox"
            checked={settings.focusGuardrailsEnabled}
            onChange={(event) => onSetFocusGuardrailsEnabled(event.target.checked)}
          />
        </label>
        <label className="flex items-center justify-between gap-3 text-sm">
          <span style={{ color: "var(--text-color)" }}>Only during work sessions</span>
          <input
            type="checkbox"
            checked={settings.focusGuardrailsWorkOnly}
            onChange={(event) => onSetFocusGuardrailsWorkOnly(event.target.checked)}
            disabled={!settings.focusGuardrailsEnabled}
          />
        </label>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Allowed hosts (comma-separated)
          <textarea
            rows={2}
            className="px-2 py-1 border rounded-md text-sm"
            style={{ borderColor: "var(--border-color)", backgroundColor: "var(--card-bg)", color: "var(--text-color)" }}
            value={settings.focusAllowlist.join(", ")}
            onChange={(event) =>
              onSetFocusAllowlist(
                event.target.value
                  .split(",")
                  .map((item) => item.trim())
                  .filter(Boolean)
              )
            }
            disabled={!settings.focusGuardrailsEnabled}
          />
        </label>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Blocked hosts (comma-separated)
          <textarea
            rows={2}
            className="px-2 py-1 border rounded-md text-sm"
            style={{ borderColor: "var(--border-color)", backgroundColor: "var(--card-bg)", color: "var(--text-color)" }}
            value={settings.focusBlocklist.join(", ")}
            onChange={(event) =>
              onSetFocusBlocklist(
                event.target.value
                  .split(",")
                  .map((item) => item.trim())
                  .filter(Boolean)
              )
            }
            disabled={!settings.focusGuardrailsEnabled}
          />
        </label>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Host check preview (comma-separated)
          <input
            className="px-2 py-1 border rounded-md text-sm"
            style={{ borderColor: "var(--border-color)", backgroundColor: "var(--card-bg)", color: "var(--text-color)" }}
            value={hostPreview}
            onChange={(event) => setHostPreview(event.target.value)}
            disabled={!settings.focusGuardrailsEnabled}
          />
        </label>
        <div className="flex gap-2">
          <button
            className="px-2 py-1 rounded-md text-xs text-white disabled:opacity-40"
            style={{ backgroundColor: "var(--accent-color)" }}
            disabled={!settings.focusGuardrailsEnabled}
            onClick={() =>
              onEvaluateGuardrails(
                "work",
                hostPreview
                  .split(",")
                  .map((item) => item.trim())
                  .filter(Boolean)
              )
            }
          >
            Evaluate
          </button>
          <button
            className="px-2 py-1 rounded-md text-xs text-white disabled:opacity-40"
            style={{ backgroundColor: "color-mix(in srgb, var(--accent-color) 60%, #f59e0b)" }}
            disabled={!settings.focusGuardrailsEnabled}
            onClick={() =>
              onInterveneGuardrails(
                "work",
                hostPreview
                  .split(",")
                  .map((item) => item.trim())
                  .filter(Boolean)
              )
            }
          >
            Intervene
          </button>
        </div>
        {guardrailStatus && (
          <div
            className="text-xs border rounded-md px-2 py-1"
            style={{
              color: "var(--text-color)",
              backgroundColor: "color-mix(in srgb, var(--accent-soft) 20%, var(--card-bg))",
              borderColor: "var(--border-color)",
            }}
          >
            <div>{guardrailStatus.message}</div>
            <div>
              level: {guardrailStatus.nudgeLevel} • action: {guardrailStatus.recommendedAction}
            </div>
          </div>
        )}
        {guardrailEvents.length > 0 && (
          <div className="flex flex-col gap-1">
            <div className="text-xs font-medium" style={{ color: "var(--muted-color)" }}>
              Recent interventions
            </div>
            {guardrailEvents.slice(0, 5).map((event) => (
              <div
                key={event.id}
                className="text-[11px] px-2 py-1 border rounded-md"
                style={{
                  color: "var(--text-color)",
                  borderColor: "var(--border-color)",
                  backgroundColor: "var(--card-bg)",
                }}
              >
                <div className="capitalize">
                  {event.phase} • {event.nudgeLevel} • {event.recommendedAction.replace("_", " ")}
                </div>
                <div style={{ color: "var(--muted-color)" }}>
                  {new Date(event.createdAt).toLocaleTimeString()} •{" "}
                  {event.matchedBlocklist.join(", ") || "no host details"}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
