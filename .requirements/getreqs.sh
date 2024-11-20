#!/bin/bash

REQUIREMENTS_DIR="$(dirname "$(realpath "$0")")"

# --------------------------------------------------
# get requirements from `apt-requirements.txt`
# install any that are missing
# --------------------------------------------------
mapfile -t requirements < "$REQUIREMENTS_DIR/apt-requirements.txt"
missing=()
for pkg in "${requirements[@]}"; do
    # --------------------------------------------------
    # edge cases: apt install name != binary name
    # --------------------------------------------------
    if [ "$pkg" == "node-typescript" ]; then
        if ! type "tsc" >/dev/null 2>&1; then
            missing+=("$pkg")
        fi
        continue
    fi
    # --------------------------------------------------
    # check if package exists
    # --------------------------------------------------
    if ! type "$pkg" &> /dev/null; then
        # --------------------------------------------------
        # edge cases: sudo apt install isnt sufficient
        # --------------------------------------------------
        if [ "$pkg" == "pandoc" ]; then
            arch=$(uname -m)
            if [ "$arch" == "x86_64" ]; then
                pkgname="pandoc-3.5-1-amd64.deb"
            else
                pkgname="pandoc-3.5-1-arm64.deb"
            fi 
            wget "https://github.com/jgm/pandoc/releases/download/3.5/$pkgname"
            sudo dpkg -i "$pkgname"
            rm "$pkgname"
            continue
        fi
        # --------------------------------------------------
        # otherwise, add as missing
        # --------------------------------------------------
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
        echo "\nAdd `cargo` to path given the instructions above and re-run this script to continue."
    else
        echo "Failed to install Rust. Please install Rust: \`https://www.rust-lang.org/tools/install\`"
    fi
    exit
fi

# --------------------------------------------------
# get `minhtml`, if doesnt exist
# --------------------------------------------------
if ! type minhtml >/dev/null 2>&1; then
    echo "\`minhtml\` not found, attempting to install..."
    cargo install minhtml
fi