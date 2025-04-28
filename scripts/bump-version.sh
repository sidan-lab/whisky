#!/bin/bash

# Path to Cargo.toml
CARGO_TOML="packages/Cargo.toml"
WHISKY_COMMON_CARGO_TOML="packages/whisky-common/Cargo.toml"
WHISKY_CSL_CARGO_TOML="packages/whisky-csl/Cargo.toml"
WHISKY_PALLAS_CARGO_TOML="packages/whisky-pallas/Cargo.toml"
WHISKY_JS_CARGO_TOML="packages/whisky-js/Cargo.toml"
WHISKY_PROVIDER_CARGO_TOML="packages/whisky-provider/Cargo.toml"
WHISKY_WALLET_CARGO_TOML="packages/whisky-wallet/Cargo.toml"
WHISKY_CARGO_TOML="packages/whisky/Cargo.toml"
EXAMPLES_CARGO_TOML="packages/whisky-examples/Cargo.toml"

# Extract the current main version
current_version=$(grep '^version = ' "$CARGO_TOML" | head -1 | sed 's/version = "\(.*\)"/\1/')

# Function to increment the patch version
increment_patch_version() {
  IFS='.' read -r major minor patch <<< "$1"
  new_patch=$((patch + 1))
  echo "$major.$minor.$new_patch"
}

# Determine the new version
if [ -z "$1" ]; then
  new_version=$(increment_patch_version "$current_version")
else
  new_version="$1"
fi

# Update the version in workspace Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$CARGO_TOML"

# Update the version in whisky-common Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_COMMON_CARGO_TOML"

# Update the version in whisky-csl Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CSL_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_CSL_CARGO_TOML"

# Update the version in whisky-pallas Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_PALLAS_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_PALLAS_CARGO_TOML"

# Update the version in whisky-js Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_JS_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_JS_CARGO_TOML"
sed -i '' "s/whisky-csl = { version = \"=$current_version\"/whisky-csl = { version = \"=$new_version\"/" "$WHISKY_JS_CARGO_TOML"
sed -i '' "s/whisky-pallas = { version = \"=$current_version\"/whisky-pallas = { version = \"=$new_version\"/" "$WHISKY_JS_CARGO_TOML"

# Update the version in whisky-provider Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_PROVIDER_CARGO_TOML"
sed -i '' "s/whisky-csl = { version = \"=$current_version\"/whisky-csl = { version = \"=$new_version\"/" "$WHISKY_PROVIDER_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_PROVIDER_CARGO_TOML"

# Update the version in whisky-wallet Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_WALLET_CARGO_TOML"
sed -i '' "s/whisky-csl = { version = \"=$current_version\"/whisky-csl = { version = \"=$new_version\"/" "$WHISKY_WALLET_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_WALLET_CARGO_TOML"

# Update the version in whisky Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-csl = { version = \"=$current_version\"/whisky-csl = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-pallas = { version = \"=$current_version\"/whisky-pallas = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-wallet = { version = \"=$current_version\"/whisky-wallet = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-provider = { version = \"=$current_version\"/whisky-provider = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-common = { version = \"=$current_version\"/whisky-common = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"

# Update the version in examples Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$EXAMPLES_CARGO_TOML"
sed -i '' "s/whisky = { version = \"=$current_version\"/whisky = { version = \"=$new_version\"/" "$EXAMPLES_CARGO_TOML"

echo "Version bumped to $new_version for all cargo.toml"