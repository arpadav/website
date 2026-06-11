#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------
# Adds a timestamped URL to the lol CSV
# Usage: ./scripts/new-link.sh [url]
# --------------------------------------------------
SCRIPTS_DIR="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
ROOT_DIR="$(realpath "$SCRIPTS_DIR/..")"
LOL_FILE="$ROOT_DIR/content/other/lol.csv"
TITLE_BYTES=65536

csv_escape() {
    local value="$1"
    value="${value//$'\r'/ }"
    value="${value//$'\n'/ }"
    value="${value//\"/\"\"}"
    printf '"%s"' "$value"
}

fetch_title() {
    local url="$1"
    local html=""
    if command -v curl >/dev/null 2>&1; then
        html="$(
            curl \
                --location \
                --silent \
                --show-error \
                --fail \
                --compressed \
                --connect-timeout 5 \
                --max-time 10 \
                --range "0-$((TITLE_BYTES - 1))" \
                --user-agent "arpadvoros.website title fetcher" \
                "$url" 2>/dev/null | head -c "$TITLE_BYTES" || true
        )"
    elif command -v wget >/dev/null 2>&1; then
        html="$(
            wget \
                -qO- \
                --timeout=10 \
                --tries=1 \
                --header="Range: bytes=0-$((TITLE_BYTES - 1))" \
                --user-agent="arpadvoros.website title fetcher" \
                "$url" 2>/dev/null | head -c "$TITLE_BYTES" || true
        )"
    fi
    printf '%s' "$html" | perl -0777 -CS -ne '
        if (m{<title\b[^>]*>(.*?)</title>}is) {
            $t = $1;
            $t =~ s/<[^>]+>//g;
            $t =~ s/\s+/ /g;
            $t =~ s/^\s+|\s+$//g;
            $t =~ s/&#x([0-9a-f]+);/chr(hex($1))/gei;
            $t =~ s/&#([0-9]+);/chr($1)/ge;
            $t =~ s/&quot;/"/gi;
            $t =~ s/&#39;/\x27/gi;
            $t =~ s/&lt;/</gi;
            $t =~ s/&gt;/>/gi;
            $t =~ s/&amp;/&/gi;
            print $t;
        }
    '
}

ensure_lol_file() {
    mkdir -p "$(dirname "$LOL_FILE")"
    if [ ! -f "$LOL_FILE" ]; then
        printf 'date,url,title\n' >"$LOL_FILE"
        return
    fi

    local header=""
    IFS= read -r header <"$LOL_FILE" || true
    case "$header" in
        "date,url,title") ;;
        "date,url")
            local tmp
            tmp="$(mktemp)"
            {
                printf 'date,url,title\n'
                tail -n +2 "$LOL_FILE" | awk 'length($0) > 0 { print $0 ",\"\"" }'
            } >"$tmp"
            mv "$tmp" "$LOL_FILE"
            ;;
        *)
            echo "Error: unexpected lol CSV header: $header" >&2
            exit 1
            ;;
    esac
}
ensure_lol_file

URL="${1:-}"
if [ -z "$URL" ]; then
    read -r -p "URL: " URL
fi
if [ -z "$URL" ]; then
    echo "Error: URL cannot be empty"
    exit 1
fi

TITLE="${2:$(fetch_title "$URL")}"
if [ -n "$TITLE" ]; then
    echo "Title: $TITLE"
else
    echo "Title: not found"
fi

DATE="$(date +"%Y-%m-%d %H:%M:%S %z")"
{
    csv_escape "$DATE"
    printf ','
    csv_escape "$URL"
    printf ','
    csv_escape "$TITLE"
    printf '\n'
} >>"$LOL_FILE"

echo "Added link to $LOL_FILE"
