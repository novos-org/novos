bin_name := "novos"
version  := `grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2`
target_os := `uname -s | tr '[:upper:]' '[:lower:]'`
target_arch := `uname -m`

default:
    @just --list

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
dist:
    mkdir -p dist
    echo "Packaging {{bin_name}} v{{version}} for {{target_os}}..."
    NAME="{{bin_name}}-{{target_arch}}-{{target_os}}.tar.xz"; \
    tar -cJvf dist/$NAME -C target/release {{bin_name}}; \
    cd dist && \
        sha256sum $NAME > $NAME.sha256 && \
        sha512sum $NAME > $NAME.sha512 && \
        b3sum      $NAME > $NAME.b3sum && \
        sha1sum    $NAME > $NAME.sha1

clean:
    cargo clean
    rm -rf dist