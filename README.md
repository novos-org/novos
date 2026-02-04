<div align="center">

# novos
<img src="./assets/icon.png" alt="novos logo" width="250" height="250">

<small> Build at the speed of thought.</small>

</div>

> [!WARNING]
> This is currently beta software.


## Features
- **Sass transpilation** via native `grass` (no C++ or Node.js required)
- **Fast Parallelism** utilizing `Rayon` for multi-core page generation
- **Live Reloading** with non-blocking `notify` event monitoring
- **Self-Contained** binary with embedded assets via `rust-embed`
- **Shortcodes, Includes, & Variables** for flexible templating
- **RSS** generation baked-in

## Supported OSes

| Tier | Operating System | Notes |
| :--- | :--- | :--- |
| **Tier 1** | **OmniOS CE** | Primary development & CI target |
| **Tier 1** | **FreeBSD** | Primary development & CI target |
| **Tier 2** | Ubuntu LTS (Noble) | Verified compatibility |
| **Tier 2** | Arch Linux | Verified compatibility |
| **Tier 2** | Void Linux | Verified compatibility |

## Backend
- **Language:** Rust (2024 Edition)
- **Markdown:** `pulldown-cmark` (CommonMark compliant, yay!)
- **License:** 3-Clause BSD
