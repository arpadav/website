#!/usr/bin/env bash
# --------------------------------------------------
# expects $DEPLOY_FOLDER to be set
# --------------------------------------------------
set -euo pipefail
# --------------------------------------------------
# cd to submodule/book, mdbook, and copy book to deploy folder
# --------------------------------------------------
{
    SCRIPTS_DIR="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
    cd "$SCRIPTS_DIR/../tinyklv/book"
    mdbook build
    cp -r book "$DEPLOY_FOLDER/tinyklv"
    cd -
} >/dev/null 2>&1
