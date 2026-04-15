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
  const commonKeyCodes: Record<string, string> = {
    Escape: "Escape",
    Enter: "Enter",
    Space: "Space",
    Tab: "Tab",
    Backspace: "Backspace",
    Delete: "Delete",

    ArrowUp: "ArrowUp",
    ArrowDown: "ArrowDown",
    ArrowLeft: "ArrowLeft",
    ArrowRight: "ArrowRight",

    Minus: "-",
    Equal: "=",
    BracketLeft: "[",
    BracketRight: "]",
    Slash: "/",
    Backslash: "\\",
    Semicolon: ";",
    Quote: "'",
    Backquote: "`",
    Comma: ",",
    Period: ".",
  };

  function getKeyFromCode(code: string): string | null {
    // KeyA...KeyZ
    if (code.startsWith("Key") && code.length === 4) {
      return code[3].toLowerCase();
    }

    // Digit0...Digit9
    if (code.startsWith("Digit") && code.length === 6) {
      return code[5];
    }

    return commonKeyCodes[code] ?? null;
  }

  function keyEventToShortcut<T>(e: React.KeyboardEvent<T>): string | null {
    // ignore only modifier key presses
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return null;

    const key = getKeyFromCode(e.code);
    if (!key) return null;

    const parts: string[] = [];
    if (e.ctrlKey) parts.push("ctrl");
    if (e.altKey) parts.push("alt");
    if (e.metaKey) parts.push("meta");
    if (e.shiftKey) parts.push("shift");

    parts.push(key);
    return parts.join("-");
  }

  useEffect(() => {
    invoke<ShortcutMap>("get_shortcuts").then(setShortcutMap);
  }, []);

  return { shortcutMap, keyEventToShortcut };
}
