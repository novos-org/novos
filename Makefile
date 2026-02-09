BIN_NAME=   novos
VERSION!=   grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2

.include "mk/os.mk"

DIST_DIR=   dist
RELEASE_BIN= target/release/${BIN_NAME}
PKG_NAME=   ${BIN_NAME}-${TARGET_ARCH}-${TARGET_OS}.tar.xz

.MAIN: help

help:
	@echo "Available targets: build, dist, clean"
	@echo "!WARNING: b3sum is required for making ${PKG_NAME}.b3sum."

build:
	cargo build --release
	@echo "Stripping binary..."
	strip ${RELEASE_BIN}

dist: build
	mkdir -p ${DIST_DIR}
	@echo "Packaging ${BIN_NAME} v${VERSION} for ${TARGET_OS}..."
	tar -cJvf ${DIST_DIR}/${PKG_NAME} -C target/release ${BIN_NAME}
	@cd ${DIST_DIR} && \
		sha256sum ${PKG_NAME} > ${PKG_NAME}.sha256 && \
		sha512sum ${PKG_NAME} > ${PKG_NAME}.sha512 && \
		b3sum     ${PKG_NAME} > ${PKG_NAME}.b3sum && \
		sha1sum   ${PKG_NAME} > ${PKG_NAME}.sha1

clean:
	cargo clean
	rm -rf ${DIST_DIR}

.PHONY: help build dist clean
