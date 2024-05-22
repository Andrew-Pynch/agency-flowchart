#!/bin/bash

# Build the release version
cargo build --release

# Get the target directory
target_dir="target/release"

# Get the executable name
executable_name="agency-flowchart"

# Determine the platform
platform=$(uname)

# Set the executable path based on the platform
if [[ "$platform" == "Linux" || "$platform" == "Darwin" ]]; then
    executable_path="$target_dir/$executable_name"
elif [[ "$platform" == MINGW* ]]; then
    executable_path="$target_dir/$executable_name.exe"
else
    echo "Unsupported platform: $platform"
    exit 1
fi

# Copy the executable to /usr/local/bin
sudo cp "$executable_path" "/usr/local/bin"

echo "Executable copied to /usr/local/bin"
