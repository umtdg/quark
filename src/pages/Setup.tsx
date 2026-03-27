import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Refresh } from "../components";

export default function Setup() {
  const [password, setPassword] = useState("");
  const [passwordRepeat, setPasswordRepeat] = useState("");
  const [creating, setCreating] = useState(false);

  const passwordMatch = password.length > 0 && password === passwordRepeat;

  function initCrypto() {
    if (password.length === 0) {
      return;
    }

    if (!passwordMatch) {
      return;
    }

    setCreating(true);
    invoke("init_crypto", { password: password }).then(() => {
      console.info("Successfully initialized crypto state");
      setCreating(false);
    }).catch((reason) => {
      console.error("Error initializing crypto state:", reason);
      setCreating(false);
    })
  }

  return (
    <div className="w-80 flex flex-col items-center justify-center gap-4">
      <h1 className="font-bold">Set a password</h1>
      <input
        autoFocus
        tabIndex={1}
        type="password"
        placeholder={"Type password"}
        value={password}
        onChange={(e) => { setPassword(e.target.value) }}
        className="px-4 py-2 border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30 placeholder-text/50 focus:outline-none"
      />
      <input
        tabIndex={2}
        type="password"
        placeholder={"Type password again"}
        value={passwordRepeat}
        onChange={(e) => { setPasswordRepeat(e.target.value) }}
        className="px-4 py-2 border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30 placeholder-text/50 focus:outline-none"
      />
      <button
        tabIndex={3}
        type="submit"
        onClick={initCrypto}
        aria-busy={creating}
        disabled={creating}
        className="w-40 p-2 bg-button text-primary active:border-primary active:ring-primary rounded-lg cursor-pointer hover:bg-button-hover"
      >
        {creating ? (
          <Refresh className="w-5 h-5 fill-primary animate-spin" />
        ) : "Creating"}
      </button>
    </div>
  );
}
