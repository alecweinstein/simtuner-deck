## GSPro — official, all verified

The side strip (positions 5, 11, 17) is display-only: Club, Distance, and Last Shot, painted live by the Golf Deck Strip plugin from Sim Tuner. Those keys have no shortcuts and do not register presses.

Source: [GSPro Knowledge Base — Keyboard Shortcuts](https://gspro.gitbook.io/gspro-knowledge-base/in-game-information/keyboard-shortcuts) (also summarized on the in-game **Tab** overlay). Shortcuts work on a course or in practice. Mulligan requires mulligans enabled for the round. Keys 8, 9, and 0 are On Course Practice only. Aim Point (**F3**) is on page 1 at the end of the aim row, next to the aim arrows — that placement is a pack-wide rule. Range Finder and other extras are on **GSPro 2**.

| Action | Key | On the pad? | Simulate Input (`down`) | Status |
|---|---|---|---|---|
| Mulligan | Ctrl+M | Yes | `[k(ctrl,p),k(uni('m'),c),k(ctrl,r)]` | Official |
| Club Up | I | Page 1 (row 1) | `[k(uni('i'),c)]` | Official |
| Club Down | K | Page 1 (row 1) | `[k(uni('k'),c)]` | Official |
| Putter | U | Yes | `[k(uni('u'),c)]` | Official |
| Flyover | O | Yes | `[k(uni('o'),c)]` | Official |
| Aim Left | Left Arrow | Yes | `[k(LeftArrow,c)]` | Official |
| Aim Forward | Up Arrow | Yes | `[k(UpArrow,c)]` | Official |
| Aim Back | Down Arrow | Yes | `[k(DownArrow,c)]` | Official |
| Aim Right | Right Arrow | Yes | `[k(RightArrow,c)]` | Official |
| Reset Aim | A | Page 2 | `[k(uni('a'),c)]` | Official |
| Scorecard | T | Page 2 | `[k(uni('t'),c)]` | Official |
| Heatmap | Y | Page 1 | `[k(uni('y'),c)]` | Official |
| Green Grid | G | Page 2 | `[k(uni('g'),c)]` | Official |
| Range Finder | R | Page 2 | `[k(uni('r'),c)]` | Official |
| Fast-forward ball flight | Hold Space | Page 1 (hold) | down `[k(Space,p)]` / up `[k(Space,r)]` | Official |
| Free Look | F5 | Page 2 | `[k(F5,c)]` | Official |
| Aim Point (cycle) | F3 | Page 1 (end of aim row) | `[k(F3,c)]` | Official |
| Camera Go To Ball | 5 | No | `[k(uni('5'),c)]` | Official |
| Camera Fly to Ball | 6 | Page 2 | `[k(uni('6'),c)]` | Official |
| Tee Left / Right | C / V | Page 1 (bottom-left, hold) | `[k(uni('c'),c)]` / `[k(uni('v'),c)]` | Official |
| Shortcut overlay | Tab | Page 2 | `[k(Tab,c)]` | Official |
| Hide UI | H | Page 2 | `[k(uni('h'),c)]` | Official |
| Show UI | Esc | No | `[k(Escape,c)]` | Official |
| Clear Tracer | F1 | Page 2 | `[k(F1,c)]` | Official |
| Fullscreen | F11 | Page 2 | `[k(F11,c)]` | Official |
| Pin Indicator | P | Page 2 | `[k(uni('p'),c)]` | Official |
| Shot Cam | J | Page 2 | `[k(uni('j'),c)]` | Official |
| Switch Hand | N | Page 2 | `[k(uni('n'),c)]` | Official |
| Map Zoom In / Out | Q / W | No | | Official |
| Map Expand | S | No | | Official |
| Vertical Dots | D | No | | Official |
| Lighting | L | No | | Official |
| 3D Grass toggle | Z | No | | Official |
| Camera Clip | B | No | | Official |
| Sound Off / On | - / + | No | | Official |
| Console Short / Tall | F8 / F9 | No | | Official |
| FPS / Vsync cycle | F | No | | Official |
| Shadow Less / More | < / > | No | | Official |
| Go To Ball (practice) | 8 | No | | Official, On Course Practice only |
| Previous / Next Hole (practice) | 9 / 0 | No | | Official, On Course Practice only |

---

## PurePlay (Beacon Games, pureplaygolf.com) — official in-game overlay

Identified as Beacon Games Pty Ltd's **Pureplay** (also written PurePlay or Pure Play). Windows only. This is **not** ProTee Play.

Source: PurePlay's own in-game Keyboard Overlay, from a screenshot Alec sent on 12 Nov 2026. The website FAQ still does not publish a list. An earlier [GSV closed-beta first look](https://golfsimulatorvideos.com/2026/10/pureplay-golf-simulator-software-first-look/) reported 5 / 6 for camera height; those keys are **not** on the official overlay and are **not bound**.

**Fly To Ball** is bound as **B**, the best reading of "Camera Fly To Ball" on the overlay. It may be **8** — the two can look similar, and the original image file was not available to re-inspect at pack-rebuild time. If the key does nothing, change it to `[k(uni('8'),c)]`.

**Toggle Weather Settings** is omitted: the key in the screenshot was unreadable.

### Page 1 — `PurePlay.json`

| Action | Official label | Key | On the pad? | Simulate Input (`down`) | Status |
|---|---|---|---|---|---|
| Mulligan | Player Mulligan | Ctrl+M | Page 1 | `[k(ctrl,p),k(uni('m'),c),k(ctrl,r)]` | Official overlay |
| Club Up | Club Select Up | I | Page 1 | `[k(uni('i'),c)]` | Official overlay |
| Club Down | Club Select Down | K | Page 1 | `[k(uni('k'),c)]` | Official overlay |
| Putter | Select Putter | U | Page 1 | `[k(uni('u'),c)]` | Official overlay |
| Flyover | Camera Flyover | O | Page 1 | `[k(uni('o'),c)]` | Official overlay |
| Aim Left / Right / Forward / Back | Move | Arrows | Page 1 | `[k(LeftArrow,c)]` and Up / Down / Right | Official overlay |
| Aim Point | Aim Point | F3 | Page 1 | `[k(F3,c)]` | Official overlay |
| Green Grid | Map Green Grid | G | Page 1 | `[k(uni('g'),c)]` | Official overlay |
| Fly To Ball | Camera Fly To Ball | **B** (possibly 8) | Page 2 | `[k(uni('b'),c)]` | Official overlay; key reading noted above |
| Tee Left | Club Tee Left | C | Page 1 (row 1) | `[k(uni('c'),c)]` | Official overlay |
| Tee Right | Club Tee Right | V | Page 1 (row 1) | `[k(uni('v'),c)]` | Official overlay |

### Page 2 — `PurePlay 2.json`

Tee Left / Tee Right are on page 1 only (they used to be duplicated here; those slots now hold Heat Map and Reset Aim). Scorecard stays on this page.


| Action | Official label | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|---|
| Heat Map | Map Heat Map | Y | Page 2 | `[k(uni('y'),c)]` | Official overlay |
| Aim Reset | Aim Reset | A | Page 2 | `[k(uni('a'),c)]` | Official overlay |
| Skip | Player Skip | F7 | Page 2 | `[k(F7,c)]` | Official overlay |
| Switch Hand | Player Switch Hand | N | Page 2 | `[k(uni('n'),c)]` | Official overlay |
| First Person | Toggle First Person | F6 | Page 2 | `[k(F6,c)]` | Official overlay |
| Scorecard | Score Card | T | Page 2 | `[k(uni('t'),c)]` | Official overlay |
| Fly To Aim | Fly to Aim Point | J | Page 2 | `[k(uni('j'),c)]` | Official overlay |
| Fast Forward | Ball Fast Forward | Hold Space | Page 1 | down `[k(Space,p)]` / up `[k(Space,r)]` | Official overlay |
| HUD | Toggle HUD | H | Page 2 | `[k(uni('h'),c)]` | Official overlay |
| Ball Trails | Toggle Ball Trails | F1 | Page 2 | `[k(F1,c)]` | Official overlay |
| Shortcuts overlay | Keyboard Overlay | Tab | Page 2 | `[k(Tab,c)]` | Official overlay |
| Mute All | Mute All | Num- | Page 2 | `[k(Subtract,c)]` (enigo `Subtract` = VK_SUBTRACT, numpad minus) | Official overlay |
| Fullscreen | Toggle Fullscreen | F11 | Page 2 | `[k(F11,c)]` | Official overlay |
| Push to Talk | Push to Talk | X | No | `[k(uni('x'),c)]` | Official overlay; not bound (easy to add) |
| Toggle FPS | Toggle FPS | F | No | `[k(uni('f'),c)]` | Official overlay; not bound |
| Toggle Weather Settings | Toggle Weather Settings | unreadable | No | — | Official overlay; key unreadable, skipped |
| Camera height up / down | — | 5 / 6 | No | — | Earlier GSV beta report only; **not** on the official overlay, not bound |

## Muni Golf (munigolfsim.com) — community-reported defaults

Muni lets you assign your own hotkeys. The list below is what a forum user posted as "all the keyboard bindings" on 13 October 2026. Treat them as **defaults**, and change the profile if you have remapped them. H = flyover was independently mentioned in a GSV review.

Source: [Bubba22, Golf Simulator Forum #419461](https://golfsimulatorforum.com/forum/golf-simulator-brands-and-types/general-and-unlisted/419461-muni-golf-sim-software). Supporting mention of H: [GSV Muni first look](https://golfsimulatorvideos.com/2026/10/free-home-golf-simulator-software-muni-golf-first-look-review/). No official shortcut page was found on munigolfsim.com.

| Action | Key | On the pad? | Simulate Input (`down`) | Status |
|---|---|---|---|---|
| Mulligan (if allowed) | M | Yes | `[k(uni('m'),c)]` | Community default |
| Club Up | I | Yes | `[k(uni('i'),c)]` | Community default |
| Club Down | K | Yes | `[k(uni('k'),c)]` | Community default |
| Shot Preview | P | Yes | `[k(uni('p'),c)]` | Community default |
| Flyover | H | Yes | `[k(uni('h'),c)]` | Community default; also mentioned by GSV |
| Aim Left / Right | ← / → | Yes | `[k(LeftArrow,c)]` / `[k(RightArrow,c)]` | Community default |
| Aim Further / Shorter | ↑ / ↓ | Yes | `[k(UpArrow,c)]` / `[k(DownArrow,c)]` | Community default |
| Reset Aim | R | Yes | `[k(uni('r'),c)]` | Community default |
| Scorecard | S | Yes | `[k(uni('s'),c)]` | Community default |
| Green slope heatmap | G | Yes | `[k(uni('g'),c)]` | Community default |
| Replay | V | Yes | `[k(uni('v'),c)]` | Community default |
| Free camera | F | Yes | `[k(uni('f'),c)]` | Community default |
| Aim / normal camera | Tab | No | `[k(Tab,c)]` | Community default |
| Speed ball flight / flyover | Hold Space | No | down `[k(Space,p)]` / up `[k(Space,r)]` | Community default |
| Skip flyover | Enter | No | `[k(Return,c)]` (confirm on your PC; enigo may want `Return`) | Community default, command not tested |
| Tee move mode | T | No | `[k(uni('t'),c)]` | Community default |
| Next / previous player | . / , | No | `[k(uni('.'),c)]` / `[k(uni(','),c)]` | Community default |
| Fullscreen | F11 | No | `[k(F11,c)]` | Community default |

---

## Sim Tuner — verified from source

The side strip is the same live Club / Distance / Last Shot display as on GSPro. It is not a shortcut and does not register presses.

Repo: [github.com/alecweinstein/SimTuner](https://github.com/alecweinstein/SimTuner), read-only. There is no shortcut section in `docs/user-guide.md`. Bindings are `keydown` listeners in the frontend. They only fire when the Sim Tuner window is focused. The Go host has no global hotkey registration.

### Neon Links — `web/static/js/features/simgolf/SimGolfGame.js`

| Action | Key | On the pad? | Notes | Status |
|---|---|---|---|---|
| Aim left / right | ← / → | Yes | `_adjustAim(±2)` | Verified in source |
| Tilt camera up / down | ↑ / ↓ | Yes | `_adjustTilt(∓5)` | Verified in source |
| Flyover | F | Yes | Tee, before the first stroke. Any key (or a click) cancels it. | Verified in source |
| Mulligan | M | Yes | Counts on the scorecard. | Verified in source |

### Driving Range — `web/static/js/features/simgolf/DrivingRange.js`

| Action | Key | On the pad? | Notes | Status |
|---|---|---|---|---|
| Aim left / right | ← / → | Yes | Same keys as Neon Links. Ignored if the range editor or session panel is open, or if focus is in an input. | Verified in source |

### Other keys in the repo, not for play, not on the pad

| Action | Key | Where | Status |
|---|---|---|---|
| Toggle Perf HUD | Ctrl+Shift+P | `PerfHUD.js` | Verified; developer overlay |
| Close / cancel | Escape | Several modals, Swing Replay dock, range editor | Verified; UI only |
| Undo | Ctrl/Cmd+Z | Range editor | Verified; editor only |
| Nudge green / resize | Arrows, +, - | Range editor map, when that canvas is focused | Verified; editor only |
| Enter / Escape | confirm / cancel | License gate, name prompts | Verified; UI only |

### Local API

The desktop app serves a local HTTP API that its own UI uses (`/api/swing/trigger`, `/api/popout`, `/api/training/sessions`, `/api/preferences`, and others). This is **not** a documented public control surface for a Stream Deck, and these profiles do not call it.

---

## Built-in OpenDeck actions used

From the Starter Pack manifest (`plugins/com.amansprojects.starterpack.sdPlugin/assets/manifest.json`) and OpenDeck itself (`src-tauri/src/shared.rs`):

| Action | UUID | Used? |
|---|---|---|
| Simulate Input | `com.amansprojects.starterpack.inputsimulation` | Yes — every game key |
| Switch Profile | `com.amansprojects.starterpack.switchprofile` | Yes — Home and mode keys |
| Run Command | `com.amansprojects.starterpack.runcommand` | Available; not pre-filled (needs a path) |
| Open URL | `com.amansprojects.starterpack.openurl` | Available; not used |
| Device Brightness | `com.amansprojects.starterpack.devicebrightness` | Available; not used |
| Multi Action (built-in) | `opendeck.multiaction` | Available; not used |
| Toggle Action (built-in) | `opendeck.toggleaction` | Available; not used |

There is no built-in "folder" action. Profiles plus Switch Profile are the documented substitute ([OpenDeck FAQ](https://github.com/nekename/OpenDeck/wiki/0.-FAQ)).
