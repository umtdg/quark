import { invoke } from "@tauri-apps/api/core";
import * as log from "@tauri-apps/plugin-log";
import React, { useState } from "react";
import { Alert, Refresh } from "../components";
import useWindowFocus from "../hooks/useWindowFocus";

export default function Setup() {
  const [password, setPassword] = useState("");
  const [passwordRepeat, setPasswordRepeat] = useState("");
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | undefined>(undefined);

  function onHide() {
    setPassword("");
    setPasswordRepeat("");
    setError(undefined);
  }
  const passwordRef = useWindowFocus<HTMLInputElement>(onHide);

  async function initCrypto(e: React.SubmitEvent<HTMLFormElement>) {
    e.preventDefault();
    setError(undefined);

    if (password.length === 0) {
      setError("Password cannot be empty");
      return;
    }

    if (password !== passwordRepeat) {
      setError("Passwords do not match");
      return;
    }

    log.info("Setting up crypto");

    try {
      setCreating(true);
      await invoke("init_crypto", { password: password });
      log.info("Successfully initialized crypto");
    } catch (reason) {
      log.error(`Error when initializing crypto: ${reason}`);
      setError(typeof reason === "string" ? reason : "Failed to initialize");
    } finally {
      setCreating(false);
    }
  }

  return (
    <form onSubmit={initCrypto} className="w-80 flex flex-col items-center justify-center gap-4">
      <h1 className="font-bold">Set a password</h1>

      <input
        autoFocus
        ref={passwordRef}
        tabIndex={1}
        type="password"
        placeholder={"Type password"}
        value={password}
        onChange={(e) => {
          setPassword(e.target.value);
        }}
        aria-busy={creating}
        disabled={creating}
        className="px-4 py-2 border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30 placeholder-text/50 focus:outline-none"
      />

      <input
        tabIndex={2}
        type="password"
        placeholder={"Type password again"}
        value={passwordRepeat}
        onChange={(e) => {
          setPasswordRepeat(e.target.value);
        }}
        aria-busy={creating}
        disabled={creating}
        className="px-4 py-2 border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30 placeholder-text/50 focus:outline-none"
      />

      <button
        tabIndex={3}
        type="submit"
        aria-busy={creating}
        disabled={creating}
        className="w-40 p-2 bg-button text-primary inline-flex items-center justify-center active:border-primary active:ring-primary rounded-lg cursor-pointer hover:bg-button-hover"
      >
        {creating ? <Refresh className="w-5 h-5 fill-primary animate-spin" /> : "Set-up"}
      </button>

      {error && <Alert kind="error" text={error} onExpire={() => setError(undefined)} />}
    </form>
  );
}
