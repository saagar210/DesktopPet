import { useState } from "react";
import { getSpeciesPackById, getSpeciesPacks } from "../../pets/species";
import { getSeasonalPacks } from "../../pets/seasonalPacks";
import type { CustomizationLoadout, PetState, Settings, SettingsPatch } from "../../store/types";

interface Props {
  settings: Settings;
  pet: PetState;
  loadouts: CustomizationLoadout[];
  onUpdateSettings: (patch: SettingsPatch) => void;
  onSetPetCustomization: (skin?: string, scene?: string) => void;
  onSetPetSpecies: (speciesId: string, evolutionThresholds: number[]) => void;
  onSaveLoadout: (loadout: CustomizationLoadout) => void;
  onApplyLoadout: (name: string) => void;
}

const THEMES = ["sunrise", "dusk", "mint", "mono"] as const;
const SKINS = ["classic", "neon", "pixel", "plush"] as const;
const SCENES = ["meadow", "forest", "space", "cozy_room"] as const;
const STYLE_BUNDLES = [
  { name: "Focus Studio", uiTheme: "mono", petSkin: "pixel", petScene: "cozy_room" },
  { name: "Cozy Flow", uiTheme: "sunrise", petSkin: "plush", petScene: "meadow" },
  { name: "Deep Work Orbit", uiTheme: "dusk", petSkin: "neon", petScene: "space" },
] as const;

