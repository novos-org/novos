bin_name := "novos"

default:
    @just --list

# Build with conditional flags for SunOS headers
build:
    #!/usr/bin/env bash
    if [ "$(uname -s)" = "SunOS" ]; then \
        export CFLAGS="-I/usr/include -D__EXTENSIONS__ -include alloca.h"; \
        export CPPFLAGS="-I/usr/include"; \
    fi; \
    cargo build --release
    @echo "Stripping binary..."
    strip target/release/{{bin_name}}

# Clean artifacts
clean:
    cargo clean
