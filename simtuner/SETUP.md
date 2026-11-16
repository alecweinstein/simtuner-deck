# Golf Deck for OpenDeck — setup guide

A set of OpenDeck profiles that turn an **Ajazz AKP153E** (15 LCD keys, used with the [opendeck-akp153](https://github.com/4ndv/opendeck-akp153) plugin) into a golf-sim controller. There is a **Golf Home** profile plus modes for **GSPro**, **PurePlay** (and **PurePlay 2** for extras), **Muni Golf**, and **Sim Tuner**. A Home key on every mode returns you to Golf Home.

This package lives at `/workspace/golfdeck/`. The zip is `/workspace/golfdeck/golfdeck.zip`.

---

## 1. What you need

- The Ajazz AKP153E plugged in over USB-C.
- [OpenDeck](https://github.com/nekename/OpenDeck) 2.5.0 or newer (the AKP153 plugin's minimum). 2.12.0+ has a full-config backup/restore in Settings, which is a good idea before you copy files.
- The **AKP153 / Mirabox HSV293S plugin** already installed (Plugins → Install from file), from [4ndv/opendeck-akp153](https://github.com/4ndv/opendeck-akp153/releases).
- The built-in **OpenDeck Starter Pack** plugin, which ships with OpenDeck. The profiles use two of its actions:
  - **Simulate Input** (`com.amansprojects.starterpack.inputsimulation`) — keyboard shortcuts.
  - **Switch Profile** (`com.amansprojects.starterpack.switchprofile`) — mode changes.
- The **Golf Deck Strip** plugin (`com.simtuner.golfdeck.strip.sdPlugin`), which this pack's installer copies into `%APPDATA%\opendeck\plugins\`. It paints live Sim Tuner data (club, distance to pin, last shot) on the side strip of the GSPro, GSPro 2, and Sim Tuner profiles. It needs Sim Tuner running with the local deck-status API (see section 5.1).
- There is no separate "Open App" action. To launch a program, use **Run Command**. Paths are machine-specific, so those keys are not pre-filled.

---

## 2. The device, as OpenDeck sees it

The AKP153E is a **15-key LCD pad** in a physical **3 row × 5 column** grid. It has **no rotary knobs, no touch strip, and no extra physical buttons**. Each LCD key is **85×85 px** on the original firmware (VID:PID `0300:1010`) and **95×95 px** on Rev. 2 (`0300:3010`), with the side strip regions at **82×82**. Images are sent as JPEG; the plugin rotates and mirrors them, so the icons in this pack are drawn upright.

The official plugin does **not** register a 3×5 device. OpenDeck only supports a rectangular key grid, so the plugin registers **3 rows × 6 columns = 18 keys**. The sixth column is the **side LCD strip**, split into three regions (positions 5, 11, and 17). **Alec confirmed that strip does not register presses**, so it is used as a display-only strip. On **GSPro**, **GSPro 2**, and **Sim Tuner**, positions 5 / 11 / 17 show live Club, Distance, and Last Shot data from Sim Tuner. **PurePlay**, **PurePlay 2**, **Muni Golf**, and **Golf Home** leave the strip empty. Your 15 real (pressable) keys are columns 1–5.

Logical OpenDeck positions (0-based, left-to-right, top-to-bottom):

```
 0   1   2   3   4    5  ← side strip
 6   7   8   9  10   11  ← side strip
12  13  14  15  16   17  ← side strip
```

On Muni Golf and Sim Tuner, **HOME** is position **16** (bottom row, fifth physical key) and returns to Golf Home. On GSPro and PurePlay page 1, that same key is **MORE** (opens page 2). On page 2, **BACK** sits there (returns to page 1) and **HOME** is one key to the left, so Golf Home is still reachable.

Device IDs, from the plugin source:

- Original AKP153E (`0300:1010`, protocol v1): the serial is shared across units and is hardcoded, so the folder is almost certainly  
  `99-355499441494-153E`
- Rev. 2 (`0300:3010`, protocol v3): `99-<the unit's real serial number>`

---

## 3. How OpenDeck profiles work (Windows)

**Location.** OpenDeck's identifier is `opendeck`, so the Windows config directory is:

```
%APPDATA%\opendeck
```

that is, `C:\Users\<you>\AppData\Roaming\opendeck`. Confirm it from OpenDeck: **Settings → Open config directory**.

Layout, from OpenDeck's own source (`src-tauri/src/store/simplified_profile.rs` and the [FAQ](https://github.com/nekename/OpenDeck/wiki/0.-FAQ)):

```
%APPDATA%\opendeck\
  profiles\
    <device_id>.json          ← which profile is selected
    <device_id>\
      Default.json
      Golf Home.json          ← you copy these here
      GSPro.json
      ...
  images\
    <device_id>\
      <profile>\
        Keypad.<pos>.<idx>\   ← created automatically if a profile
                                embeds a data: URI and is later re-saved
  applications.json           ← auto-switch map (optional)
  plugins\
```

Each profile is one JSON file named `<Profile Name>.json`. The file's `keys` array is a list of action instances (or `null`). The profiles in this pack embed icons as `data:image/png;base64,...`, which OpenDeck accepts on load (the loader leaves `data:` URIs alone). After you first edit and save a profile in the UI, OpenDeck may extract those images into the `images\` folder.

**There is no individual profile import in the UI.** As of OpenDeck 2.12.0 you can backup and restore the whole config, but not one profile at a time ([discussion #298](https://github.com/nekename/OpenDeck/discussions/298)). The supported way to add these is to copy the JSON files into the device folder.

**Auto-switch by active application.** Yes. OpenDeck's `application_watcher` polls the foreground window every 250 ms. If that application's name is in `applications.json`, it switches to the mapped profile. The map is `application name → device id → profile name`. On Windows the name comes from the focused process (often something like `GSPro` or `GSPro.exe` — check what OpenDeck lists after you have each game open). You can also set this in the OpenDeck UI if your version shows an application-to-profile picker. A sample file is in `applications.sample.json`. Do not copy it over an existing `applications.json` until you have filled in your real device id and process names.

**Creating profiles yourself.** In OpenDeck, use the profile dropdown to add a profile, then drag **Simulate Input** or **Switch Profile** onto a key. Input Simulation uses [enigo](https://github.com/enigo-rs/enigo/blob/v0.5.0/src/keycodes.rs) RON syntax, documented in the [OpenDeck FAQ](https://github.com/nekename/OpenDeck/wiki/0.-FAQ):

```
[k(uni('i'),c)]                          # tap I
[k(LeftArrow,c)]                         # tap Left Arrow
[k(ctrl,p),k(uni('m'),c),k(ctrl,r)]      # Ctrl+M
[k(Space,p)]   on key-down,  [k(Space,r)] on key-up   # hold Space
```

---

## 4. Install these profiles

### Option A — installer (recommended on Windows)

OpenDeck and the AKP153 plugin must already be installed, and you should have opened the pad in OpenDeck at least once so a device folder exists under `%APPDATA%\opendeck\profiles\`.

1. Run **`dist\GolfDeckSetup.exe`**. The `.exe` is **unsigned**, so Windows SmartScreen will likely say "Windows protected your PC." Choose **More info**, then **Run anyway**. The installer does not need administrator rights; it only writes under `%APPDATA%\opendeck`.
2. If you prefer not to run the `.exe`, double-click **`installer\Install-GolfDeck.bat`** (same logic, in PowerShell). Windows may still prompt because the script is unsigned; that is expected.

The installer will:

1. Find `%APPDATA%\opendeck`.
2. Close OpenDeck if it is running (`opendeck.exe` — that process name is from OpenDeck's own source, `mainBinaryName` / `default-run`).
3. Find Ajazz AKP153 device folder(s) under `profiles\` (names starting `99-` that match the [opendeck-akp153](https://github.com/4ndv/opendeck-akp153) plugin IDs: v1 `99-355499441494-153`, `…-153E`, or `…-153R`; Rev. 2 `99-<serial>`). If several match, it installs to all of them. If none exist, it tells you to plug the deck in, open OpenDeck, change one key, quit OpenDeck, and rerun.
4. Back up each matching device folder to `%APPDATA%\opendeck\GolfDeck-backups\<timestamp>\`.
5. Copy all seven Golf Deck profile JSON files into each folder.
6. Install the Golf Deck Strip plugin into `%APPDATA%\opendeck\plugins\com.simtuner.golfdeck.strip.sdPlugin` (backs up any existing copy). If the plugin folder is missing from the pack, the profiles are still installed and the strip tiles stay on their baked-in offline image.
7. **Skip** `applications.json` auto-switch. OpenDeck keys auto-switch on the focused window's application name, and the exact names for GSPro, PurePlay, Muni Golf, and Sim Tuner are not a verified published list. After install, you can set mappings in OpenDeck or merge `applications.sample.json` once you know the names OpenDeck shows.
8. Restart OpenDeck (or tell you to start it if the `.exe` path cannot be found). The strip plugin is started by OpenDeck at launch.
9. Show a success or failure message. A log is saved in the backup folder.

Then select **Golf Home** from the profile list. Press **GS**, **PP**, **MU**, or **ST** to enter a mode; press **HOME** (bottom-right physical key) to return.

A 64-bit Windows is required for `GolfDeckSetup.exe`. The `.bat` fallback uses the built-in Windows PowerShell and does not have that restriction.

### Option B — copy the files yourself

1. In OpenDeck, **Settings → Open config directory**.
2. Optional but recommended: make a copy of the whole `opendeck` folder, or use Settings → Backup if you have 2.12.0+.
3. Open `profiles\`. You should see a folder named after your device (`99-355499441494-153E` on a v1 AKP153E, or `99-<serial>` on Rev. 2). If the folder is not there, plug the pad in, start OpenDeck, and it will create one.
4. **Quit OpenDeck completely** (tray icon included) so it does not overwrite files as you copy.
5. Copy these seven files from `profiles\` in this pack into that device folder, next to `Default.json`:

   - `Golf Home.json`
   - `GSPro.json`
   - `GSPro 2.json`
   - `PurePlay.json`
   - `PurePlay 2.json`
   - `Muni Golf.json`
   - `Sim Tuner.json`

   If you already have profiles with those exact names, rename the old ones first.
6. Copy the folder `plugin\com.simtuner.golfdeck.strip.sdPlugin` into `%APPDATA%\opendeck\plugins\`, so the final path is `%APPDATA%\opendeck\plugins\com.simtuner.golfdeck.strip.sdPlugin\` (it must contain `manifest.json` and `golfdeck-strip.exe`). Quit OpenDeck before copying, then start it so it launches the plugin.
7. Start OpenDeck. Select **Golf Home** from the profile list. Press **GS**, **PP**, **MU**, or **ST** to enter a mode. On Muni and Sim Tuner, **HOME** (bottom-right) returns to Golf Home. On GSPro and PurePlay, bottom-right is **MORE** (page 2); **HOME** is on page 2, one key left of **BACK**.

If a key does nothing:

- Make sure the Starter Pack plugin is installed and enabled.
- If the **side strip** shows a dimmed "Sim Tuner offline" tile, start Sim Tuner (with the deck-status API) and confirm nothing is blocking `127.0.0.1:8080`. The strip does not accept presses.
- Make sure the game window is focused. OpenDeck types into the foreground app. Sim Tuner in particular only listens for keys while its own window is focused (there is no global hotkey in the code).
- If a key's property inspector is blank, the image paths still work but the inspector path may not resolve until you re-assign the action. You can drag a fresh **Simulate Input** onto the key and paste the same `down` command from `SHORTCUTS.md`.

---

## 5. Layouts

Rightmost column is the side strip (display only — presses do not register). On GSPro, GSPro 2, and Sim Tuner it shows live Club, Distance, and Last Shot data from Sim Tuner. On PurePlay, Muni, and Golf Home it is empty. "Official" means GSPro's knowledge base or PurePlay's in-game overlay (screenshot Alec sent on 12 Nov 2026). Sim Tuner bindings are from the repo source. Muni bindings are community-reported defaults (the game lets you remap them).

### Golf Home

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip |
|---|---|---|---|---|---|---|
| Row 1 | GSPro | PurePlay | Muni Golf | Sim Tuner | — | — |
| Row 2 | — | — | — | — | — | — |
| Row 3 | — | — | — | — | — | — |

### GSPro — all official

Aim Point sits at the end of the aim row, next to the aim arrows. That is a pack-wide rule. The official key is **F3**: "your camera will alternate between your selected aim point on the mini-map/screen, the hole, and back to the ball" ([GSPro Knowledge Base — Keyboard Shortcuts](https://gspro.gitbook.io/gspro-knowledge-base/in-game-information/keyboard-shortcuts)). Club Up and Club Down sit together in the lower-left corner of the main grid (positions 12–13). Heatmap, Reset Aim, and Range Finder are on **GSPro 2**. **MORE** (bottom-right) opens GSPro 2. Mulligans only work if they are enabled for the round.

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip (display) |
|---|---|---|---|---|---|---|
| Row 1 | Mulligan **Ctrl+M** | Tee Left **C** | Tee Right **V** | Putter **U** | Flyover **O** | **Club** (live) |
| Row 2 | Aim Left **←** | Aim Forward **↑** | Aim Back **↓** | Aim Right **→** | Aim Point **F3** | **Distance** (live) |
| Row 3 | Club Up **I** | Club Down **K** | Scorecard **T** | Green Grid **G** | **MORE** | **Last Shot** (live) |

### GSPro 2 — Range Finder, Heatmap, Reset Aim, and other official extras

**BACK** sits on the same physical key as **MORE** on page 1 and returns you to GSPro. **HOME** is one key to the left and goes to Golf Home. Heatmap and Reset Aim sit in previously empty slots on row 2. Fast Forward is a **hold**. Fly To Ball here is official **6** (Camera Fly to Ball) — not the same as PurePlay's B.

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip (display) |
|---|---|---|---|---|---|---|
| Row 1 | Range Finder **R** | Tee Left **C** | Tee Right **V** | Switch Hand **N** | Free Look **F5** | **Club** (live) |
| Row 2 | Fast Forward **hold Space** | Fly To Ball **6** | Heatmap **Y** | Hide UI **H** | Reset Aim **A** | **Distance** (live) |
| Row 3 | Shortcuts **Tab** | Fullscreen **F11** | — | **HOME** | **BACK** | **Last Shot** (live) |

### PurePlay — official in-game overlay

Source: PurePlay's in-game Keyboard Overlay, screenshot from Alec, 12 Nov 2026. The 5 / 6 camera-height keys from an earlier GSV closed-beta first look are **not** on this official list and are no longer bound.

Aim Point is at the end of the aim row, same place as on GSPro, next to the aim arrows. Club Up and Club Down sit together in the lower-left corner of the main grid (positions 12–13). Heat Map and Reset Aim are on **PurePlay 2**. **MORE** (bottom-right) opens PurePlay 2.

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip |
|---|---|---|---|---|---|---|
| Row 1 | Mulligan **Ctrl+M** | Tee Left **C** | Tee Right **V** | Putter **U** | Flyover **O** | — |
| Row 2 | Aim Left **←** | Aim Forward **↑** | Aim Back **↓** | Aim Right **→** | Aim Point **F3** | — |
| Row 3 | Club Up **I** | Club Down **K** | Green Grid **G** | Fly To Ball **B** | **MORE** | — |

**Fly To Ball is bound to B.** That is the best reading of the official overlay ("Camera Fly To Ball"). It is possible the key is **8**; the two can look similar in the screenshot, and this pack's builder could not reopen the original image file to settle it independently. If Fly To Ball does nothing, change that key's Simulate Input command to `[k(uni('8'),c)]`.

This is **Beacon Games' Pureplay** (also written PurePlay / Pure Play), Windows-only — **not** the older ProTee Play product.

### PurePlay 2 — remaining official shortcuts

**BACK** sits on the same physical key as **MORE** on page 1 and returns you to PurePlay. **HOME** is one key to the left and still goes to Golf Home. Heat Map and Reset Aim took the row-1 slots that previously held Tee Left / Tee Right (those tees are already on page 1's main grid, and page 2 had no empty main-grid slots). **Scorecard** stays on this page. Fast Forward is a **hold**: Space is pressed on key-down and released on key-up.

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip |
|---|---|---|---|---|---|---|
| Row 1 | Heat Map **Y** | Reset Aim **A** | Skip **F7** | Switch Hand **N** | First Person **F6** | — |
| Row 2 | Scorecard **T** | Fly To Aim **J** | Fast Forward **hold Space** | HUD **H** | Ball Trails **F1** | — |
| Row 3 | Shortcuts **Tab** | Mute All **Num-** | Fullscreen **F11** | **HOME** | **BACK** | — |

Official extras not on either page: **X** Push to Talk, **F** Toggle FPS. **Toggle Weather Settings** is skipped because the key in the screenshot was unreadable.

### Muni Golf — community-reported defaults, remappable

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip |
|---|---|---|---|---|---|---|
| Row 1 | Mulligan **M** | Club Up **I** | Club Down **K** | Shot Preview **P** | Flyover **H** | — |
| Row 2 | Aim Left **←** | Aim Further **↑** | Aim Shorter **↓** | Aim Right **→** | Reset Aim **R** | — |
| Row 3 | Scorecard **S** | Green Slope **G** | Replay **V** | Free Cam **F** | **HOME** | — |

Source: forum user Bubba22's list of "all the keyboard bindings" on [Golf Simulator Forum, 13 Oct 2026](https://golfsimulatorforum.com/forum/golf-simulator-brands-and-types/general-and-unlisted/419461-muni-golf-sim-software). The same post notes you can assign your own hotkeys. The H = flyover binding was also mentioned in a [GSV first look](https://golfsimulatorvideos.com/2026/10/free-home-golf-simulator-software-muni-golf-first-look-review/). If you have remapped keys in Muni, edit the matching Simulate Input commands. Other reported defaults not on the pad: **Tab** aim/normal camera, **Space** hold to speed flight, **T** tee-move mode, **.** / **,** next / previous player, **Enter** skip flyover, **F11** fullscreen.

### Sim Tuner — from the repo, Neon Links and Driving Range

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Strip (display) |
|---|---|---|---|---|---|---|
| Row 1 | Mulligan **M** | Flyover **F** | — | — | — | **Club** (live) |
| Row 2 | Aim Left **←** | Tilt Up **↑** | Tilt Down **↓** | Aim Right **→** | — | **Distance** (live) |
| Row 3 | — | — | — | — | **HOME** | **Last Shot** (live) |

Verified in `web/static/js/features/simgolf/SimGolfGame.js` (Neon Links) and `DrivingRange.js` at [github.com/alecweinstein/SimTuner](https://github.com/alecweinstein/SimTuner). Details:

- **M** mulligan and **F** flyover (tee, before the first stroke; any key cancels an in-progress flyover) are Neon Links only.
- Arrow left/right aim on both Neon Links and the Driving Range (2-degree nudges).
- Arrow up/down tilt the camera on Neon Links only. On the range they do nothing.
- These are `window` `keydown` listeners, not OS-level hotkeys. **The Sim Tuner window must be focused.** There is no `RegisterHotKey` (or equivalent) in the Go host.
- There is a local HTTP API used by the app's own UI (swing trigger, popouts, training sessions, preferences), but it is not a documented public control API and is not wired to these keys.
- `Ctrl+Shift+P` toggles a developer Perf HUD; it is not on the pad.

---


### 5.1 Live side strip (GSPro and Sim Tuner)

The Ajazz side strip is **display-only**. A small OpenDeck plugin shipped in this pack, `com.simtuner.golfdeck.strip`, paints three tiles there:

| Position | Action | What it shows | Color |
|---|---|---|---|
| 5 (top) | Club | Short club label (`7i`, `DR`, `PW`, …) | Green |
| 11 (middle) | Distance | Yards to target, e.g. `168 yd` | Blue |
| 17 (bottom) | Last Shot | Ball speed in mph, with carry underneath (`168c`, or `154c est` when the carry is an estimate) | Orange / gold |

The plugin is a single compiled Go binary (no Node). OpenDeck launches it like any other native plugin (`CodePathWin: golfdeck-strip.exe`). It talks to Sim Tuner on the local machine:

- `GET http://127.0.0.1:8080/api/deck-status` — JSON `{club, clubName, handed, distanceToTarget, ballSpeedMph, carryYards, carrySource, shotCount, gsproConnected, updatedAt}`
- WebSocket `ws://127.0.0.1:8080/ws`, event `deckStatus` (envelope `{type, data}` — parsing is tolerant of `event`/`payload` aliases)

It subscribes to the WebSocket with auto-reconnect, and falls back to polling the GET every 1 second when the socket is down. Images are pushed with `setImage` **only when values change**. The payload is a JPEG data URI (the AKP153 hardware plugin rejects anything that is not `image/jpeg`; OpenDeck does not convert keypad images). Tiles are drawn at 95 px (85 px variants are also generated) in the same card style as the rest of the pack. If Sim Tuner is unreachable, the tiles go dim and say **Sim Tuner offline**.

This data contract is being added to Sim Tuner itself; until that is running, the strip will stay on the offline state. You can exercise the plugin without Sim Tuner with the mock server:

```bash
python3 plugin/mock_server.py --scenario cycle
# or: --scenario normal | estimate | empty | offline
```

Override the Sim Tuner base URL with the `GOLFDECK_SIMTUNER` environment variable if you need to (default `http://127.0.0.1:8080`). Plugin sources, a README, and a build script are in `plugin/`. Preview tiles (normal, estimate carry, offline) are in `icons/previews/strip/`, including the contact sheet `strip-states.png`.

PurePlay and Muni strips are intentionally left empty — those games have no equivalent live feed in this pack.

## 6. Icons

PNG icons, same art, three sizes:

- `icons/95x95/` — Rev. 2 main-key native size (used inside the profile JSON).
- `icons/85x85/` — original AKP153E native size.
- `icons/72x72/` — classic Stream Deck size, if you want to reuse them elsewhere.
- `icons/previews/` — full-pad mockups and a contact sheet.
- `icons/previews/strip/` — live-tile states (normal, estimate carry, offline) and `strip-states.png`.

Style is a dark, color-tinted rounded card with a matching cap, a white glyph, and a short label — built to stay readable on the 85–95 px LCD. Colors are consistent by function across every profile:

- **Green** — club and shot (club up/down, putter, tee left/right, fast forward, skip) **and the live Club strip tile**
- **Blue** — aim (arrows, reset aim, aim point, fly to aim) **and the live Distance strip tile**
- **Orange** — camera (flyover, fly to ball, first person, free cam, replay, preview, tilt)
- **Purple** — map, green reading, and info (scorecard, heat map, grid, slope, range finder, HUD, trails, shortcuts overlay)
- **Red** — mulligan
- **Gold** — home, back, and more; also the estimated-carry line on Last Shot
- **Orange / gold** — the live Last Shot strip tile (ball speed orange, estimate carry in gold)
- **Gray** — utility (mute, fullscreen, switch hand)
- Golf Home mode keys keep each sim's own color: gold GSPro, cyan PurePlay, green Muni, lime Sim Tuner.

You can also drop any of these PNGs onto a key in the OpenDeck UI.

---

## 7. Open questions

1. **Which AKP153E revision** do you have? VID:PID `0300:1010` (v1, folder `99-355499441494-153E`) or `0300:3010` (Rev. 2, folder `99-<serial>`)? Both use the same 3×6 plugin grid.
2. Please confirm **Fly To Ball** is **B**, not **8**, by tapping that key in PurePlay. If it is 8, say so and we can flip the command.
3. ~~Does the side LCD strip register presses?~~ **Confirmed: it does not.** It is now a display-only live-data strip on GSPro, GSPro 2, and Sim Tuner (Club, Distance, Last Shot). PurePlay and Muni leave it empty. Club Up, Club Down, and MORE stay on the main 15 keys.
4. Have you **remapped Muni's keys**, or are you still on the defaults?
5. For auto-switch: after launching each game with OpenDeck open, what **application names** appear in OpenDeck's list?
6. Do you want **Run Command** keys that launch each game? If so, what are the `.exe` paths on your PC?
7. When you use Sim Tuner as a companion beside GSPro, the Sim Tuner keys will not fire unless its window is focused. The new local deck-status API is for the *display* strip (club / distance / last shot) and does not type keys into Sim Tuner. A small local *control* API would still be the clean fix if you want Sim Tuner shortcuts to fire while GSPro is focused.

The full shortcut tables, with sources and an unverified/verified flag on every entry, are in `SHORTCUTS.md`.
