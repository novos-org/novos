<h1>
<p align="center">
novos
</p>
</h1>

<p align="center"> Build at the speed of thought.</p>

> [!WARNING]
> This is currently beta software.

# Features
- **Sass transpilation**
- **Shortcodes**
- **Includes**
- **RSS generation**
- **Variables**
- **Templates**

# Supported OSes

| Tier | Operating System | Notes |
| :--- | :--- | :--- |
| **Tier 1** | **OmniOS CE** | Primary development & CI target |
| **Tier 1** | **FreeBSD** | Primary development & CI target |
| **Tier 2** | Ubuntu LTS (Noble) | Verified compatibility |
| **Tier 2** | Arch Linux | Verified compatibility |
| **Tier 2** | Void Linux | Verified compatibility |

# Engine
- **Core:** Rust (2024 Edition)
- **Parallelism:** Work-stealing scheduling via `Rayon`
- **Styling:** Native `grass` compiler (No Ruby/Node dependency)
- **IO:** Non-blocking event monitoring via `notify`

# License

novos is free software. Released under the 3-Clause BSD license.