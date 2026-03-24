# Encrypted State Implementation Plan

## 1. Dependencies

Add the following to `Cargo.toml`:

- `aes-gcm = "0.10"` — AES-256-GCM via `Aes256Gcm`, `Nonce`, `Key`, `KeyInit`, `Aead`
- `argon2 = "0.5"` — Argon2id via `Argon2`, `Params`, `Algorithm::Argon2id`, `Version::V0x13`
- `rand = "0.8"` — CSPRNG via `OsRng`, `RngCore::fill_bytes`
- `zeroize = { version = "1", features = ["derive"] }` — `Zeroize`, `ZeroizeOnDrop` derives
- `base64 = "0.22"` — `engine::general_purpose::STANDARD` for JSON encoding of binary fields
- `serde`, `serde_json` — assumed already present

## 2. Structs

### On-disk

`AppState` is the JSON root written to disk. Fields:

- `kdf: String` — always `"argon2id"`
- `kdf_params: KdfParams` — `m_cost: u32`, `t_cost: u32`, `p_cost: u32`
- `salt: [u8; 16]` — base64-encoded via `base64_serde`
- `wrapped_dek: EncryptedData` — the DEK encrypted under the KEK
- `records: HashMap<String, EncryptedRecord>` — keyed by `"{share_id}/{id}"`
- Derives: `Debug, Serialize, Deserialize`

`EncryptedRecord` wraps an `EncryptedData` with unencrypted timestamps:

- `encrypted: EncryptedData` — the encrypted `ItemContent`
- `create_time: DateTime<Utc>` — unencrypted, via existing `date` serde module
- `modify_time: DateTime<Utc>` — unencrypted, via existing `date` serde module
- Derives: `Debug, Serialize, Deserialize`

`EncryptedData` is the reusable nonce+ciphertext pair (already implemented):

- `nonce: [u8; 12]` — base64-encoded
- `data: Vec<u8>` — base64-encoded
- Derives: `Debug, Serialize, Deserialize`
- Has `decrypt<T: TryFrom<Vec<u8>>>(&self, key: &[u8]) -> Result<T>` already implemented

`KdfParams` already implemented with `new()` returning defaults (m=65536, t=3, p=4).

### In-memory only

`Dek` — tuple struct `[u8; 32]`, derives `Zeroize`, `ZeroizeOnDrop`. Already implemented with:

- `Dek::new() -> Self` — generates random DEK
- `Dek::encrypt(&self, kek: &Kek) -> Result<EncryptedData>` — wraps itself under the KEK

Add:

- `Dek::decrypt_from(kek: &Kek, encrypted: &EncryptedData) -> Result<Self>` — unwraps a DEK from an `EncryptedData` using the KEK. Uses `EncryptedData::decrypt` internally, converts the resulting `Vec<u8>` to `[u8; 32]`, returns `Dek(bytes)`. GCM auth failure means wrong password.
- `Dek::encrypt_record(&self, plaintext: &[u8]) -> Result<EncryptedData>` — encrypts arbitrary plaintext under this DEK. Generates a fresh nonce, encrypts with AES-256-GCM, returns `EncryptedData`.
- `Dek::decrypt_record(&self, record: &EncryptedData) -> Result<Vec<u8>>` — decrypts an `EncryptedData` using this DEK. Delegates to `EncryptedData::decrypt` with `self.0` as the key.

`Kek` — tuple struct `[u8; 32]`, derives `Zeroize`, `ZeroizeOnDrop`. Already implemented with `Kek::new(password, salt, params)`.

### Frontend-facing

`ItemRef` is what the frontend receives. It must contain all fields the user sees in the quick access window and all fields that are searchable. Updated fields:

- `id: String`
- `share_id: String`
- `title: String`
- `itype: String` — `"Login"` or `"CreditCard"`
- `username: String` — empty string for non-login items
- `email: String` — empty string for non-login items
- `urls: Vec<String>` — empty vec for non-login items

`ItemRef` is built by decrypting `ItemContent` during `get_items`, extracting these fields, then dropping the decrypted `ItemContent`. The `From<&Item>` impl needs updating to populate the new fields.

