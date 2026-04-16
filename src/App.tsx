import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getVersion, getName } from "@tauri-apps/api/app";

import { LockScreen, QuickAccess, Setup } from "./pages";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const [isLocked, setIsLocked] = useState(true);
  const [isFirstLaunch, setIsFirstLaunch] = useState(false);
  const [version, setVersion] = useState<string | null>(null);
  const [name, setName] = useState<string | null>(null);

  function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
    if (e.key == "Escape") {
      const currentWindow = getCurrentWindow();
      currentWindow.hide();
    }
  }

  function checkState() {
    invoke<boolean>("is_first_launch").then(setIsFirstLaunch);
    invoke<boolean>("is_locked").then(setIsLocked);
  }

  useEffect(checkState, []);

  useEffect(() => {
    const unlisten = listen("state-changed", checkState);
    return () => {
      unlisten.then((fn) => fn());
    };
  });

  useEffect(() => {
    getVersion().then(setVersion);
    getName().then(setName);
  }, []);

  return (
    <div
      tabIndex={-1}
      onKeyDown={handleKeyDown}
      className="h-screen w-full flex flex-col items-center justify-center bg-bg text-text"
    >
      {isFirstLaunch ? <Setup /> : isLocked ? <LockScreen /> : <QuickAccess />}
      <span className="flex-end self-start p-1 text-xs text-text/60">
        {name ?? "Unknown"} v{version ?? "0.0.0"}
      </span>
    </div>
  );
}
