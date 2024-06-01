# !/bin/env bash

# get the name passed into the script
PACKAGE_NAME=$1

echo "Creating or updating GitHub release..."
echo "Publishing $PACKAGE_NAME..."

# Get the latest tag name or use "test-release" if no tags exist
TAG_NAME=$(git describe --tags --abbrev=0 2>/dev/null || echo "test-release")
echo "Using tag: $TAG_NAME"

# Check if a release with the tag name already exists
if gh release view $TAG_NAME >/dev/null 2>&1; then
  echo "Release with tag $TAG_NAME already exists. Uploading assets..."
  gh release upload $TAG_NAME ./release/$PACKAGE_NAME --clobber
else
  echo "Release with tag $TAG_NAME does not exist. Creating release and uploading assets..."
  gh release create $TAG_NAME ./release/$PACKAGE_NAME --title "Release $TAG_NAME" --notes "Auto-generated release"
fi

