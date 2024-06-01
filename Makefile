.PHONY: build release clean github-release install man

# Target binary name
BINARY_NAME=cgip

# Release directory
RELEASE_DIR=./release

PLATFORM=$(shell uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(shell uname -m | tr '[:upper:]' '[:lower:]')

# GitHub variables
GITHUB_OWNER=your-username
GITHUB_REPO=your-repo

build:
	@echo "Building the application..."
	@cargo build --release

release: build
	@echo "Packaging the release..."
	@mkdir -p $(RELEASE_DIR)
	# Copy docs/cgip.1 into directory before compression
	@cp docs/cgip.1 target/release/
	@tar -czf $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz -C target/release $(BINARY_NAME) cgip.1

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

mac-release: release
	@echo "Creating or updating GitHub release..."
	# Get the last tag name
	@TAG_NAME=$$(git describe --tags --abbrev=0 2>/dev/null || echo "test-release") && \
	echo "Using tag: $$TAG_NAME" && \
	gh release view $$TAG_NAME >/dev/null 2>&1 && \
	{ echo "Release with tag $$TAG_NAME already exists. Uploading assets..."; \
	  gh release upload $$TAG_NAME $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz --clobber; } || \
	{ echo "Release with tag $$TAG_NAME does not exist. Creating release and uploading assets..."; \
	  gh release create $$TAG_NAME $(RELEASE_DIR)/$(BINARY_NAME)-$(PLATFORM)-$(ARCH).tar.gz --title "Release $$TAG_NAME" --notes "Auto-generated release"; }
