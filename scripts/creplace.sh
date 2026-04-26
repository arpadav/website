#!/bin/bash
# Author: aav
# --------------------------------------------------
# Description:
#   This script finds all "fancy" single and double
# quotes in all .rs files, and replaces them with
# standard ones, in addition to some other characters
# like long dashes. This is useful for ensuring consistency
#   The main usage is if you are copy-and-pasting
# documentation or comments into a file, these "fancy"
# quotes could exist and cause inconsistencies. This
# script is mainly for aesthetics, and is not imperative
# --------------------------------------------------
# Usage:
#   `bash scripts/creplace.sh`
# --------------------------------------------------
set -euo pipefail

# --------------------------------------------------
# resolve this script's absolute path - this is to
# exclude the same file
# --------------------------------------------------
SCRIPT_PATH="$(realpath "${BASH_SOURCE[0]}")"

# --------------------------------------------------
# file extensions to search for
# --------------------------------------------------
FILE_EXTENSIONS=("rs" "sh" "html" "md" "cu" "js" "css" "md")
name_expr=()
for ext in "${FILE_EXTENSIONS[@]}"; do
    if [ ${#name_expr[@]} -gt 0 ]; then
        name_expr+=(-o)
    fi
    name_expr+=(-name "*.${ext}")
done

# --------------------------------------------------
# directories to exclude
# --------------------------------------------------
EXCLUDE_DIRS=("vendor" "target" ".git")
prune_expr=()
for dir in "${EXCLUDE_DIRS[@]}"; do
    [ ${#prune_expr[@]} -gt 0 ] && prune_expr+=(-o)
    prune_expr+=(-type d -name "$dir")
done

# --------------------------------------------------
# * find file extensions in FILE_EXTENSIONS, recursively
# * exclude `vendor/target` directories with a `Cargo.toml`
# * perform the replacements using `sed`
# --------------------------------------------------
LC_ALL=en_US.UTF-8 find . \
    \( "${prune_expr[@]}" \) -prune -o \
    -type f \( "${name_expr[@]}" \) \
    ! -path "*/vendor/*" ! -path "*/target/*" \
    ! -samefile "$SCRIPT_PATH" \
    -exec sed -i \
    -e "s/[’‘]/'/g" \
    -e 's/[“”]/"/g' \
    -e $'s/\xE2\x80\x94/-/g' \
    -e 's/→/->/g' \
    {} +
