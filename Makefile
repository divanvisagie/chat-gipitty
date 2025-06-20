.PHONY: build release clean github-release install man tag-release tag-release-dry version help book

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

book:
	@echo "Building mdBook documentation..."
	@if command -v mdbook >/dev/null 2>&1; then \
		cd book && mdbook build --dest-dir ../docs; \
		echo "Documentation built and output to docs/"; \
	else \
		echo "Error: mdbook not found. Install with: cargo install mdbook"; \
		exit 1; \
	fi

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

version:
	@VERSION=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	echo "Current version: $$VERSION"; \
	echo "Would create tag: v$$VERSION"

tag-release-dry:
	@echo "DRY RUN: Checking GitHub CLI availability..."
	@if command -v gh >/dev/null 2>&1; then \
		echo "DRY RUN: GitHub CLI (gh) is available"; \
	else \
		echo "DRY RUN: Note: GitHub CLI (gh) not found - you can still create releases manually"; \
	fi
	@echo "DRY RUN: Checking git status..."
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "DRY RUN: Warning: Working directory is not clean"; \
	else \
		echo "DRY RUN: Working directory is clean"; \
	fi
	@echo "DRY RUN: Extracting version from Cargo.toml..."
	@VERSION=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	if [ -z "$$VERSION" ]; then \
		echo "DRY RUN: Error: Could not extract version from Cargo.toml"; \
	else \
		TAG="v$$VERSION"; \
		echo "DRY RUN: Checking if tag $$TAG already exists..."; \
		if git rev-parse "$$TAG" >/dev/null 2>&1; then \
			echo "DRY RUN: Warning: Tag $$TAG already exists"; \
		else \
			echo "DRY RUN: Tag $$TAG does not exist, safe to create"; \
		fi; \
		echo "DRY RUN: Would create and push tag: $$TAG"; \
		echo "DRY RUN: git tag $$TAG"; \
		echo "DRY RUN: git push origin $$TAG"; \
	fi

tag-release:
	@echo "Checking GitHub CLI availability..."
	@if command -v gh >/dev/null 2>&1; then \
		echo "GitHub CLI (gh) is available - you can create releases with 'gh release create'"; \
	else \
		echo "Note: GitHub CLI (gh) not found - you can install it or create releases manually"; \
	fi
	@echo "Checking git status..."
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "Error: Working directory is not clean. Please commit or stash changes first."; \
		exit 1; \
	fi
	@echo "Extracting version from Cargo.toml..."
	@VERSION=$$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'); \
	if [ -z "$$VERSION" ]; then \
		echo "Error: Could not extract version from Cargo.toml"; \
		exit 1; \
	fi; \
	TAG="v$$VERSION"; \
	echo "Checking if tag $$TAG already exists..."; \
	if git rev-parse "$$TAG" >/dev/null 2>&1; then \
		echo "Error: Tag $$TAG already exists"; \
		exit 1; \
	fi; \
	echo "Creating and pushing tag: $$TAG"; \
	git tag $$TAG; \
	git push origin $$TAG; \
	echo "Tag $$TAG created and pushed to trigger release build"

help:
	@echo "Available targets:"
	@echo "  build           - Build the application"
	@echo "  clean           - Clean up build artifacts"
	@echo "  install         - Build and install cgip to /usr/local/bin/"
	@echo "  man             - View the manual page"
	@echo "  book            - Build mdBook documentation to docs/ directory"
	@echo "  deb             - Build debian package"
	@echo "  deb-publish     - Build and publish debian package"
	@echo "  tarball         - Create release tarball"
	@echo "  tarball-publish - Create and publish release tarball"
	@echo "  version         - Show current version from Cargo.toml"
	@echo "  tag-release-dry - Show what tag-release would do (dry run)"
	@echo "  tag-release     - Extract version from Cargo.toml, create and push git tag to trigger release"
	@echo "  help            - Show this help message"
	@echo ""
	@echo "Release Workflow:"
	@echo "  1. Update version in Cargo.toml"
	@echo "  2. Run 'make tag-release-dry' to preview"
	@echo "  3. Run 'make tag-release' to create tag and trigger GitHub Actions release"
