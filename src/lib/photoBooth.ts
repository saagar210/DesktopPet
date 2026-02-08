import type { PetState, Settings, UserProgress } from "../store/types";
import type { PetSpeciesPack } from "../pets/species";

interface PetCardInput {
  pet: PetState;
  species: PetSpeciesPack;
  stageName: string;
  coinsAvailable: number;
  progress: UserProgress;
  settings: Settings;
}

function roundedRect(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  width: number,
  height: number,
  radius: number
) {
  ctx.beginPath();
  ctx.moveTo(x + radius, y);
  ctx.lineTo(x + width - radius, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + radius);
  ctx.lineTo(x + width, y + height - radius);
  ctx.quadraticCurveTo(x + width, y + height, x + width - radius, y + height);
  ctx.lineTo(x + radius, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - radius);
  ctx.lineTo(x, y + radius);
  ctx.quadraticCurveTo(x, y, x + radius, y);
  ctx.closePath();
}

async function loadImage(url: string) {
  return new Promise<HTMLImageElement>((resolve, reject) => {
    const image = new Image();
    image.onload = () => resolve(image);
    image.onerror = () => reject(new Error("Unable to load pet sprite"));
    image.src = url;
  });
}

export async function downloadPetCard(input: PetCardInput) {
  const canvas = document.createElement("canvas");
  canvas.width = 960;
  canvas.height = 540;
  const ctx = canvas.getContext("2d");
  if (!ctx) {
    throw new Error("Canvas context unavailable");
  }

  const bg = ctx.createLinearGradient(0, 0, 960, 540);
  bg.addColorStop(0, "#fef9f2");
  bg.addColorStop(1, "#e0f2fe");
  ctx.fillStyle = bg;
  ctx.fillRect(0, 0, 960, 540);

  roundedRect(ctx, 44, 44, 872, 452, 28);
  ctx.fillStyle = "rgba(255, 255, 255, 0.86)";
  ctx.fill();

  ctx.fillStyle = "#1f2937";
  ctx.font = "700 38px 'Avenir Next', 'Segoe UI', sans-serif";
  ctx.fillText(`${input.species.name} Pet Card`, 82, 110);
  ctx.font = "500 22px 'Avenir Next', 'Segoe UI', sans-serif";
  ctx.fillStyle = "#475569";
  ctx.fillText(`Stage: ${input.stageName}   Mood: ${input.pet.mood}`, 82, 148);

  const sprite = await loadImage(
    input.species.stageSprites[Math.max(0, Math.min(2, input.pet.currentStage))]
  );
  ctx.drawImage(sprite, 88, 176, 250, 250);

  ctx.fillStyle = "#0f172a";
  ctx.font = "600 24px 'Avenir Next', 'Segoe UI', sans-serif";
  ctx.fillText(`Coins: ${input.coinsAvailable}`, 390, 216);
  ctx.fillText(`Level: ${input.progress.level}`, 390, 258);
  ctx.fillText(`Streak: ${input.progress.streakDays} days`, 390, 300);
  ctx.fillText(`Total Sessions: ${input.pet.totalPomodoros}`, 390, 342);
  ctx.fillText(`Theme: ${input.settings.uiTheme}`, 390, 384);
  ctx.fillText(`Loadout: ${input.settings.petSkin} / ${input.settings.petScene}`, 390, 426);

  ctx.font = "500 16px 'Avenir Next', 'Segoe UI', sans-serif";
  ctx.fillStyle = "#64748b";
  ctx.fillText("Generated in DesktopPet Photo Booth", 82, 478);
  ctx.fillText(new Date().toLocaleString(), 690, 478);

  const blob = await new Promise<Blob | null>((resolve) =>
    canvas.toBlob((result) => resolve(result), "image/png")
  );

  if (!blob) {
    throw new Error("Unable to render pet card");
  }

  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = `desktop-pet-card-${input.species.id}-${new Date()
    .toISOString()
    .slice(0, 10)}.png`;
  anchor.click();
  URL.revokeObjectURL(url);
}
