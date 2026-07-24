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
    cd "$SCRIPTS_DIR/../tinyklv-0.1.2/book"
    mdbook build
    cp -r book "$DEPLOY_FOLDER/tinyklv-0.1.2"
    cd -
} >/dev/null 2>&1
