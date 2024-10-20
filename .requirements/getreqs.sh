#!/bin/bash

REQUIREMENTS_DIR="$(dirname "$(realpath "$0")")"

# --------------------------------------------------
# get requirements from `apt-requirements.txt`
# install any that are missing
# --------------------------------------------------
mapfile -t requirements < "$REQUIREMENTS_DIR/apt-requirements.txt"
missing=()
for pkg in "${requirements[@]}"; do
    if ! type "$pkg" &> /dev/null; then
        missing+=("$pkg")
    fi
done
if [ "${#missing[@]}" -gt 0 ]; then
    echo "Installing missing packages: ${missing[*]}"
    sudo apt install -y "${missing[@]}"
fi

# --------------------------------------------------
# check if `cargo` exists
# --------------------------------------------------
if ! type cargo >/dev/null 2>&1; then
    echo "\`cargo\` not found, attempting to install Rust..."
    CMD="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    if wget -qO- https://www.rust-lang.org/tools/install | grep -q "$CMD"; then
        eval "$CMD"
    else
        echo "Failed to install Rust. Please install Rust: \`https://www.rust-lang.org/tools/install\`"
        exit
    fi
fi

# --------------------------------------------------
# get `minhtml`, if doesnt exist
# --------------------------------------------------
if ! type minhtml >/dev/null 2>&1; then
    echo "\`minhtml\` not found, attempting to install..."
    cargo install minhtml
fi