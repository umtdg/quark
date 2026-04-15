import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export type ShortcutAction =
  | "copy_primary"
  | "copy_secondary"
  | "copy_alt"
  | "refresh_items"
  | "lock";

export type ShortcutMap = Record<string, ShortcutAction>;

interface ShortcutState<T> {
  shortcutMap: ShortcutMap;
  keyEventToShortcut: (e: React.KeyboardEvent<T>) => string | null;
}

export default function useShortcuts<T>(): ShortcutState<T> {
  const [shortcutMap, setShortcutMap] = useState<ShortcutMap>({});

  function keyEventToShortcut<T>(e: React.KeyboardEvent<T>): string | null {
    // ignore only modifier key presses
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return null;

    const parts: string[] = [];
    if (e.ctrlKey) parts.push("ctrl");
    if (e.altKey) parts.push("alt");
    if (e.metaKey) parts.push("meta");
    if (e.shiftKey) parts.push("shift");

    parts.push(e.key.length === 1 ? e.key.toLowerCase() : e.key);
    return parts.join("-");
  }

  useEffect(() => {
    invoke<ShortcutMap>("get_shortcuts").then(setShortcutMap);
  }, []);

  return { shortcutMap, keyEventToShortcut };
}
