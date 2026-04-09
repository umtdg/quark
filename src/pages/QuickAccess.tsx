import React, { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Alert, AlertKind, Refresh, Lock } from "../components";
import useWindowFocus from "../hooks/useWindowFocus";

interface ItemRefLogin {
  type: "login";
  login: string;
  urls: string[];
}

interface ItemRefCreditCard {
  type: "credit_card";
  masked_number: string;
}

type ItemRefData = ItemRefLogin | ItemRefCreditCard;

interface ItemRef {
  id: string;
  shareId: string;
  title: string;
  data: ItemRefData;
}

function getItemInfo(itemRef: ItemRef): string {
  switch (itemRef.data.type) {
    case "login":
      if (itemRef.data.urls.length > 0) {
        return `${itemRef.data.login} - ${itemRef.data.urls.join("-")}`;
      } else {
        return itemRef.data.login;
      }
    case "credit_card":
      return itemRef.data.masked_number;
  }
}

export default function QuickAccess() {
  const [query, setQuery] = useState("");
  const [items, setItems] = useState<ItemRef[]>([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [refreshing, setRefreshing] = useState(false);

  const [message, setMessage] = useState<{ kind: AlertKind; text: string } | undefined>(undefined);

  const searchRef = useWindowFocus<HTMLInputElement>();
  const listRef = useRef(null);

  const selectedRef: ItemRef | undefined = items[selectedIndex] ?? undefined;

  function setError(reason: unknown, fallback: string) {
    const text = typeof reason === "string" ? reason : fallback;
    setMessage({ kind: "error", text });
  }

  function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
    if (e.ctrlKey) {
      if (e.key == "c") {
        if (!selectedRef) {
          setMessage({ kind: "info", text: "No item is selected" });
          return;
        }

        setMessage(undefined);
        if (e.altKey) {
          invoke("copy_alt", { itemRef: selectedRef }).catch((reason) => {
            setError(reason, "Failed to copy alt value");
          });
        } else {
          invoke("copy_primary", { itemRef: selectedRef }).catch((reason) => {
            setError(reason, "Failed to copy primary value");
          });
        }
      } else if (e.key == "C") {
        if (!selectedRef) {
          setMessage({ kind: "info", text: "No item is selected" });
          return;
        }

        setMessage(undefined);
        invoke("copy_secondary", { itemRef: selectedRef }).catch((reason) => {
          setError(reason, "Failed to copy secondary value");
        });
      } else if (e.key == "r") {
        refreshItems();
      } else if (e.key == "l") {
        lock();
      }
    }
  }

  function getItems() {
    const trimmedQuery = query.trim();
    if (trimmedQuery.length === 0) return;

    invoke<ItemRef[]>("get_items", { query: trimmedQuery })
      .then((items) => {
        setItems(items);
        setSelectedIndex(0);
      })
      .catch((reason) => {
        setError(reason, "Failed to fetch items");
      });
  }

  function refreshItems() {
    setRefreshing(true);
    setMessage(undefined);
    invoke("refresh_items")
      .then(() => {
        setRefreshing(false);
      })
      .catch((reason) => {
        setError(reason, "Failed to refresh items");
        setRefreshing(false);
      });
  }

  function lock() {
    invoke("lock").catch((reason) => {
      setError(reason, "Failed to lock");
    });
  }

  useEffect(getItems, [query]);

  return (
    <div
      tabIndex={-1}
      onKeyDown={handleKeyDown}
      className="flex flex-col h-full w-full p-2 outline-none"
    >
      <div className="flex flex-row gap-2 p-2">
        <input
          autoFocus
          ref={searchRef}
          type="text"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            if (e.target.value.trim().length === 0) {
              setItems([]);
              setSelectedIndex(0);
            }
          }}
          placeholder="Search"
          className="flex-1 px-3 py-2 border border-text/20 rounded-lg placeholder-text/50 focus:outline-none focus:ring-2 focus:ring-text/30"
        />
        <button
          disabled={refreshing}
          onClick={refreshing ? undefined : refreshItems}
          className="p-2 rounded-lg hover:bg-text/10 cursor-pointer disabled:opacity-50 disabled:cursor-default"
        >
          <Refresh className={`w-5 h-5 fill-primary ${refreshing ? "animate-spin" : ""}`} />
        </button>
        <button
          disabled={refreshing}
          onClick={refreshing ? undefined : lock}
          className="p-2 rounded-lg hover:bg-text/10 cursor-pointer"
        >
          <Lock className="w-5 h-5 fill-primary" />
        </button>
      </div>

      <ul ref={listRef} className="flex-1 overflow-y-auto">
        {items.map((item, index) => {
          return (
            <li
              key={item.id}
              onClick={() => setSelectedIndex(index)}
              className={
                "px-4 py-3 cursor-pointer hover:bg-text/10" +
                (index === selectedIndex && " bg-text/20")
              }
            >
              <div className="text-sm font-medium">{item.title}</div>
              <div className="text-xs text-text/50">{getItemInfo(item)}</div>
            </li>
          );
        })}
      </ul>

      {message && (
        <Alert kind={message.kind} text={message.text} onExpire={() => setMessage(undefined)} />
      )}
    </div>
  );
}
