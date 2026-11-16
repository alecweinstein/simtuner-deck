# Sim Deck logo

`simdeck-logo.png` is the app mark (340×338 PNG): the green lightning-bolt tile with a small 3×2 keypad badge in the bottom-right corner. The pixels outside that tile are a dark gray backdrop. `simtuner-logo.png` is the Sim Tuner mark without the badge and is not used for Sim Deck icons.

`icon-square.png` is the icon master made from `simdeck-logo.png`. The gray backdrop is removed so the corners outside the rounded tile are transparent, and a few percent of transparent padding is added so the tile is not clipped by icon masks. Every app icon, the tray icon (the window icon), and the NSIS installer icon, header, and sidebar are drawn from this master.

The master is about 368 pixels on a side. 16, 32, 48, 128, and 256 pixel icons are Lanczos downscales. 512 and 1024 pixel images, including `src-tauri/icons/icon.png` and the large `.icns` entries, are Lanczos upscales. The source artwork is only 340 pixels, so those larger sizes are soft.
