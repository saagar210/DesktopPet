import type { PetSpeciesPack } from "./species";

export interface PackValidationCheck {
  id: string;
  label: string;
  pass: boolean;
  detail: string;
}

export interface PackValidationResult {
  speciesId: string;
  pass: boolean;
  checks: PackValidationCheck[];
}

function isSlug(value: string) {
  return /^[a-z0-9_-]+$/.test(value);
}

function anchorInBounds(anchor: { x: number; y: number }) {
  return anchor.x >= 0 && anchor.x <= 200 && anchor.y >= 0 && anchor.y <= 200;
}

export function validateSpeciesPack(pack: PetSpeciesPack): PackValidationResult {
  const checks: PackValidationCheck[] = [];

  checks.push({
    id: "id-slug",
    label: "Valid species id slug",
    pass: isSlug(pack.id),
    detail: pack.id,
  });

  checks.push({
    id: "stage-names",
    label: "Exactly 3 stage names",
    pass: pack.stageNames.length === 3,
    detail: `${pack.stageNames.length} names`,
  });

  const thresholds = pack.evolutionThresholds;
  const thresholdsValid =
    thresholds.length === 3 &&
    thresholds[0] === 0 &&
    thresholds[1] >= 1 &&
    thresholds[2] > thresholds[1];
  checks.push({
    id: "thresholds",
    label: "Valid ascending evolution thresholds",
    pass: thresholdsValid,
    detail: thresholds.join(" / "),
  });

  const spriteChecks = pack.stageSprites.every(
    (sprite) => typeof sprite === "string" && sprite.trim().length > 0
  );
  checks.push({
    id: "sprites",
    label: "3 stage sprites resolved with supported format",
    pass: spriteChecks && pack.stageSprites.length === 3,
    detail: `${pack.stageSprites.length} sprite assets`,
  });

  checks.push({
    id: "blink-window",
    label: "Idle blink range is sane",
    pass:
      pack.idleBehavior.blinkIntervalMs[0] >= 1200 &&
      pack.idleBehavior.blinkIntervalMs[1] > pack.idleBehavior.blinkIntervalMs[0],
    detail: `${pack.idleBehavior.blinkIntervalMs[0]}-${pack.idleBehavior.blinkIntervalMs[1]}ms`,
  });

  const anchors = pack.accessoryAnchors;
  checks.push({
    id: "anchors",
    label: "Accessory anchors are within sprite bounds",
    pass:
      anchorInBounds(anchors.head) &&
      anchorInBounds(anchors.neck) &&
      anchorInBounds(anchors.left) &&
      anchorInBounds(anchors.right),
    detail: "head/neck/left/right",
  });

  const verbs = new Set(pack.interactionVerbs.map((verb) => verb.id));
  checks.push({
    id: "verbs",
    label: "All core interaction verbs are present",
    pass:
      verbs.has("pet") &&
      verbs.has("feed") &&
      verbs.has("play") &&
      verbs.has("nap") &&
      verbs.has("clean") &&
      verbs.has("train"),
    detail: `${verbs.size} verbs`,
  });

  return {
    speciesId: pack.id,
    pass: checks.every((check) => check.pass),
    checks,
  };
}

export function validateSpeciesPacks(packs: PetSpeciesPack[]) {
  return packs.map(validateSpeciesPack);
}
