#!/usr/bin/env bash
REQUIREMENTS_DIR="$(dirname "$(realpath "$0")")"
# --------------------------------------------------
# get requirements, install any that are missing
# --------------------------------------------------
mapfile -t apt_requirements <"$REQUIREMENTS_DIR/apt-requirements.txt"
mapfile -t cargo_requirements <"$REQUIREMENTS_DIR/cargo-requirements.txt"

missing=()
for pkg in "${apt_requirements[@]}"; do
    # --------------------------------------------------
    # check if package exists
    # --------------------------------------------------
    if ! type "$pkg" &>/dev/null; then
        # --------------------------------------------------
        # edge cases: sudo apt install isnt sufficient
        # --------------------------------------------------
        if [ "$pkg" == "pandoc" ]; then
            arch=$(uname -m)
            pandoc_version="3.9.0.2"
            if [ "$arch" == "x86_64" ]; then
                pkgname="pandoc-$pandoc_version-1-amd64.deb"
            else
                pkgname="pandoc-$pandoc_version-1-arm64.deb"
            fi
            wget "https://github.com/jgm/pandoc/releases/download/$pandoc_version/$pkgname"
            sudo dpkg -i "$pkgname"
            rm "$pkgname"
            continue
        fi
        if [ "$pkg" == "mold" ]; then
            echo "Please install the \`mold\` linker for quicker building times: https://github.com/rui314/mold"
            continue
        fi
        # --------------------------------------------------
        # otherwise, add as missing
        # --------------------------------------------------
        missing+=("$pkg")
    fi
done
if [ "${#missing[@]}" -gt 0 ]; then
    echo "Installing missing apt packages: ${missing[*]}"
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
        echo "\nAdd $(cargo) to path given the instructions above and re-run this script to continue."
    else
        echo "Failed to install Rust. Please install Rust: \`https://www.rust-lang.org/tools/install\`"
    fi
    exit
fi

missing=()
for pkg in "${cargo_requirements[@]}"; do
    # --------------------------------------------------
    # check if package exists
    # --------------------------------------------------
    # first, strip the package by taking everything before
    # the version indicator (i.e. the `@` symbol)
    # --------------------------------------------------
    stripped_pkg="${pkg%%@*}"
    if ! type "$stripped_pkg" &>/dev/null; then
        missing+=("$pkg")
    fi
done
if [ "${#missing[@]}" -gt 0 ]; then
    echo "Installing missing cargo packages: ${missing[*]}"
    cargo install "${missing[@]}"
fi
