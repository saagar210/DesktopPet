import { useState, type CSSProperties } from "react";
import { usePomodoro } from "../../hooks/usePomodoro";
import { usePet } from "../../hooks/usePet";
import { useCoins } from "../../hooks/useCoins";
import { useGoals } from "../../hooks/useGoals";
import { useTasks } from "../../hooks/useTasks";
import { useSettings } from "../../hooks/useSettings";
import { useCustomization } from "../../hooks/useCustomization";
import { useFocusGuardrails } from "../../hooks/useFocusGuardrails";
import { useProgress } from "../../hooks/useProgress";
import { useAnalytics } from "../../hooks/useAnalytics";
import { usePetEvents } from "../../hooks/usePetEvents";
import { getThemeTokens } from "../../lib/themes";
import { TimerDisplay } from "./TimerDisplay";
import { CoinDisplay } from "./CoinDisplay";
import { GoalsList } from "./GoalsList";
import { TaskList } from "./TaskList";
import { ShopPanel } from "./ShopPanel";
import { SettingsPanel } from "./SettingsPanel";
import { StatsPanel } from "./StatsPanel";
import { CustomizationPanel } from "./CustomizationPanel";
import { PetPanel } from "./PetPanel";

type Tab = "timer" | "pet" | "goals" | "tasks" | "shop" | "stats" | "customize" | "settings";

const TABS: { id: Tab; label: string }[] = [
  { id: "timer", label: "Timer" },
  { id: "pet", label: "Pet" },
  { id: "goals", label: "Goals" },
  { id: "tasks", label: "Tasks" },
  { id: "shop", label: "Shop" },
  { id: "stats", label: "Stats" },
  { id: "customize", label: "Customize" },
  { id: "settings", label: "Settings" },
];

export function ControlPanel() {
  const [tab, setTab] = useState<Tab>("timer");
  const pomo = usePomodoro();
  const { pet, stageName, progressToNext, stageProgress, stageSpan, setCustomization, interact } = usePet();
  const { available } = useCoins();
  const { goals } = useGoals();
  const { tasks, addTask, toggleTask, deleteTask } = useTasks();
  const { settings, updateSettings } = useSettings();
  const { loadouts, saveLoadout, applyLoadout } = useCustomization();
  const { status: guardrailStatus, events: guardrailEvents, evaluate, intervene } = useFocusGuardrails();
  const { progress } = useProgress();
  const { summaries } = useAnalytics();
  const { events: petEvents, activeQuest, rollEvent, resolveEvent } = usePetEvents();
  const theme = getThemeTokens(settings.uiTheme);

  return (
    <div
      className="h-screen flex flex-col"
      style={
        {
          backgroundColor: theme.appBg,
          "--app-bg": theme.appBg,
          "--panel-bg": theme.panelBg,
          "--card-bg": theme.cardBg,
          "--border-color": theme.border,
          "--accent-color": theme.accent,
          "--accent-soft": theme.accentSoft,
          "--text-color": theme.text,
          "--muted-color": theme.muted,
          "--tab-inactive": theme.tabInactive,
        } as CSSProperties
      }
    >
      {/* Header */}
      <div className="p-4 pb-2">
        <CoinDisplay
          available={available}
          stageName={stageName}
          progressToNext={progressToNext}
          stageProgress={stageProgress}
          stageSpan={stageSpan}
          level={progress.level}
          streakDays={progress.streakDays}
        />
      </div>

      {/* Tabs */}
      <div className="flex px-2" style={{ borderBottom: "1px solid var(--border-color)" }}>
        {TABS.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className="flex-1 py-2 text-xs font-medium transition-colors border-b-2"
            style={{
              color: tab === t.id ? "var(--accent-color)" : "var(--tab-inactive)",
              borderBottomColor: tab === t.id ? "var(--accent-color)" : "transparent",
            }}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4" style={{ color: "var(--text-color)" }}>
        {tab === "timer" && (
          <TimerDisplay
            phase={pomo.phase}
            secondsLeft={pomo.secondsLeft}
            totalSeconds={pomo.totalSeconds}
            sessionsCompleted={pomo.sessionsCompleted}
            paused={pomo.paused}
            guardrailMessage={pomo.guardrailMessage}
            onStart={pomo.start}
            onPause={pomo.pause}
            onResume={pomo.resume}
            onReset={pomo.reset}
          />
        )}
        {tab === "pet" && (
          <PetPanel
            pet={pet}
            events={petEvents}
            activeQuest={activeQuest}
            onInteract={interact}
            onRollEvent={rollEvent}
            onResolveEvent={resolveEvent}
          />
        )}
        {tab === "goals" && <GoalsList goals={goals} />}
        {tab === "tasks" && (
          <TaskList
            tasks={tasks}
            onAdd={addTask}
            onToggle={toggleTask}
            onDelete={deleteTask}
          />
        )}
        {tab === "shop" && (
          <ShopPanel
            available={available}
            ownedAccessories={pet.accessories}
          />
        )}
        {tab === "stats" && (
          <StatsPanel progress={progress} summaries={summaries} />
        )}
        {tab === "customize" && (
          <CustomizationPanel
            settings={settings}
            pet={pet}
            loadouts={loadouts}
            onUpdateSettings={(patch) => {
              void updateSettings(patch);
            }}
            onSetPetCustomization={(skin, scene) => {
              void setCustomization(skin, scene);
            }}
            onSaveLoadout={(loadout) => {
              void saveLoadout(loadout);
            }}
            onApplyLoadout={(name) => {
              void applyLoadout(name);
            }}
          />
        )}
        {tab === "settings" && (
          <SettingsPanel
            preset={pomo.preset}
            settings={settings}
            onSetPreset={pomo.setPreset}
            onSetNotificationsEnabled={(enabled) =>
              void updateSettings({ notificationsEnabled: enabled })
            }
            onSetSoundsEnabled={(enabled) =>
              void updateSettings({ soundsEnabled: enabled })
            }
            onSetSoundVolume={(volume) =>
              void updateSettings({ soundVolume: volume })
            }
            onSetFocusGuardrailsEnabled={(enabled) =>
              void updateSettings({ focusGuardrailsEnabled: enabled })
            }
            onSetFocusGuardrailsWorkOnly={(enabled) =>
              void updateSettings({ focusGuardrailsWorkOnly: enabled })
            }
            onSetFocusAllowlist={(hosts) =>
              void updateSettings({ focusAllowlist: hosts })
            }
            onSetFocusBlocklist={(hosts) =>
              void updateSettings({ focusBlocklist: hosts })
            }
            onEvaluateGuardrails={(phase, hosts) => {
              void evaluate(phase, hosts);
            }}
            onInterveneGuardrails={(phase, hosts) => {
              void intervene(phase, hosts);
            }}
            guardrailStatus={guardrailStatus}
            guardrailEvents={guardrailEvents}
            disabled={pomo.phase !== "idle"}
          />
        )}
      </div>
    </div>
  );
}
