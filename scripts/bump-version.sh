#!/bin/bash

# Path to Cargo.toml
CARGO_TOML="packages/Cargo.toml"
WHISKY_CORE_CARGO_TOML="packages/whisky-core/Cargo.toml"
WHISKY_CARGO_TOML="packages/whisky/Cargo.toml"
WHISKY_CSL_CARGO_TOML="packages/whisky-csl/Cargo.toml"
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

# Update the version in whisky Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CARGO_TOML"
sed -i '' "s/whisky-core = { version = \"=$current_version\"/whisky-core = { version = \"=$new_version\"/" "$WHISKY_CARGO_TOML"

# Update the version in whisky-csl Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CSL_CARGO_TOML"

# Update the version in whisky-core Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$WHISKY_CORE_CARGO_TOML"

# Update the version in examples Cargo.toml
sed -i '' "s/version = \"$current_version\"/version = \"$new_version\"/" "$EXAMPLES_CARGO_TOML"
sed -i '' "s/whisky = { version = \"=$current_version\"/whisky = { version = \"=$new_version\"/" "$EXAMPLES_CARGO_TOML"

echo "Version bumped to $new_version for all cargo.toml"