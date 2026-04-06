import { useEffect } from "react";

export type AlertKind = "error" | "info";

interface AlertProps {
  kind?: AlertKind;
  duration?: number | undefined;
  text: string;
  onExpire?: () => void;
}

export default function Alert({ kind = "error", duration = 2000, text, onExpire }: AlertProps) {
  useEffect(() => {
    const timer = setTimeout(() => onExpire?.(), duration);
    return () => clearTimeout(timer);
  }, [duration, onExpire]);

  const color =
    kind === "error"
      ? "text-alert border-alert/40 bg-alert/10"
      : "text-text border-text/20 bg-text/5";

  return (
    <div className="fixed bottom-4 left-0 right-0 px-4 flex justify-center">
      <div className="w-full max-w-sm">
        <div
          role={kind === "error" ? "alert" : "status"}
          onFocus={(e) => e.currentTarget.blur()}
          className={`w-full px-3 py-2 rounded-lg border text-xs text-center wrap-break-word animate-slide-up ${color}`}
        >
          {text}
        </div>
      </div>
    </div>
  );
}
