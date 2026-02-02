import { CoinBadge } from "../shared/CoinBadge";
import { ProgressBar } from "../shared/ProgressBar";

interface Props {
  available: number;
  stageName: string;
  progressToNext: number;
  stageProgress: number;
  stageSpan: number;
}

export function CoinDisplay({
  available,
  stageName,
  progressToNext,
  stageProgress,
  stageSpan,
}: Props) {
  return (
    <div className="flex flex-col gap-2 p-3 bg-amber-50 rounded-lg">
      <div className="flex items-center justify-between">
        <CoinBadge amount={available} />
        <span className="text-sm font-medium text-gray-600">
          Stage: <span className="text-blue-600">{stageName}</span>
        </span>
      </div>
      {stageSpan > 0 ? (
        <div>
          <div className="flex justify-between text-xs text-gray-500 mb-1">
            <span>Evolution progress</span>
            <span>
              {stageProgress}/{stageSpan}
            </span>
          </div>
          <ProgressBar value={progressToNext} color="bg-blue-500" />
        </div>
      ) : (
        <div className="text-xs text-gray-500 text-center">Max evolution reached!</div>
      )}
    </div>
  );
}
