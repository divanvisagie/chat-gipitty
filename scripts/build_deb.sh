#!/bin/bash

# Exit on first error
set -e

# Variables
PACKAGE_NAME=cgip
VERSION=0.2.12
ARCHITECTURE=amd64
DEBIAN_FOLDER=debian_package
BUILD_FOLDER=$DEBIAN_FOLDER/$PACKAGE_NAME-$ARCHITECTURE-$VERSION

# Ensure cargo is available
command -v cargo >/dev/null 2>&1 || { echo >&2 "Cargo is required but it's not installed. Exiting."; exit 1; }

# Create Debian directory structure
echo "Creating Debian package structure..."
mkdir -p $BUILD_FOLDER/DEBIAN
mkdir -p $BUILD_FOLDER/usr/bin
mkdir -p $BUILD_FOLDER/usr/share/man/man1

# Control file
cat <<EOF >$BUILD_FOLDER/DEBIAN/control
Package: $PACKAGE_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCHITECTURE
Depends: libc6 (>= 2.28)
Maintainer: Divan Visagie <me@divanv.com>
Description: Terminal client for Chat GPT
 This is a simple terminal client to interact with Chat GPT models.
EOF

# Copy binary and man page
echo "Copying binary and man page..."
cp target/release/$PACKAGE_NAME $BUILD_FOLDER/usr/bin/
cp docs/$PACKAGE_NAME.1 $BUILD_FOLDER/usr/share/man/man1/

# Build the package
echo "Building DEB package..."
dpkg-deb --build $BUILD_FOLDER

# Move the DEB package to a more accessible location
mv $DEBIAN_FOLDER/$PACKAGE_NAME-$ARCHITECTURE-$VERSION.deb ./release/

# Cleanup
echo "Cleaning up..."
rm -rf $DEBIAN_FOLDER

echo "DEB package created: $PACKAGE_NAME-$VERSION.deb"

# if the first param is 'publish'
if [ "$1" = "publish" ]; then
  FILENAME="${PACKAGE_NAME}-${ARCHITECTURE}-${VERSION}.deb"
  echo "Calling script to publish $FILENAME"
  ./scripts/publish_asset.sh "$FILENAME"
fi
