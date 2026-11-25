# Sim Deck

White-label desktop app for a Sim Tuner golf-sim controller on the AKP153E (15 LCD keys plus a side LCD strip). This repository is the GPL-3.0-or-later source for the Sim Deck application, a branded fork of OpenDeck.

Sim Deck is based on OpenDeck by nekename (GPL-3.0-or-later), modified by Sim Tuner, 2026-11-16.
Source: https://github.com/alecweinstein/simtuner-deck

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. See `LICENSE.md`. Anyone who receives a Sim Deck application binary is entitled to this corresponding source.

Sim Tuner (simtuner.com) is a separate closed-source application. None of its code is in this repository.

The config directory is `%APPDATA%\com.simtuner.deck`, so this install can sit next to stock OpenDeck.

## Install

The official paid Sim Deck installer (the app plus the private Golf Deck profile pack and strip plugin) is distributed to buyers through Whop, not from this GitHub repository. Later updates are published on simtuner.com.

A build from this repository is a plain Sim Deck host: the app, the GPL OpenDeck Starter Pack, and the GPL AKP153 device plugin. It does not include the Golf Deck layouts or the strip plugin. Profile seeding runs when a pack is bundled into an official build and is a no-op otherwise.

The installer is per-user, embeds the WebView2 bootstrapper, adds a Start Menu shortcut and a desktop shortcut named Sim Deck, and can launch the app when it finishes. It does not install a driver. The AKP153E is plain HID. Installing over the 2.14.2 "Sim Tuner Deck" build removes that install, its shortcuts, and its autostart entry. Config stays in `%APPDATA%\com.simtuner.deck`.

When an official build first sees an AKP153, AKP153E (`0300:1010`), or AKP153E rev. 2 (`0300:3010`) with no profiles, it copies every bundled Golf Deck profile into that device's profile folder and selects Golf Home. On a later profile pack, missing pages are added. Golf Home is replaced when its keys still match the 2.14.3 layout (or the current bundled layout), even if OpenDeck has rewritten the file. If you changed Golf Home, that page is kept and the new sim buttons are filled into slots you left empty. Any other existing profile is left as it is.

## What is in this repository

- OpenDeck Starter Pack (Simulate Input, Switch Profile), built from the sources in `plugins/`
- AKP153 device plugin 0.11.2, built in CI from `alecweinstein/simtuner-deck-akp153` commit `fedfea2aaf3c353f80807595323c96a2cc4abf43`. The device is shown as Sim Deck. The profile folder id is unchanged.
- Host-side seeding and upgrade logic in `src-tauri/src/golfdeck.rs`, which is a no-op when no profile pack is bundled

App branding lives under `simtuner/branding/`. The paid profile pack, key icons, and Golf Deck strip plugin are a separate work and are not part of this corresponding source.

## Defaults

Start at login is on. The login launch uses `--hide`, so the window stays in the tray. Run in background is on. Elgato devices are disabled. There is no Aptabase telemetry, no upstream update check, and no star/donate prompt. `streamdeck://` is not registered.

## Build the Windows setup

The GitHub Actions workflow `.github/workflows/windows-setup.yml` builds the AKP153 plugin from the pinned commit, then builds the NSIS `.exe` on tag pushes matching `v*` or `simtuner-*`, and on `workflow_dispatch`. It uploads the exe as a CI artifact. It does not attach installers to GitHub Releases. Official paid installers are built separately, with the private pack overlaid, and are distributed through Whop and simtuner.com.

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
