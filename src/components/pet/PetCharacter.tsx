import type { ShopItemId } from "../../lib/constants";
import { PetStage1 } from "./PetStage1";
import { PetStage2 } from "./PetStage2";
import { PetStage3 } from "./PetStage3";

interface Props {
  stage: number;
  accessories: ShopItemId[];
}

export function PetCharacter({ stage, accessories }: Props) {
  switch (stage) {
    case 2:
      return <PetStage3 accessories={accessories} />;
    case 1:
      return <PetStage2 accessories={accessories} />;
    default:
      return <PetStage1 accessories={accessories} />;
  }
}
