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


---

## Awesome Golf (insights.awesome-golf.com) — official

Windows only. There are **no documented club up/down, tee-move, or hold-to-fast-forward keys**. Left and Right arrows aim in Virtual Golf, Approach Shots, Closest to the Pin, and Target Golf; in Shot-by-Shot and Coaching the same keys go to the previous or next shot. Mulligan (**M**) is Virtual Golf only. **Space** stops the auto-countdown; it is a tap, not a hold. "Any key" skip-replay / skip-fly-in is not bound, because it would fire whatever key we picked.

Source: [In-Game Key Mappings & Hotkeys](https://insights.awesome-golf.com/docs/reference/key-mappings/keyboard)

### Page 1 — `Awesome Golf.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Mulligan (Virtual Golf only) | M | Page 1 | `[k(uni('m'),c)]` | Official |
| Replay | R | Page 1 | `[k(uni('r'),c)]` | Official |
| Switch Player | S | Page 1 | `[k(uni('s'),c)]` | Official |
| Cycle Camera | C | Page 1 | `[k(uni('c'),c)]` | Official (Shot-by-Shot / Club Distances) |
| Cycle View | V | Page 1 | `[k(uni('v'),c)]` | Official (Shot-by-Shot Overview / Detail) |
| Aim Left / Right | ← / → | Page 1 (hold) | `[k(LeftArrow,c)]` / `[k(RightArrow,c)]` | Official |
| Cycle Firmness | F | Page 1 | `[k(uni('f'),c)]` | Official (Approach / Shot-by-Shot) |
| Stop Auto-Countdown | Space | Page 1 (click) | `[k(Space,c)]` | Official |
| Toggle Audio | A | Page 1 | `[k(uni('a'),c)]` | Official |
| Dismiss | Esc | Page 1 | `[k(Escape,c)]` | Official (also dismisses Switch Player) |
| Continue | Enter | Page 1 | `[k(Return,c)]` | Official (Dismiss/Continue; also Match Status Continue) |
| Skip Replay / Skip Hole Fly-in | Any key | No | — | Official; not bound |
| Previous / Next Shot | ← / → | No (same keys as aim) | — | Official in Shot-by-Shot and Coaching |
| Re-take / Delete Shot | Delete | No | `[k(Delete,c)]` | Official in Club Distances / Coaching; not bound |

One page is enough, so this profile has no page 2. Golf Home is on the bottom-right key.

---

## TGC 2019 (The Golf Club) — official

There is **no direct mulligan key**. **Rewind Shot** (take a mulligan) is an item in the pause menu (**Esc** or **M**). **R** toggles Auto-Rewind, which the official guide describes as auto-mulligan and useful for practice. There are no documented tee-move or hold-to-fast-forward keys. **J** is a second flyover binding and is omitted so it does not duplicate **H**.

Quick club select (**`** driver, **1**–**9**, **0**, **-**, **=**, **P**) and **Alt+Enter** (windowed mode) are documented on 247.golf's help article, not in the ProTee user-guide table. **Putter** is on page 1 from that list; only **Driver** was added as a quick-select on page 2. The other numbered clubs are documented but not bound.

Sources:

- [TGC2019 Sim User Guide — ProTee Customer Support](https://csc.protee-united.com/hc/en-us/articles/360015650600-TGC2019-Sim-User-Guide)
- [TGC 2019 Keyboard Shortcuts — 247.golf Help](https://help.247.golf/en-US/tgc-2019-keyboard-shortcuts-774064)

### Page 1 — `TGC 2019.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Club Down | Z | Page 1 | `[k(uni('z'),c)]` | Official (also moves the ball in drop mode) |
| Club Up | X | Page 1 | `[k(uni('x'),c)]` | Official (also moves the ball in drop mode) |
| Putter | P | Page 1 | `[k(uni('p'),c)]` | 247.golf |
| Flyover | H | Page 1 | `[k(uni('h'),c)]` | Official (**J** is the same flyover, not bound) |
| Scout Camera | Q | Page 1 | `[k(uni('q'),c)]` | Official |
| Aim Left / Right | ← / → (also A / D) | Page 1 (hold) | `[k(LeftArrow,c)]` / `[k(RightArrow,c)]` | Official |
| Aim Forward / Back | ↑ / ↓ (also W / S while putting) | Page 1 (hold) | `[k(UpArrow,c)]` / `[k(DownArrow,c)]` | Official, putting |
| Aerial View | E | Page 1 | `[k(uni('e'),c)]` | Official (also places the ball in practice) |
| Auto-Rewind | R | Page 1 | `[k(uni('r'),c)]` | Official |
| Green Grid | G | Page 1 | `[k(uni('g'),c)]` | Official (putting section; 247.golf also lists it generally) |
| Shot Information | I | Page 1 | `[k(uni('i'),c)]` | Official |
| Menu / Pause | Esc (also M) | Page 1 | `[k(Escape,c)]` | Official; use this to reach Rewind Shot |

### Page 2 — `TGC 2019 2.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Hole Data | C | Page 2 | `[k(uni('c'),c)]` | Official |
| Lie Grid | L | Page 2 | `[k(uni('l'),c)]` | Official |
| Remove Flag | F | Page 2 | `[k(uni('f'),c)]` | Official (off the green only) |
| Cycle View | V | Page 2 | `[k(uni('v'),c)]` | Official |
| Driver | ` (backtick) | Page 2 | `[k(uni('`'),c)]` | 247.golf |
| Move Ball Forward / Back | W / S | Page 2 | `[k(uni('w'),c)]` / `[k(uni('s'),c)]` | Official, drop / unplayable mode (same keys also aim the putting pole) |
| Drop / Advance | Enter | Page 2 | `[k(Return,c)]` | Official (drop; 247.golf also lists advance when holed out) |
| Windowed Mode | Alt+Enter | Page 2 | `[k(alt,p),k(Return,c),k(alt,r)]` | 247.golf |

---

## ProTee Play — official

This is ProTee's own older sim, **not** Beacon Games PurePlay. There are **no documented mulligan, club up/down cycle, tee-move, up/down aim, or fast-forward keys**. Club changes are by family or by a specific club key. **D** toggles driver and woods; **W** cycles lob / sand / pitching wedge; **H** toggles hybrids.

Source: [ProTee Play Keyboard Shortcuts](https://csc.protee-united.com/hc/en-us/articles/360000056133-ProTee-Play-Keyboard-Shortcuts)

### Page 1 — `ProTee Play.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Putter | P | Page 1 | `[k(uni('p'),c)]` | Official |
| Toggle Driver / Woods | D | Page 1 | `[k(uni('d'),c)]` | Official |
| Toggle Hybrids | H | Page 1 | `[k(uni('h'),c)]` | Official |
| Cycle Wedges | W | Page 1 | `[k(uni('w'),c)]` | Official (lob / sand / pitching) |
| Sand Wedge | S | Page 1 | `[k(uni('s'),c)]` | Official |
| Aim Left / Right | ← / → | Page 1 (hold) | `[k(LeftArrow,c)]` / `[k(RightArrow,c)]` | Official |
| Pitching Wedge | 0 | Page 1 | `[k(uni('0'),c)]` | Official |
| Clear Tracers | C | Page 1 | `[k(uni('c'),c)]` | Official |
| Last Shot Data | I | Page 1 | `[k(uni('i'),c)]` | Official |

### Page 2 — `ProTee Play 2.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Iron 1–9 | 1–9 | Page 2 | `[k(uni('1'),c)]` … `[k(uni('9'),c)]` | Official |

---

## Creative Golf 3D / Creative Golf Advanced — official

The current Creative Golf Advanced user manual (v3.2.005, 2026) documents three keyboard shortcuts: **F** show flag, **M** mulligan, **X** skip player. Left and Right arrows for view rotation appear in the official 2019 Foresight/Datacrea v2.5 manual, when Game Option 2 "View direction handling" is set to the directional-arrow icon. Later manuals (SkyTrak v2.8, 2021, and the 2026 Advanced manual's aiming chapter) describe aiming as click or touch and no longer mention those arrows, so they may depend on version or that older setting. There are **no documented club up/down, tee-move, or fast-forward keys**. Replay, scorecard, hole view, and drop are menu / on-screen icons, not published key bindings.

Sources:

- [Creative Golf Advanced / Golfisimo User Manual v3.2.005](https://www.creativegolf3d.com/setups/manual/Creative%20Golf%20Advanced%20Manual.pdf) (SHORTCUTS section: F, M, X)
- [Creative Golf 3D and Golfisimo User Manual v2.5 for Foresight GC2, November 2019](https://20033507.fs1.hubspotusercontent-na1.net/hubfs/20033507/Product%20Documents/Software/Creative%20Golf%203D%20manual%20Foresight%20Sports%20GC2.pdf) (left/right arrows for rotation)

### Page 1 — `Creative Golf.json`

| Action | Key | On the pad? | Simulate Input | Status |
|---|---|---|---|---|
| Mulligan | M | Page 1 | `[k(uni('m'),c)]` | Official, 2026 Advanced manual |
| Show Flag | F | Page 1 | `[k(uni('f'),c)]` | Official, 2026 Advanced manual |
| Skip Player | X | Page 1 | `[k(uni('x'),c)]` | Official, 2026 Advanced manual |
| Aim / Rotate Left / Right | ← / → | Page 1 (hold) | `[k(LeftArrow,c)]` / `[k(RightArrow,c)]` | Official in the 2019 v2.5 manual; confirm on your build |

One page is enough, so this profile has no page 2.

---

## Sims researched but not added

None of these have a published in-round keyboard map that is complete and current enough to bind without inventing keys.

| Sim | Why it was skipped | Sources |
|---|---|---|
| Foresight FSX Play | The official Play manual describes a clickable Shortcuts Bar (Bird's Eye, Mulligan, Grid, and so on) but does **not** list key assignments. A Keyboard Shortcuts window was added in [FSX Play v1.1.0.6](https://support.foresightsports.com/support/software/fsx-play); forum users say **K** opens it and that it looks like the FSX 2020 list, but also that several keys were non-operational and that Mulligan was missing in some Play builds. | [FSX Play User Manual](https://support.foresightsports.com/fsx-play-user-manual); [FSXPLAY Keyboard Shortcuts thread](https://golfsimulatorforum.com/forum/foresight-sports/367866-fsxplay-keyboard-shortcuts); [Practice / shortcuts thread](https://golfsimulatorforum.com/forum/foresight-sports/373608-does-fsx-play-contain-a-practice-button-shortcuts) |
| Foresight FSX 2020 | Official manual documents **I** for Analysis and arrow keys to move a drop, and the rest of the in-round actions as Shortcuts Bar clicks. Forum reports say **Down Arrow** became Mulligan in a September 2020 release, which already differs from the older 2018 list. Binding the 2018 keys on 2020/Play would be guesswork. | [FSX 2020 User Manual](https://support.foresightsports.com/fsx-2020-user-manual); [FSX: Keyboard Shortcuts thread](https://golfsimulatorforum.com/forum/foresight-sports/288634-fsx-keyboard-shortcuts) |
| Foresight FSX 2018 | An official hot-key PDF does exist (**E** bird's eye, **F** flag, **G** grid, **H** hole flyby, **I** analysis, **R** replays, **C** club, **K** shortcuts, and others), but this is the older product, not what most people run now, so it was not added as a profile. | [FSX 2018 Keyboard Shortcuts PDF](https://support.foresightsports.com/sites/default/files/downloads/2021/FSX%202018%20Keyboard%20Shortcuts.pdf) |
| Foresight FSX Pro Studio | Coaching / fitting software. The official manual is mouse-driven (click club groups, tiles, and toolbar). No gameplay keyboard map. | [FSX Pro Studio User Manual](https://support.foresightsports.com/fsx-pro-studio-user-manual) |
| E6 Connect | Official manual describes mouse and touch (club button, mini-map aim, on-screen Mulligan). No official keyboard-shortcut list. Third-party device guides sometimes mention **C** / **V**, but those are not from E6. | [E6 Connect Manual](https://connect.e6golf.com/e6-connect/manual/) |
| Uneekor VIEW / REFINE | Official help and manuals describe on-screen modes and display settings, not keys. A 2020 forum post said older Refine showed hotkeys on screen, and linked a 2019 QED in-game manual that is now a dead 404, with no archived copy found. | [Uneekor software overview](https://support.uneekor.com/knowledge/what-uneekor-software-are-available); [Uneekor Ignite and Refine hot keys thread](https://golfsimulatorforum.com/forum/golf-simulator-brands-and-types/uneekor-qed/278753-uneekor-ignite-and-refine-hot-keys) |
| Trackman Virtual Golf (VG2 / VG3 in TPS) | Official mulligan help is click-only (player-score dropdown in VG3, on-screen button in VG2). Aim and club select are screen / mini-map clicks. The official TPS keyboard-shortcut article is for navigating TPS without a mouse, not in-round VG play, and the key list is an image that was not transcribed into the article text. | [Mulligans in VG2 and VG3](https://support.trackmangolf.com/hc/en-us/articles/30834967923612-Courses-Mulligans-in-VG2-and-VG3); [TPS keyboard shortcuts](https://support.trackmangolf.com/hc/en-us/articles/37008938200731-TPS-List-Of-Keyboard-Shortcuts-In-TPS) |
| Golfzon | Commercial bay software (Golfzon Real / Vision) has some keys in Korean or Japanese materials (**F2** flyover, **F12** cancel shot). Consumer Golfzon WAVE's English support only documents left/right arrows to change distance in Approach Practice and Putting Arena, which is not a full play map. | [Golfzon WAVE support](https://golfzonwave.co.uk/pages/suppport); [Golfzon Real shortcuts (Korean)](https://story.golfzon.com/65) |
| Full Swing Kit / Studio | Official user guides describe a MENU tab and on-screen Replay / Mulligan / Camera / Top View controls. No keyboard-shortcut list was found. | [Full Swing user guides](https://fullswinggolf.my.site.com/support/s/userguides) |

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

## FSX Play
Source: FSX Play in-game Keyboard Shortcuts screen (photo from Alec, Nov 17, 2026).
- Page 1: Bird's Eye E, Flag F, Grid G, Hole Flyby H, Periscope Up (hold) | Previous Hole `,`, Next Hole `.`, Replays R, Analysis I, Hole Review Q | Club menu C, Ball menu B, Practice P, Exit to Menu Esc, MORE
- Page 2: Course Flyby Alt+H, Drop Balls Alt+G, View menu V, Dashboard M, Shortcuts O | On-Screen Data N, Graphics S, Swing Cam W, Dual Display Z, Devices D | Keyboard Shortcuts K, Language U, HOME, BACK
- No mulligan, club up/down, aim, tee or fast-forward keys are listed.

## Infinite Tees
Source: Alec's hotkey list (Nov 17, 2026); aim arrows from RoomCaddie docs (needs IT Side Screen on). Hotkeys are remappable in IT Settings.
- Row 1: Aim Left (hold ←), Aim Point F3, Flyover F4, Heat Map Y, Aim Right (hold →)
- Row 2: FOV 1–4 (Ctrl+1..4)
- Row 3: Eye Height 1–4 (Alt+1..4), HOME
