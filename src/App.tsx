import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { LockScreen, QuickAccess, Setup } from "./pages";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const [isLocked, setIsLocked] = useState(true);
  const [isFirstLaunch, setIsFirstLaunch] = useState(false);

  function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
    console.log("Got key: ", e.key);

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

  listen("state-changed", checkState);

  return (
    <div
      tabIndex={-1}
      onKeyDown={handleKeyDown}
      className="h-screen w-full flex flex-col items-center justify-center bg-bg text-text"
    >
      {isFirstLaunch ? <Setup /> : isLocked ? <LockScreen /> : <QuickAccess />}
    </div>
  );
}
