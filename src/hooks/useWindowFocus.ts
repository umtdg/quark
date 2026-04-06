import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useRef } from "react";

export default function useWindowFocus<T extends HTMLElement>() {
  const ref = useRef<T>(null);

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload }) => {
      if (payload) ref.current?.focus();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return ref;
}
