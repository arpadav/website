#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------
# Adds a timestamped idea to the ideas CSV
# Usage: ./scripts/new-idea.sh
# --------------------------------------------------
SCRIPTS_DIR="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
ROOT_DIR="$(realpath "$SCRIPTS_DIR/..")"
IDEAS_FILE="$ROOT_DIR/content/ideas.csv"

csv_escape() {
    local value="$1"
    value="${value//$'\r'/ }"
    value="${value//$'\n'/ }"
    value="${value//\"/\"\"}"
    printf '"%s"' "$value"
}

mkdir -p "$(dirname "$IDEAS_FILE")"
if [ ! -f "$IDEAS_FILE" ]; then
    printf 'date,name,description\n' >"$IDEAS_FILE"
fi

read -r -p "Idea name: " NAME
if [ -z "$NAME" ]; then
    echo "Error: idea name cannot be empty"
    exit 1
fi

read -r -p "Description: " DESCRIPTION
if [ -z "$DESCRIPTION" ]; then
    echo "Error: description cannot be empty"
    exit 1
fi

DATE="$(date +"%Y-%m-%d %H:%M:%S %z")"
{
    csv_escape "$DATE"
    printf ','
    csv_escape "$NAME"
    printf ','
    csv_escape "$DESCRIPTION"
    printf '\n'
} >>"$IDEAS_FILE"

echo "Added idea to $IDEAS_FILE"
