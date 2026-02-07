<div align="center">

# **novos**
###### ( aka 'novos omits very old standards' )
<img src="./assets/icon.png" alt="novos logo" width="250" height="250">

**Build at the speed of thought.**


![License](https://img.shields.io/github/license/novos-org/novos)
![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)
</div>

> [!WARNING]
> **novos** is currently beta software. Expect frequent updates and potential breaking changes as we approach v1.0.
> 
> **We are currently changing to Tera. For the meantime, the default template partially works. Same with the official website**

## What is novos?
novos is a lightning-fast static site generator (SSG) built in Rust. It takes your Markdown files and turns them into a website in milliseconds.

## Features
- **Sass transpilation** via native `grass` (no C++ or Node.js required)
- **Fast Parallelism** utilizing `Rayon` for multi-core page generation
- **Self-Contained** binary with embedded assets via `rust-embed`
- **Shortcodes, Includes, & Variables** for flexible templating
- **RSS** generation baked-in
- **search.json**, so you can search
- **syntect**, because who hates colors?
- **minification + WebP-ification**, because you want speed, right?
- **Axum**, for a stellar webserver + WebSockets.
- **Themes**, for a wide variety of presets.
- [**Tera**](https://keats.github.io/tera), because who loves repeating themselves?

## Supported OSes

| Tier | Operating System | Notes |
| :--- | :--- | :--- |
| **Tier 1** | **OmniOS CE** | Primary development |
| **Tier 1** | **FreeBSD** | Primary development  |
| **Tier 2** | Ubuntu LTS (Noble) | Verified compatibility |
| **Tier 2** | Arch Linux | Verified compatibility |
| **Tier 2** | Void Linux | Verified compatibility |
| **Tier 3** | macOS (intel/Sillicon) | *"CI compiled it"* |
| **Tier 3** | Windows | *"Wine said it worked"* |

## Roadmap
- [X] Parallel page generation
- [X] Live reload inside dev server
- [ ] Theme: Nord-ish
- [ ] Plugin API (WASM + .so)
- [ ] Taxonomies
- [ ] Pagination

## Themes
- Solarnight (themes/solarnight)
  - Solarnight is a Rose Pine inspired theme. 

## Documentation
Documentation is available at [docs/](./docs)
- [**Get Started**](./docs/SETUP.md): Setup guide for novos
- [**Installation**](./docs/INSTALL.md): Install guide for novos

## Backend
- **Language:** Rust (2024 Edition)
- **Markdown:** `pulldown-cmark` (CommonMark compliant, yay!)
- **License:** 3-Clause BSD



*Commits prior to 2d1acf1be5fb605694cc2f95c5efe8dad0b35de0 are three clause BSD. Commits after that point are Apache-2.0 OR MIT*