Search operates on `ItemRef` fields: `title`, `username`, `email`, `urls`. No decryption at search time.

## 3. Crypto Functions

Already implemented and properly scoped to their structs:

- `Dek::new()` — generates random 32-byte DEK
- `Dek::encrypt(&self, kek: &Kek)` — wraps DEK under KEK, returns `EncryptedData`
- `Kek::new(password, salt, params)` — derives KEK via Argon2id
- `EncryptedData::decrypt(&self, key)` — generic decryption
- `generate_salt()` — 16 random bytes
- `generate_nonce()` — 12 random bytes

To add:

- `Dek::decrypt_from(kek, encrypted)` — unwrap DEK from `EncryptedData`
- `Dek::encrypt_record(&self, plaintext)` — encrypt arbitrary data under DEK
- `Dek::decrypt_record(&self, record)` — decrypt `EncryptedData` under DEK

`generate_salt` and `generate_nonce` stay as free functions. They don't belong to any struct.

## 4. State File I/O

`AppState` gets two associated functions:

`AppState::load(path: &Path) -> Option<AppState>` — reads and deserializes. Returns `None` if file doesn't exist.

`AppState::save(&self, path: &Path) -> Result<()>` — serializes to JSON, writes to a temp file in the same directory, renames over the target. Atomic write.

The path is under Tauri's `app_data_dir`.

## 5. Runtime State

A struct (e.g. `AppManager`) behind a `Mutex` in Tauri's managed state. Fields:

- `dek: Option<Dek>` — `Some` = unlocked, `None` = locked
- `state_path: PathBuf` — resolved at startup
- `app_state: Option<AppState>` — loaded from disk, updated on refresh
- `refs: Vec<ItemRef>` — built during `get_items`

## 6. Application Lifecycle

### Startup

1. Resolve `state_path` under Tauri's `app_data_dir`.
2. Attempt `AppState::load`.
3. Run `pass-cli test`. If it fails, the app stays in tray with no window. The periodic check will detect when `pass-cli test` starts succeeding.
4. If no state file exists, show the window with the setup/password-creation view.
5. If state file exists, show the window with the unlock/password-entry view.

### Tray

The app runs as a tray-only application. No dock icon, no taskbar window entry. The tray icon has a context menu with:

- **Quick Access** — shows/focuses the window. If locked, shows password prompt. If unlocked, shows the item list.
- **Lock** — zeroizes DEK, sets `dek` to `None`. If window is visible, it switches to the password prompt view.
- **Quit** — zeroizes DEK, exits the process.

### CLI Subcommands

