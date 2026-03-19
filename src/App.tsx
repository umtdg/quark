import React, { useEffect, useRef, useState } from "react";
import { Box, List, ListItemButton, ListItemText, Pagination, TextField } from "@mui/material";

import { invoke } from "@tauri-apps/api/core";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";

interface PageResult<T> {
    items: T[];
    total: number;
}

interface Pagination {
    page: number;
    pageSize: number;
}

interface ItemRef {
    id: string;
    shareId: string;
    title: string;
    itype: string;
}

interface ItemLogin {
    email: string;
    username: string;
    password: string;
    urls: string[];
    totp_uri: string;
}

interface ItemContentMap {
    Login: ItemLogin;
}

type ItemType = keyof ItemContentMap;

interface ItemContent<T extends ItemType = ItemType> {
    title: string;
    note: string;
    content: { [K in T]: ItemContentMap[K] };
}

interface Item<T extends ItemType = ItemType> {
    id: string;
    shareId: string;
    vaultId: string;
    content: ItemContent<T>;
}

export default function App() {
    const PAGE_SIZE = 25;

    const [query, setQuery] = useState("");
    const [items, setItems] = useState<ItemRef[]>([]);
    const [total, setTotal] = useState(0);
    const [page, setPage] = useState(1);
    const [selectedIndex, setSelectedIndex] = useState(0);
    const listRef = useRef(null);

    useEffect(() => {
        invoke<PageResult<ItemRef>>("get_item_list", {
            pagination: {
                offset: (page - 1) * PAGE_SIZE,
                limit: PAGE_SIZE,
            }
        }).then(({ items, total }) => {
            setItems(items);
            setTotal(total);
            setSelectedIndex(0);
        });
    }, [query, page]);

    useEffect(() => { setPage(1); }, [query]);

    async function getItemInfo(itemRef: ItemRef): Promise<Item | undefined> {
        return await invoke<Item | undefined>("get_item_info", { itemRef });
    }

    function getItemContent(item: Item, itype: string): ItemContentMap[ItemType] | undefined {
        const type = itype as ItemType;

        switch (type) {
            case "Login":
                return item.content.content[type] as ItemLogin;
            default:
                return undefined;
        }
    }

    function handleKeyDown<T>(e: React.KeyboardEvent<T>) {
        const itemRef = items[selectedIndex];
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
                if (!itemRef) {
                    break;
                }

                if (e.ctrlKey) {
                    console.log("Ctrl + C");
                    getItemInfo(itemRef).then((item) => {
                        if (!item) return;

                        const content = getItemContent(item, itemRef.itype) as ItemLogin;
                        writeText(content.username.length > 0 ? content.username : content.email);
                    })
                }
                break;
            case "C":
                if (!itemRef) {
                    break;
                }

                if (e.ctrlKey) {
                    console.log("Ctrl + Shift + C");
                    getItemInfo(itemRef).then((item) => {
                        if (!item) return;

                        const content = getItemContent(item, itemRef.itype) as ItemLogin;
                        writeText(content.password);
                    });
                }
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
                count={Math.ceil(total / PAGE_SIZE)}
                page={page}
                onChange={(_, val) => setPage(val)}
            />
        </Box>
    );
}
