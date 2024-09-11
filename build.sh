#!/bin/bash

# Load environment variables from .env file
export $(grep -v '^#' .env | xargs)

# Default description
DESCRIPTION=""

# Parse optional argument for description
while getopts m: flag
do
    case "${flag}" in
        m) DESCRIPTION=${OPTARG};;
    esac
done

# Clean out release/ directory if it exists, otherwise create it
if [ -d "release" ]; then
    rm -rf release/*
else
    mkdir release
fi

# Run the Tauri build
cargo tauri build

# Extract version from src-tauri/tauri.conf.json
VERSION=$(grep -Po '"version": *"\K[^"]*' src-tauri/tauri.conf.json)

# Get the signature from the generated file
SIGNATURE_FILE="src-tauri/target/release/bundle/nsis/stargazer_${VERSION}_x64-setup.exe.sig"
if [ -f "$SIGNATURE_FILE" ]; then
    SIGNATURE=$(cat "$SIGNATURE_FILE")
else
    echo "Error: Signature file not found at $SIGNATURE_FILE"
    exit 1
fi

# Generate the latest.json file and place it in release/
cat <<EOF > release/latest.json
{
    "version": "$VERSION",
    "notes": "$DESCRIPTION",
    "platforms": {
        "windows-x86_64": {
            "signature": "$SIGNATURE",
            "url": "https://github.com/henrymbaldwin/orion_api_stargazer/releases/latest/download/stargazer_${VERSION}_x64-setup.exe"
        }
    }
}
EOF

# Copy the generated executable to release/
EXE_FILE="src-tauri/target/release/bundle/nsis/stargazer_${VERSION}_x64-setup.exe"
if [ -f "$EXE_FILE" ]; then
    cp "$EXE_FILE" release/
else
    echo "Error: Executable file not found at $EXE_FILE"
    exit 1
fi

echo "Build complete, latest.json and executable copied to release/"
