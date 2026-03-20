import React, { useEffect, useRef, useState } from "react";
import { Box, List, ListItemButton, ListItemText, Pagination, TextField } from "@mui/material";

import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

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

export default function App() {
    const PAGE_SIZE = 25;

    const [query, setQuery] = useState("");
    const [items, setItems] = useState<ItemRef[]>([]);
    const [page, setPage] = useState(1);
    const [pageCount, setPageCount] = useState(1);
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [selectedRef, setSelectedRef] = useState<ItemRef | undefined>(undefined)
    const listRef = useRef(null);

    useEffect(() => {
        invoke<PageResult<ItemRef>>("get_items", {
            pagination: {
                offset: (page - 1) * PAGE_SIZE,
                limit: PAGE_SIZE,
            },
            query: query.toLowerCase(),
        }).then(({ items, total }) => {
            setItems(items);
            setSelectedIndex(0);
            setSelectedRef(items.length > 0 ? items[selectedIndex] : undefined);

            const pageCount = Math.floor(total / PAGE_SIZE);
            const lastPage = (total % PAGE_SIZE == 0) ? 0 : 1;
            setPageCount(pageCount + lastPage);
        });
    }, [query, page]);

    useEffect(() => { setPage(1); }, [query]);

    useEffect(() => {
        if (items.length === 0 || selectedIndex < 0 || selectedIndex >= items.length) {
            setSelectedRef(undefined);
        } else {
            setSelectedRef(items[selectedIndex]);
        }
    }, [selectedIndex]);

    function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
        switch (e.key) {
            case "ArrowDown":
                e.preventDefault();
                setSelectedIndex(i => Math.min(i + 1, items.length - 1));
                break;
            case "ArrowUp":
                e.preventDefault();
                setSelectedIndex(i => Math.max(i - 1, 0));
                break;
            case "c":
                if (!e.ctrlKey) {
                    break;
                }

                if (!selectedRef) {
                    break;
                }

                const fn = e.altKey ? "copy_alt" : "copy_primary"
                invoke<string | undefined>(fn, { itemRef: selectedRef })
                    .then((secret) => {
                        if (secret) {
                            writeText(secret);
                        }
                    });
                break;
            case "C":
                if (!e.ctrlKey) {
                    break;
                }

                if (!selectedRef) {
                    break;
                }

                invoke<string | undefined>("copy_secondary", { itemRef: selectedRef })
                    .then((secret) => {
                        if (secret) {
                            writeText(secret);
                        }
                    })
                break;
            // add other shortcuts here
        }
    };

    return (
        <Box onKeyDown={handleKeyDown} tabIndex={-1} sx={{ outline: "none" }}>
            <TextField
                autoFocus
                fullWidth
                value={query}
                onChange={e => setQuery(e.target.value)}
                placeholder="Search"
            />

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

            <Pagination
                count={pageCount}
                page={page}
                onChange={(_, val) => setPage(val)}
            />
        </Box>
    );
}
