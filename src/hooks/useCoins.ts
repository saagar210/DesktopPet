import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { CoinBalance } from "../store/types";

export function useCoins() {
  const [coins, setCoins] = useState<CoinBalance>({ total: 0, spent: 0 });

  useEffect(() => {
    invoke<CoinBalance>("get_coin_balance").then(setCoins);

    const unlisten = listen<CoinBalance>("coins-changed", (event) => {
      setCoins(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const spend = useCallback(async (amount: number) => {
    const updated = await invoke<CoinBalance>("spend_coins", { amount });
    setCoins(updated);
    return updated;
  }, []);

  return { coins, available: coins.total - coins.spent, spend };
}