export function CustomizationPanel({
  settings,
  pet,
  loadouts,
  onUpdateSettings,
  onSetPetCustomization,
  onSetPetSpecies,
  onSaveLoadout,
  onApplyLoadout,
}: Props) {
  const [loadoutName, setLoadoutName] = useState("");
  const speciesPacks = getSpeciesPacks();
  const seasonalPacks = getSeasonalPacks();

  return (
    <div className="flex flex-col gap-4">
      <div
        className="p-3 rounded-lg border flex flex-col gap-3"
        style={{ backgroundColor: "var(--card-bg)", borderColor: "var(--border-color)" }}
      >
        <h3 className="text-sm font-medium" style={{ color: "var(--text-color)" }}>Theme</h3>
        <div className="grid grid-cols-2 gap-2">
          {THEMES.map((theme) => (
            <button
              key={theme}
              onClick={() => onUpdateSettings({ uiTheme: theme })}
              className="px-2 py-2 rounded-md border text-xs capitalize"
              style={{
                backgroundColor:
                  settings.uiTheme === theme ? "var(--accent-soft)" : "var(--card-bg)",
                borderColor:
                  settings.uiTheme === theme
                    ? "color-mix(in srgb, var(--accent-color) 35%, white)"
                    : "var(--border-color)",
                color:
                  settings.uiTheme === theme
                    ? "var(--accent-color)"
                    : "var(--muted-color)",
              }}
            >
              {theme.replace("_", " ")}
            </button>
          ))}
        </div>
      </div>

      <div
        className="p-3 rounded-lg border flex flex-col gap-3"
        style={{ backgroundColor: "var(--card-bg)", borderColor: "var(--border-color)" }}
      >
        <h3 className="text-sm font-medium" style={{ color: "var(--text-color)" }}>Pet Style</h3>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Species
          <select
            value={pet.speciesId}
            onChange={(event) => {
              const selected = speciesPacks.find((pack) => pack.id === event.target.value);
              if (!selected) {
                return;
              }
              onSetPetSpecies(selected.id, selected.evolutionThresholds);
            }}
            className="px-2 py-1 border rounded-md text-sm"
            style={{
              borderColor: "var(--border-color)",
              backgroundColor: "var(--card-bg)",
              color: "var(--text-color)",
            }}
          >
            {speciesPacks.map((pack) => (
              <option key={pack.id} value={pack.id}>
                {pack.name}
              </option>
            ))}
          </select>
        </label>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Skin
          <select
            value={settings.petSkin}
            onChange={(event) => {
              const skin = event.target.value;
              onUpdateSettings({ petSkin: skin });
              onSetPetCustomization(skin, undefined);
            }}
            className="px-2 py-1 border rounded-md text-sm"
            style={{
              borderColor: "var(--border-color)",
              backgroundColor: "var(--card-bg)",
              color: "var(--text-color)",
            }}
          >
            {SKINS.map((skin) => (
              <option key={skin} value={skin}>
                {skin}
              </option>
            ))}
          </select>
        </label>
        <label className="text-xs flex flex-col gap-1" style={{ color: "var(--muted-color)" }}>
          Scene
          <select
            value={settings.petScene}
            onChange={(event) => {
              const scene = event.target.value;
              onUpdateSettings({ petScene: scene });
              onSetPetCustomization(undefined, scene);
            }}
            className="px-2 py-1 border rounded-md text-sm"
            style={{
              borderColor: "var(--border-color)",
              backgroundColor: "var(--card-bg)",
              color: "var(--text-color)",
            }}
          >
            {SCENES.map((scene) => (
              <option key={scene} value={scene}>
                {scene.replace("_", " ")}
              </option>
            ))}
          </select>
        </label>
      </div>

      <div
        className="p-3 rounded-lg border flex flex-col gap-2"
        style={{ backgroundColor: "var(--card-bg)", borderColor: "var(--border-color)" }}
      >
        <h3 className="text-sm font-medium" style={{ color: "var(--text-color)" }}>Style Bundles</h3>
        <div className="grid grid-cols-1 gap-2">
          {STYLE_BUNDLES.map((bundle) => (
            <button
              key={bundle.name}
              onClick={() => {
                onUpdateSettings({
                  uiTheme: bundle.uiTheme,
                  petSkin: bundle.petSkin,
                  petScene: bundle.petScene,
                });
                onSetPetCustomization(bundle.petSkin, bundle.petScene);
              }}
              className="text-left px-2 py-2 rounded-md border text-xs transition-opacity hover:opacity-90"
              style={{ borderColor: "var(--border-color)", color: "var(--text-color)" }}
            >
              <div className="font-medium">{bundle.name}</div>
              <div style={{ color: "var(--muted-color)" }}>
                {bundle.uiTheme} • {bundle.petSkin} • {bundle.petScene}
              </div>
            </button>
          ))}
        </div>
      </div>

      <div
        className="p-3 rounded-lg border flex flex-col gap-2"
        style={{ backgroundColor: "var(--card-bg)", borderColor: "var(--border-color)" }}
      >
        <h3 className="text-sm font-medium" style={{ color: "var(--text-color)" }}>
          Seasonal Cosmetic Packs (Optional)
        </h3>
        <p className="text-xs" style={{ color: "var(--muted-color)" }}>
          No timed nudges. Enable only what you want.
        </p>
        <div className="flex flex-col gap-2">
          {seasonalPacks.map((pack) => {
            const enabled = settings.enabledSeasonalPacks.includes(pack.id);
            return (
              <div key={pack.id} className="rounded-md border p-2" style={{ borderColor: "var(--border-color)" }}>
                <label className="flex items-center justify-between gap-2 text-sm">
                  <span style={{ color: "var(--text-color)" }}>{pack.name}</span>
                  <input
                    type="checkbox"
                    checked={enabled}
                    onChange={(event) => {
                      const next = event.target.checked
                        ? [...settings.enabledSeasonalPacks, pack.id]
                        : settings.enabledSeasonalPacks.filter((id) => id !== pack.id);
                      onUpdateSettings({ enabledSeasonalPacks: next });
                    }}
                  />
                </label>
                <div className="text-xs mt-1" style={{ color: "var(--muted-color)" }}>
                  {pack.description}
                </div>
                {enabled && (
                  <div className="mt-2 flex flex-col gap-1">
                    {pack.bundles.map((bundle) => (
                      <button
                        key={bundle.name}
                        onClick={() => {
                          onUpdateSettings({
                            uiTheme: bundle.uiTheme,
                            petSkin: bundle.petSkin,
                            petScene: bundle.petScene,
                          });
                          onSetPetCustomization(bundle.petSkin, bundle.petScene);
                          const species = getSpeciesPackById(bundle.speciesId);
                          onSetPetSpecies(species.id, species.evolutionThresholds);
                        }}
                        className="text-left px-2 py-2 rounded-md border text-xs transition-opacity hover:opacity-90"
                        style={{ borderColor: "var(--border-color)", color: "var(--text-color)" }}
                      >
                        <div className="font-medium">{bundle.name}</div>
                        <div style={{ color: "var(--muted-color)" }}>
                          {bundle.uiTheme} • {bundle.petSkin} • {bundle.petScene} • {bundle.speciesId}
                        </div>
                      </button>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      <div
        className="p-3 rounded-lg border flex flex-col gap-2"
        style={{ backgroundColor: "var(--card-bg)", borderColor: "var(--border-color)" }}
      >
        <h3 className="text-sm font-medium" style={{ color: "var(--text-color)" }}>Loadouts</h3>
        <div className="flex gap-2">
          <input
            value={loadoutName}
            onChange={(event) => setLoadoutName(event.target.value)}
            placeholder="Loadout name"
            className="flex-1 px-2 py-1 border rounded-md text-sm"
            style={{
              borderColor: "var(--border-color)",
              backgroundColor: "var(--card-bg)",
              color: "var(--text-color)",
            }}
          />
          <button
            onClick={() => {
              const name = loadoutName.trim();
              if (!name) return;
              onSaveLoadout({
                name,
                uiTheme: settings.uiTheme,
                petSkin: settings.petSkin,
                petScene: settings.petScene,
                accessories: pet.accessories,
              });
              setLoadoutName("");
            }}
            className="px-3 py-1 text-white rounded-md text-sm transition-opacity hover:opacity-90"
            style={{ backgroundColor: "var(--accent-color)" }}
          >
            Save
          </button>
        </div>
        {loadouts.length === 0 ? (
          <p className="text-xs" style={{ color: "var(--muted-color)" }}>No loadouts saved yet.</p>
        ) : (
          <div className="flex flex-col gap-1">
            {loadouts.map((loadout) => (
              <button
                key={loadout.name}
                onClick={() => onApplyLoadout(loadout.name)}
                className="text-left px-2 py-2 rounded-md border text-xs transition-opacity hover:opacity-90"
                style={{ borderColor: "var(--border-color)" }}
              >
                <div className="font-medium" style={{ color: "var(--text-color)" }}>{loadout.name}</div>
                <div style={{ color: "var(--muted-color)" }}>
                  {loadout.uiTheme} • {loadout.petSkin} • {loadout.petScene}
                </div>
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
