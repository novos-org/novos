bin_name := "novos"
version  := `grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2`
target_os := `uname -s | tr '[:upper:]' '[:lower:]'`
target_arch := `uname -m`

default:
    @just --list

# Build with conditional flags for SunOS/illumos
build:
    #!/usr/bin/env bash
    if [ "$(uname -s)" = "SunOS" ]; then \
        export CFLAGS="-I/usr/include -D__EXTENSIONS__ -include alloca.h"; \
        export CPPFLAGS="-I/usr/include"; \
        export LDFLAGS="-L/usr/lib/64 -L/usr/gnu/lib"; \
    fi; \
    cargo build --release
    @echo "Stripping binary..."
    strip target/release/{{bin_name}}

# Package the binary for release
dist: build
    @mkdir -p dist
    @echo "Packaging {{bin_name}} v{{version}} for {{target_os}}..."
    tar -cJvf dist/{{bin_name}}-{{target_arch}}-{{target_os}}.tar.xz -C target/release {{bin_name}}
    cd dist && sha256sum {{bin_name}}-{{target_arch}}-{{target_os}}.tar.xz > {{bin_name}}-{{target_arch}}-{{target_os}}.tar.xz.sha256
    @echo "Done! Check the dist/ folder."

clean:
    cargo clean
    rm -rf dist