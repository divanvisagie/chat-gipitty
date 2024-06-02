#!/bin/bash
# Get the latest release and populate a Homebrew formula with the release
# details

RELEASE_NAME=$(gh release view --json tagName | jq .tagName)
echo "Release name: $RELEASE_NAME"

# strip double quotes form release name
RELEASE_NAME=$(echo $RELEASE_NAME | tr -d '"')

gh release download $RELEASE_NAME -D release/assets

# ls release/assets | grep darwin
# cgip-darwin-aarch64.tar.gz
# cgip-darwin-x86_64.tar.gz
# shasum -a 256 yourfile.tar.gz
INTEL_HASH=$(shasum -a 256 release/assets/cgip-darwin-x86_64.tar.gz | awk '{print $1}')
ARM_HASH=$(shasum -a 256 release/assets/cgip-darwin-aarch64.tar.gz | awk '{print $1}')

cat scripts/cgip.rb | sed "s/{{tag}}/$RELEASE_NAME/g" \
	| sed "s/{{intel_hash}}/$INTEL_HASH/g" \
	| sed "s/{{arm_hash}}/$ARM_HASH/g" > ../homebrew-tap/cgip.rb
