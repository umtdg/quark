import {
  Box,
  IconButton,
  List,
  ListItemButton,
  ListItemText,
  Pagination,
  Stack,
  TextField,
} from "@mui/material";
import { RefreshRounded as RefreshIcon } from "@mui/icons-material";
import { Theme, SxProps } from "@mui/material/styles";
import React, { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

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
    console.info("pressed:", e.key);
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
      });
  }

  useEffect(getItems, [query, page]);

  let refreshIconSx: SxProps<Theme> = {};
  if (refreshing) {
    refreshIconSx = {
      animation: "spin 2s linear infinite",
      "@keyframes spin": {
        "0%": {
          transform: "rotate(-360deg)",
        },
        "100%": {
          transform: "rotate(0deg)",
        },
      },
    };
  }

  return (
    <Box onKeyDown={handleKeyDown}>
      <Stack direction="row">
        <TextField
          autoFocus
          fullWidth
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setPage(1);
          }}
          placeholder="Search"
        />
        <IconButton disabled={refreshing}>
          <RefreshIcon onClick={refreshing ? undefined : refreshItems} sx={refreshIconSx} />
        </IconButton>
      </Stack>

      <List ref={listRef}>
        {items.map((item, index) => {
          return (
            <ListItemButton
              key={item.id}
              selected={index == selectedIndex}
              onClick={() => setSelectedIndex(index)}
            >
              <ListItemText primary={item.title} secondary={item.itype} />
            </ListItemButton>
          );
        })}
      </List>

      {pageCount > 0 && (
        <Pagination count={pageCount} page={page} onChange={(_, val) => setPage(val)} />
      )}
    </Box>
  );
}
