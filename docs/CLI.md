# Command Line Interface
novos has various commands. Examples include: "build", "init", and "serve".

`serve` starts a local development server, whilst `build`... well, builds the site.

`init` is unique in a way, you run it once. It scaffolds a entire project, inculding SASS, JavaScript. templates, and includues.

# novos.toml
`novos.toml` is where you configure you site.

`cat novos.toml`

```toml
base_url = "https://novos.srclo.net"
base = ""

posts_dir   = "./posts"
pages_dir   = "./pages"
static_dir  = "./static"
output_dir  = "./.build"
posts_outdir = "doc"

template_path      = "./index.html"
includes_dir       = "./includes"
view_template_path = "./includes/view_template.html"

[site]
title = "novos"
description = "A fast, minimal static site generator."
author = "novos project"
generate_rss = true
generate_search = true

[build]
# If true, wipes the .build folder every time (recommended for prod)
clean_output = true

# Options: "expanded" or "compressed"
sass_style = "compressed"

```