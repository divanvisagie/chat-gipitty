.PHONY: build release clean

# Target binary name
BINARY_NAME=cgip

# Release directory
RELEASE_DIR=./release

build:
	@echo "Building the application..."
	@cargo build --release

release: build
	@echo "Packaging the release..."
	@mkdir -p $(RELEASE_DIR)
	# Copy docs/cgip.1 into directory before compression
	@cp docs/cgip.1 $(RELEASE_DIR)
	@tar -czf $(RELEASE_DIR)/$(BINARY_NAME)-macos.tar.gz -C target/release $(BINARY_NAME)

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
