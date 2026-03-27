import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { LockScreen, QuickAccess, Setup } from "./pages";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const [isLocked, setIsLocked] = useState(true);
  const [isFirstLaunch, setIsFirstLaunch] = useState(false);

  function checkState() {
    invoke<boolean>("is_first_launch").then(setIsFirstLaunch);
    invoke<boolean>("is_locked").then(setIsLocked);
  }

  useEffect(checkState, []);

  listen("state-changed", checkState);

  return (
    <div className="h-screen w-full flex flex-col items-center justify-center bg-bg text-text">
      {isFirstLaunch ? <Setup /> : (isLocked ? <LockScreen /> : <QuickAccess />)}
    </div>
  );
}
