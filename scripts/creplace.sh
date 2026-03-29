#!/usr/bin/env bash
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
# Usage: `bash scripts/creplace.sh`
# --------------------------------------------------
set -euo pipefail

# --------------------------------------------------
# file extensions to search for
# --------------------------------------------------
FILE_EXTENSIONS=("rs" "sh" "html" "md" "cu")
name_expr=()
for ext in "${FILE_EXTENSIONS[@]}"; do
    if [ ${#name_expr[@]} -gt 0 ]; then
        name_expr+=(-o)
    fi
    name_expr+=(-name "*.${ext}")
done

# --------------------------------------------------
# * find file extensions in FILE_EXTENSIONS, recursively
# * exclude `vendor/target` directories with a `Cargo.toml`
# * perform the replacements using `sed`
# --------------------------------------------------
LC_ALL=en_US.UTF-8 find . -type f \( "${name_expr[@]}" \) \
    ! -path "*/vendor/*" ! -path "*/target/*" \
    -exec sed -i \
    -e "s/'/'/g" \
    -e "s/'/'/g" \
    -e 's/[""]/"/g' \
    -e $'s/\xE2\x80\x94/-/g' \
    {} +
