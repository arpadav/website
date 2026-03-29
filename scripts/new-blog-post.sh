#!/usr/bin/env bash
set -euo pipefail

# --------------------------------------------------
# Creates a new blog post with today's date
# Usage: ./new-blog-post.sh
# --------------------------------------------------
SCRIPTS_DIR="$(dirname $(realpath "${BASH_SOURCE[0]}"))"
ROOT_DIR="$(realpath "$SCRIPTS_DIR/..")"
BLOG_DIR="$ROOT_DIR/content/blog"

# --------------------------------------------------
# prompt for title
# --------------------------------------------------
read -p "Blog post title: " TITLE

if [ -z "$TITLE" ]; then
    echo "Error: title cannot be empty"
    exit 1
fi

# --------------------------------------------------
# generate date prefix (YYYYMMDD)
# --------------------------------------------------
DATE_PREFIX=$(date +%Y%m%d-%H%M)

# --------------------------------------------------
# slugify title: lowercase, replace spaces/special
# chars with hyphens, collapse multiple hyphens
# --------------------------------------------------
SLUG=$(echo "$TITLE" \
    | tr '[:upper:]' '[:lower:]' \
    | sed 's/[^a-z0-9 -]//g' \
    | sed 's/ \+/-/g' \
    | sed 's/-\+/-/g' \
    | sed 's/^-//;s/-$//')

# --------------------------------------------------
# create folder + index.md
# --------------------------------------------------
POST_DIR="$(realpath -m "$BLOG_DIR/${DATE_PREFIX}-${SLUG}")"
mkdir -p "$POST_DIR"
POST_FILE="$POST_DIR/index.md"
if [ -f "$POST_FILE" ]; then
    echo "Error: $POST_FILE already exists"
    exit 1
fi
echo "# $TITLE" > "$POST_FILE"
echo "" >> "$POST_FILE"
echo "Created: $POST_FILE"
