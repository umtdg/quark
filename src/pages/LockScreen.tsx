import { ArrowForward } from "../components";

export default function LockScreen() {
  const message = "Type your password to unlock";

  return (
    <div className="w-80 flex flex-col items-center justify-center gap-8">
      <img src="/src/assets/icon.png" width={64} height={64} />
      <div className="flex items-center w-full max-w-sm bg-bg border border-text/20 rounded-lg focus-within:ring-2 focus-within:ring-text/30">
        <input
          autoFocus
          type="password"
          placeholder={message}
          className="flex-1 px-4 py-2 bg-transparent placeholder-text/50 focus:outline-none"
        />
        <button type="submit" className="p-2 hover:bg-text/10 rounded-r-lg cursor-pointer">
          <ArrowForward className="w-5 h-5 text-text/50" />
        </button>
      </div>
    </div>
  );
}
