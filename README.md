# Sim Tuner Deck

White-label desktop app for a Sim Tuner golf-sim controller on the Ajazz AKP153E (15 LCD keys plus a side LCD strip). One per-user Windows NSIS setup installs the app, the device plugin, the strip plugin, and the Golf Deck profiles.

Sim Tuner Deck is based on OpenDeck by nekename (GPL-3.0-or-later), modified by Sim Tuner, 2026-11-16.
Source: https://github.com/alecweinstein/simtuner-deck

Sim Tuner (simtuner.com) is a separate closed-source application. None of its code is in this repository.

The config directory is `%APPDATA%\com.simtuner.deck`, so this install can sit next to stock OpenDeck.

## Install

Download `Sim Tuner Deck_*-setup.exe` from [GitHub Releases](https://github.com/alecweinstein/simtuner-deck/releases). The installer is per-user, embeds the WebView2 bootstrapper, adds a Start Menu shortcut and a desktop shortcut named Sim Tuner Deck, and launches the app when it finishes. It does not install a driver. The AKP153E is plain HID.

The first time an Ajazz AKP153, AKP153E (`0300:1010`), or AKP153E rev. 2 (`0300:3010`) is seen with no profiles, the app copies the bundled Golf Deck profiles into that device's profile folder and selects Golf Home. Existing profiles are not overwritten.

## What is bundled

- OpenDeck Starter Pack (Simulate Input, Switch Profile), built from the sources in `plugins/`
- Golf Deck strip plugin, `com.simtuner.golfdeck.strip`
- Ajazz AKP153 device plugin 0.11.1, pinned to `fc4116ab1fb70fecbc619b95ad59a5320b24b3ce` (`alecweinstein/simtuner-deck-akp153` tag `v0.11.1`, the same commit as `4ndv/opendeck-akp153` `v0.11.1`)

Profiles and icons live under `simtuner/`.

## Defaults

Start at login is on. The login launch uses `--hide`, so the window stays in the tray. Run in background is on. Elgato devices are disabled. There is no Aptabase telemetry, no upstream update check, and no star/donate prompt. `streamdeck://` is not registered.

## Build the Windows setup

The GitHub Actions workflow `.github/workflows/windows-setup.yml` builds the NSIS `.exe` on tag pushes matching `v*` or `simtuner-*`, and on `workflow_dispatch`. It uploads the exe as an artifact and, on a tag, attaches it to a GitHub Release.

Authenticode signing runs only when secrets are present:

- `WINDOWS_CERTIFICATE` (base64 PFX) and `WINDOWS_CERTIFICATE_PASSWORD`, or
- Azure Trusted Signing: `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`, `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_SIGNING_PROFILE`

Local build, from a Windows machine with Rust, Deno, and the usual Tauri prerequisites:

```bash
deno install
deno task tauri build --bundles nsis
```

The setup exe is written under `src-tauri/target/release/bundle/nsis/`.

## License

GPL-3.0-or-later. See `LICENSE.md`, `NOTICE`, `CHANGES`, and `THIRD-PARTY-LICENSES.md`. Those files are also copied into the install directory.
