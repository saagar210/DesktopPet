import { useState } from "react";
import { usePomodoro } from "../../hooks/usePomodoro";
import { usePet } from "../../hooks/usePet";
import { useCoins } from "../../hooks/useCoins";
import { useGoals } from "../../hooks/useGoals";
import { useTasks } from "../../hooks/useTasks";
import { TimerDisplay } from "./TimerDisplay";
import { CoinDisplay } from "./CoinDisplay";
import { GoalsList } from "./GoalsList";
import { TaskList } from "./TaskList";
import { ShopPanel } from "./ShopPanel";
import { SettingsPanel } from "./SettingsPanel";

type Tab = "timer" | "goals" | "tasks" | "shop" | "settings";

const TABS: { id: Tab; label: string }[] = [
  { id: "timer", label: "Timer" },
  { id: "goals", label: "Goals" },
  { id: "tasks", label: "Tasks" },
  { id: "shop", label: "Shop" },
  { id: "settings", label: "Settings" },
];

export function ControlPanel() {
  const [tab, setTab] = useState<Tab>("timer");
  const pomo = usePomodoro();
  const { pet, stageName, progressToNext, stageProgress, stageSpan } = usePet();
  const { available } = useCoins();
  const { goals } = useGoals();
  const { tasks, addTask, toggleTask, deleteTask } = useTasks();

  return (
    <div className="h-screen flex flex-col bg-gray-50">
      {/* Header */}
      <div className="p-4 pb-2">
        <CoinDisplay
          available={available}
          stageName={stageName}
          progressToNext={progressToNext}
          stageProgress={stageProgress}
          stageSpan={stageSpan}
        />
      </div>

      {/* Tabs */}
      <div className="flex border-b border-gray-200 px-2">
        {TABS.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className={`flex-1 py-2 text-xs font-medium transition-colors ${
              tab === t.id
                ? "text-blue-600 border-b-2 border-blue-600"
                : "text-gray-400 hover:text-gray-600"
            }`}
          >
            {t.label}
          </button>
        ))}
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-4">
        {tab === "timer" && (
          <TimerDisplay
            phase={pomo.phase}
            secondsLeft={pomo.secondsLeft}
            totalSeconds={pomo.totalSeconds}
            sessionsCompleted={pomo.sessionsCompleted}
            paused={pomo.paused}
            onStart={pomo.start}
            onPause={pomo.pause}
            onResume={pomo.resume}
            onReset={pomo.reset}
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
        {tab === "settings" && (
          <SettingsPanel
            preset={pomo.preset}
            onSetPreset={pomo.setPreset}
            disabled={pomo.phase !== "idle"}
          />
        )}
      </div>
    </div>
  );
}
