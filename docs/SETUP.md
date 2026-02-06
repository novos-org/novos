# Getting Started
First off, you need to install novos. You can look at the [installation guide](./docs/INSTALL.md) for refence.

# Setup
After you've installed novos, it's time to make a site
To make a site, use `novos init`.
Setup process:
```sh
novos init my-website # defaults to . if no arg1
```
You will prompted a series of questions:

```text
What is the URL of your site? (https://example.com):
Site Title (novos example):
Author Name (Your Name):
Do you want to enable Sass compilation? [Y/n]:
Do you want to enable syntax highlighting? [y/N]:
Do you want to build a search index of the content? [Y/n]:
```

# Learning by Example
If you prefer to start with a working template, you can clone the official documentation or a live production site:
- Official Website: `git clone https://github.com/novos-org/website.git`

# Directory Structure
A novos project is designed to be flat and transparent. No hidden magic—just files where you expect them:

| File/Folder | Purpose |
|---|---|
| novos.toml | The brain of your site. Configure your title, URL, and RSS settings here. |
| pages/  | Static HTML files for high-level structure (e.g., `index.html`, `about.html`). |
| posts/ |  Where your Markdown lives. These are processed by pulldown-cmark. |
|  sass/ |  `.scss` files. Compiled natively via grass without needing Node.js. |
| static/ | Pass-through assets like images, fonts, and client-side JavaScript.|
| includes/ | Reusable snippets, shortcodes, and templating logic. |

# Development Workflow
Once your site is initialized, you can use the built-in development server to preview changes in real-time:
```sh
# Build the site and start a local server
novos serve
```
