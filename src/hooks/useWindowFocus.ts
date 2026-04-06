import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useRef } from "react";

export default function useWindowFocus<T extends HTMLElement>(onHide?: () => void) {
  const ref = useRef<T>(null);

  useEffect(() => {
    const unlisten = getCurrentWindow().onFocusChanged(({ payload }) => {
      if (payload) {
        ref.current?.focus();
      } else {
        onHide?.();
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [onHide]);

  return ref;
}
