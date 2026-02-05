# Getting Started
First off, you need to install novos. You can look at the [installation guide](./docs/INSTALL.md) for refence.

# Setup
After you've installed novos, it's time to make a site
To make a site, use `novos init`.
Setup process:
```sh
mkdir site && cd site && novos init
# you can also clone the novos documentation site: git clone https://github.com/novos-org/website.git
# and, you might want to clone xer's site for a blog example: git clone https://github.com/xerrkk/srclo.net.git
```

The default directory structure is
```text
├── novos.toml # config
├── pages # static HTML pages (e.x., blog.html, privacy.html)
├── sass # SASS stylesheets
├── posts # markup posts (e.x., docs, blog, etc)
├── static  # static content (e.x., javascript, images)
└── includes # templates, shortcodes, partials
```
