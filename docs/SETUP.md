---
title: "Getting Started"
date: 2026-02-06
tags: setup, install, beginner
---
# Getting Started
First off, you need to install novos. You can look at the [installation guide](./docs/INSTALL.md) for reference.

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
Site Title (new novos site):
Author Name (admin):
Do you want to enable Sass compilation? [Y/n]:
Do you want to enable syntax highlighting? [Y/n]:
Do you want to build a search index? [Y/n]:
Do you want to generate an RSS feed? [Y/n]:
Clean output directory before build? [Y/n]:
Minify HTML output? [y/N]:
```

# Learning by Example
If you prefer to start with a working template, you can clone the official documentation or a live production site:
- Official Website: `git clone https://github.com/novos-org/website.git`

# Directory Structure
A novos project is designed to be flat and transparent. No hidden magic—just files where you expect them:

| File/Folder | Purpose |
|---|---|
| novos.toml | The brain of your site. Configure your title, URL, and RSS settings here. |
| posts/ |  Where your Markdown lives. These are processed by pulldown-cmark. |
|  sass/ |  `.scss` files. Compiled natively via grass without needing Node.js. |
| static/ | Pass-through assets like images, fonts, and client-side JavaScript.|
| templates/ | Reusable snippets, shortcodes, and templating logic. These are processed by `Tera`. |

# Development Workflow
Once your site is initialized, you can use the built-in development server to preview changes in real-time:
```sh
# Build the site and start a local server
novos serve
```
