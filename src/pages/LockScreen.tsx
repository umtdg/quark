import { invoke } from "@tauri-apps/api/core";
import * as log from "@tauri-apps/plugin-log";
import { ArrowForward, Refresh } from "../components";
import { useState } from "react";

export default function LockScreen() {
  const [password, setPassword] = useState("");
  const [unlocking, setUnlocking] = useState(false);

  async function unlock(e: React.SubmitEvent<HTMLFormElement>) {
    try {
      e.preventDefault();
      setUnlocking(true);
      await invoke("unlock", { password: password });
      log.debug("Unlocked successfully");
    } catch {
      log.error("Error when unlocking");
    } finally {
      setUnlocking(false);
    }
  }

  return (
    <div className="w-80 flex flex-col items-center justify-center gap-8">
      <img src="/src/assets/icon.png" width={64} height={64} />
      <form
        onSubmit={unlock}
        className="flex items-center w-full max-w-sm bg-bg border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30"
      >
        <input
          autoFocus
          type="password"
          placeholder={"Enter your password to unlock"}
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          aria-busy={unlocking}
          disabled={unlocking}
          className="flex-1 px-4 py-2 bg-transparent placeholder-text/50 focus:outline-none"
        />
        <button
          type="submit"
          aria-busy={unlocking}
          disabled={unlocking}
          className="px-2 py-2 text-primary hover:bg-text/10 rounded-r-lg cursor-pointer"
        >
          {unlocking ? (
            <Refresh className="w-5 h-5 animate-spin" />
          ) : (
            <ArrowForward className="w-5 h-5 hover:" />
          )}
        </button>
      </form>
    </div>
  );
}
