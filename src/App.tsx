import { useEffect, useState } from "react";
import { Box } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";

import QuickAccess from "./QuickAccess";
import LockScreen from "./LockScreen";

export default function App() {
  const [isLocked, setIsLocked] = useState(true);

  useEffect(() => {
    invoke<boolean>("is_locked")
      .then(setIsLocked)
      .catch(() => setIsLocked(true));
  }, [isLocked]);

  return (
    <Box tabIndex={-1} sx={{ outline: "none", justifyContent: "center", alignItems: "center" }}>
      {isLocked ? <LockScreen /> : <QuickAccess />}
    </Box>
  );
}
