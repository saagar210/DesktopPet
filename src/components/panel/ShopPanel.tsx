import { invoke } from "@tauri-apps/api/core";
import { SHOP_ITEMS } from "../../lib/constants";
import type { ShopItemId } from "../../lib/constants";
import { CoinBadge } from "../shared/CoinBadge";

interface Props {
  available: number;
  ownedAccessories: ShopItemId[];
}

export function ShopPanel({ available, ownedAccessories }: Props) {
  const handleBuy = async (itemId: string) => {
    try {
      await invoke("purchase_item", { itemId });
    } catch (e) {
      console.error("Purchase failed:", e);
    }
  };

  return (
    <div className="flex flex-col gap-3">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium text-gray-600">Your balance:</span>
        <CoinBadge amount={available} />
      </div>
      <div className="grid grid-cols-2 gap-2">
        {SHOP_ITEMS.map((item) => {
          const owned = ownedAccessories.includes(item.id);
          const canAfford = available >= item.cost;
          return (
            <div
              key={item.id}
              className={`p-3 rounded-lg border text-center ${
                owned ? "bg-green-50 border-green-200" : "bg-white border-gray-100"
              }`}
            >
              <div className="text-2xl mb-1">{item.icon}</div>
              <div className="text-sm font-medium text-gray-700">{item.name}</div>
              <div className="text-xs text-gray-400 mb-2">
                <CoinBadge amount={item.cost} />
              </div>
              {owned ? (
                <span className="text-xs text-green-600 font-medium">Owned</span>
              ) : (
                <button
                  onClick={() => handleBuy(item.id)}
                  disabled={!canAfford}
                  className={`px-3 py-1 text-xs rounded-full font-medium transition-colors ${
                    canAfford
                      ? "bg-amber-500 text-white hover:bg-amber-600"
                      : "bg-gray-200 text-gray-400 cursor-not-allowed"
                  }`}
                >
                  Buy
                </button>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
