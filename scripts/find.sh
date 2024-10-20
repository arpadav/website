#!/bin/bash

# --------------------------------------------------
# Finds a string, printing exact location(s) to console
# for quick Ctrl+Click navigation in VSCode (and other
# supported editors)
# --------------------------------------------------
# Usage: `bash find.sh <string-to-find>`
# --------------------------------------------------
if [ -z "$1" ]; then
    echo "Usage: bash $0 <string-to-find>"
    exit 1
fi
search_string="$1"
# --------------------------------------------------
# omit the following directories:
# --------------------------------------------------
OMIT=(
    "scripts"
    "deploy"
    "target"
)
EXCLUDE_DIRS=("${OMIT[@]/#/--exclude-dir=}")
# --------------------------------------------------
# magic
# --------------------------------------------------
echo "Searching for: $search_string in `pwd`"
grep -rn -F "$search_string" . "${EXCLUDE_DIRS[@]}" | while IFS=: read -r file line content; do
    col=$(grep -bo "$search_string" "$file" | grep -m 1 -Eo '^[0-9]+' | head -1)
    if [[ -n $col ]]; then
        col=$((col + 1))
    fi
    echo "$file:$line:$col"
done
