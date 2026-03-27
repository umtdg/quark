import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { LockScreen, QuickAccess } from "./pages";

export default function App() {
  const [isLocked, setIsLocked] = useState(true);

  useEffect(() => {
    invoke<boolean>("is_locked")
      .then(setIsLocked)
      .catch(() => setIsLocked(true));
  }, [isLocked]);

  return (
    <div className="h-screen w-full flex flex-col items-center justify-center bg-bg text-text">
      {isLocked ? <LockScreen /> : <QuickAccess />}
    </div>
  );
}
