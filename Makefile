.PHONY: build release clean github-release install man

# Target binary name
BINARY_NAME=cgip

# Release directory
RELEASE_DIR=./release

PLATFORM=$(shell uname -s | tr '[:upper:]' '[:lower:]')
ARCH ?= $(shell uname -m | tr '[:upper:]' '[:lower:]')

build:
	@echo "Building the application..."
	@if [ "$(TARGET)" = "" ]; then \
		cargo build --release; \
	else \
		cargo build --release --target $(TARGET); \
	fi

clean:
	@echo "Cleaning up..."
	@cargo clean
	@rm -rf $(RELEASE_DIR)

install:
	cargo build --release
	cp target/release/cgip /usr/local/bin/
	cp docs/cgip.1 /usr/local/share/man/man1/

man:
	groff -man -Tascii docs/cgip.1 | less

deb:
	@sh ./scripts/build_deb.sh

deb-publish:
	@sh ./scripts/build_deb.sh "publish"

tarball: build
	@echo "Packaging the release..."
	@mkdir -p $(RELEASE_DIR)
	@if [ "$(TARGET)" = "" ]; then \
		cp docs/cgip.1 target/release/; \
		tar -czf $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz -C target/release $(BINARY_NAME) cgip.1; \
	else \
		cp docs/cgip.1 target/$(TARGET)/release/; \
		tar -czf $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz -C target/$(TARGET)/release $(BINARY_NAME) cgip.1; \
	fi
	@echo "Release package created: $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz"


tarball-publish: tarball
	@TARBALL=$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz; \
	echo "Sending tarball $$TARBALL to script"; \
	sh ./scripts/publish_asset.sh $$TARBALL

