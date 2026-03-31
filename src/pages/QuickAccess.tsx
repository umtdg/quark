import React, { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Refresh, Lock } from "../components";

interface ItemRef {
  id: string;
  shareId: string;
  title: string;
  itype: string;
}

interface PageResult<T> {
  items: T[];
  total: number;
}

export default function QuickAccess() {
  const PAGE_SIZE = 25;

  const [query, setQuery] = useState("");
  const [items, setItems] = useState<ItemRef[]>([]);
  const [page, setPage] = useState(1);
  const [pageCount, setPageCount] = useState(1);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [refreshing, setRefreshing] = useState(false);
  const listRef = useRef(null);

  const selectedRef: ItemRef | undefined = items[selectedIndex] ?? undefined;

  function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
    if (e.ctrlKey && e.key == "c") {
      if (!selectedRef) {
        // TODO: better ux for informing user
        console.info("No item is selected for copying");
        return;
      }

      invoke("copy_primary", { itemRef: selectedRef }).catch((reason) => {
        console.error("failed to copy primary:", reason);
      });
    }
  }

  function getItems() {
    const trimmedQuery = query.trim().toLowerCase();
    if (trimmedQuery.length === 0) return;

    invoke<PageResult<ItemRef>>("get_items", {
      pagination: {
        offset: (page - 1) * PAGE_SIZE,
        limit: PAGE_SIZE,
      },
      query: trimmedQuery,
    })
      .then(({ items, total }) => {
        setItems(items);
        setSelectedIndex(0);
        setPageCount(Math.floor(total / PAGE_SIZE) + (total % PAGE_SIZE == 0 ? 0 : 1));
      })
      .catch((reason) => {
        console.error("failed to fetch items:", reason);
      });
  }

  function refreshItems() {
    setRefreshing(true);
    invoke("refresh_items")
      .then(() => {
        setRefreshing(false);
      })
      .catch((reason) => {
        console.error("error when refreshing items:", reason);
        setRefreshing(false);
      });
  }

  function lock() {
    invoke("lock")
      .then(() => {
        console.log("Locked successfully");
      })
      .catch((reason) => {
        console.log("Failed to lock:", reason);
      });
  }

  useEffect(getItems, [query, page]);

  return (
    <div
      tabIndex={-1}
      onKeyDown={handleKeyDown}
      className="flex flex-col h-full w-full px-2 outline-none"
    >
      <div className="flex flex-row gap-2 p-2">
        <input
          autoFocus
          type="text"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setPage(1);
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
              <div className="text-xs text-text/50">{item.itype}</div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
