# Proton Pass Quick Access

Quick access application for Proton Pass, similar to 1Password Quick Access,
using `pass-cli`.

The application accesses Proton Pass items using pass-cli. All items with
supported types are loaded on-demand and stored encrypted using a password
provided by the user on first launch.

The app itself mainly sits on the tray after the first launch, and the quick
access window can be accessed either from the tray or via running `quark show`.
It is possible to assign a shortcut to `quark show` to quickly open the app,
search for an item, and then copy.

Currently only items with type `Login` and `Credit Card` are supported, and only
two actions, primary and secondary, are supported.

**Note**: Currently I don't have any icons created for the app and I am using
Tauri's default application icons. This will change in the earliest possible
time.

## Copy Actions

### Primary Copy

Primary copy action is bound to `Ctrl-C` and copies the following by item type:

- **Login**: `username` if it is not empty, `email` otherwise. This is because
pass-cli provides us with both.
- **Credit Card**: Card number

### Secondary Copy

Secondary copy action is bound to `Ctrl-Shift-C` and copies the following by
item type:

- **Login**: Password
- **Credit Card**: Verification number

### Alternative Copy

This is currently not implemented, and planned shortcut is `Ctrl-Alt-C` with the
following copy table:

- **Login**: TOTP, pass-cli gives us an `otpauth://` uri for the TOTP and the
actual TOTP value needs to be recieved from the otpauth uri
- **Credit Card**: Expiry date

## Configuration

Location of the configuration file is platform specific. Check the below list
for its location:

- **Linux**: `$XDG_CONFIG_HOME/quark` or `$HOME/.config/quark` --
`/home/umtdg/.config/quark`
- **MacOS**: `$HOME/Library/Application Support/quark` --
`/Users/umtdg/Library/Application Support/quark`
- **Windows**: `{FOLDERID_RoamingAppData}\quark` --
`C:\Users\umtdg\AppData\Roaming\quark`

The file is named `config.toml` and contains the following fields:

```toml
# Full path to pass-cli. Setting this may be desirable for most of the users
# since on Linux and MacOS GUI apps don't have access to PATH variable exported
# from .bashrc, .zshrc, etc.
#
# Default: pass-cli
pass_cli_path = "path/to/pass-cli"

# Sets the log level. Supported levels are trace, debug, info, warn, error.
#
# Default: info
log_level = "info"
```

All of the configuration options are supported through the CLI and options
provided by the cli take precedence over the configuration file:

```shell
$ quark --help
Usage: quark [OPTIONS] [COMMAND]

Commands:
  show
  lock
  quit
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>
  -l, --log-level <LOG_LEVEL>
  -p, --pass-cli <PASS_CLI>
  -h, --help                   Print help
```

## Install

### Linux

There is a Debian package available for download under releases. There is also
an [AUR package](https://aur.archlinux.org/packages/quark-quick-access) for Arch
Linux.

### MacOS

There is an Apple Disk Image (DMG) file provided under releases. This image file
is not signed.

### Windows

There is an MSI installer for Windows. This installer is not signed.

## TODOs

- [ ] Fuzzy search support for searching in titles, usernames, and websites
- [ ] Overall better UI/UX for the user facing parts
- [x] Auto-hide after copy
- [ ] Clear clipboard after a configurable amount of seconds
- [ ] Automatically lock after a configurable amount of minutes/hours
- [ ] Add support for copying TOTP codes from an otpauth uri
- [ ] Add support for launching directly to tray instead of creating a window

<!-- :vim set sw=2 ts=2 sts=2 sw=80: -->