The binary accepts subcommands that communicate with the running tray instance (e.g. via a Unix domain socket, named pipe, or Tauri's single-instance plugin with deep links):

- `app launch` — tells the running instance to show/focus the quick access window
- `app lock` — tells the running instance to lock
- `app quit` — tells the running instance to quit

If no instance is running, `launch` starts the app. `lock` and `quit` are no-ops or print an error.

### Window

A single window serves all views. It tracks locked/unlocked state internally:

- **Setup view** — shown when no state file exists. Accepts a new password. On submit, calls `setup`.
- **Unlock view** — shown when state file exists but DEK is not in memory. Accepts password. On submit, calls `unlock`.
- **Quick access view** — shown when DEK is in memory. Displays `ItemRef` list, search bar, copy actions.
- **Password change** — accessible from within the quick access view.

The window can be closed without locking. Closing hides the window; the app remains in the tray. Showing the window again resumes wherever the state was.

### Periodic `pass-cli test`

A background timer (e.g. every 30 seconds) runs `pass-cli test`. On failure: zeroize DEK, set to `None`, clear `refs`, notify the frontend to switch to the unlock view. The timer continues running while locked — when `pass-cli test` succeeds again, the frontend can prompt for the password.

## 7. Tauri Commands

### `setup(password: String)`

Precondition: no state file exists.

1. Generate salt.
2. Derive KEK from password + salt + default `KdfParams`.
3. Generate DEK.
4. Wrap DEK with KEK via `dek.encrypt(&kek)`.
5. Construct `AppState` with kdf = `"argon2id"`, params, salt, wrapped DEK, empty records.
6. Save state file.
7. Store DEK in runtime state. App is unlocked.
8. Zeroize KEK. Password is dropped (owned `String`, not `Zeroize` — consider converting to `Vec<u8>` and zeroizing manually).
9. Trigger `refresh_items` to populate records.

### `unlock(password: String)`

Precondition: state file exists, DEK is `None`.

1. Load salt and kdf params from `app_state`.
2. Derive KEK from password + salt + params.
3. Attempt `Dek::decrypt_from(&kek, &app_state.wrapped_dek)`.
4. If GCM auth fails, return wrong-password error.
5. If success, store DEK in runtime state. App is unlocked.
6. Zeroize KEK and password.

### `lock()`

Zeroize and drop DEK (set to `None`). Clear `refs`. Frontend switches to unlock view.

### `refresh_items(app_handle: AppHandle)`

Precondition: DEK is `Some`.

1. Emit `refresh-started`.
2. Run `pass-cli` to fetch all items as JSON, parse into `Vec<Item>`.
3. For each item:
   - Serialize `item.content` (`ItemContent`) to JSON bytes.
   - Encrypt with `dek.encrypt_record(&json_bytes)`.
   - Build composite key `"{share_id}/{id}"`.
   - Insert into `app_state.records` as `EncryptedRecord { encrypted, create_time, modify_time }`.
   - Zeroize the JSON bytes after encryption.
4. Save state file.
5. Emit `refresh-completed`.
6. On failure at any point, emit `refresh-failed`.

Does not build `refs`. That happens in `get_items`.

### `get_items(query: Option<String>) -> Vec<ItemRef>`

Precondition: DEK is `Some`.

1. For each entry in `app_state.records`:
   - Decrypt the `EncryptedData` to get `ItemContent`.
   - Build `ItemRef` from the composite key (split on `/` to recover `share_id` and `id`) and the decrypted `ItemContent` (title, itype, username, email, urls).
   - Drop the decrypted `ItemContent`.
2. If `query` is `Some`, filter the `ItemRef` list: case-insensitive substring match on `title`, `username`, `email`, and each entry in `urls`.
3. Store the full (unfiltered) list in `refs` for reference.
4. Return the (possibly filtered) list.

Every call to `get_items` decrypts all records to build fresh `ItemRef`s. This is the tradeoff for not storing searchable fields in plaintext on disk.

### `copy_primary(id: String, share_id: String)`

Precondition: DEK is `Some`.

1. Build composite key `"{share_id}/{id}"`.
2. Look up `EncryptedRecord` in `app_state.records`.
3. Decrypt to `ItemContent`.
4. Call `item_content.get_primary()`, copy result to OS clipboard.
5. Zeroize decrypted `ItemContent`.

### `copy_secondary(id: String, share_id: String)`

Same as `copy_primary`, calls `get_secondary()`.

### `copy_alt(id: String, share_id: String)`

Same as `copy_primary`, calls `get_alt()`.

### `change_password(current_password: String, new_password: String)`

Precondition: DEK is `Some` (app is unlocked).

1. Derive current KEK from `current_password` + stored salt + stored params.
2. Attempt `Dek::decrypt_from(&current_kek, &app_state.wrapped_dek)`.
3. If GCM auth fails, return wrong-password error.
4. Generate new salt.
5. Derive new KEK from `new_password` + new salt + params.
6. Re-wrap the existing in-memory DEK with new KEK via `dek.encrypt(&new_kek)`.
7. Update `app_state` with new salt, new wrapped DEK.
8. Save state file.
9. Zeroize both KEKs and both passwords.
10. Records are untouched — encrypted with the DEK, which hasn't changed.

## 8. Biometric Unlock (Optional, Later)

On enrollment: store the DEK directly in the OS keyring, gated behind biometric authentication (macOS `kSecAccessControlBiometryCurrentSet`, Windows Hello). On biometric unlock: retrieve DEK from keyring instead of deriving KEK from password. Both paths produce the same DEK in memory. If biometric retrieval fails, fall back to password unlock.
