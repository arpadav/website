#!/usr/bin/env bash
# --------------------------------------------------
# expects $DEPLOY_FOLDER to be set
# --------------------------------------------------
set -euo pipefail

# --------------------------------------------------
# verify deploy folder name, know which version to
# build
# --------------------------------------------------
DEPLOY_FOLDER_NAME=$(basename "$DEPLOY_FOLDER")
CMD=""
if [ $DEPLOY_FOLDER_NAME == "dev" ]; then
    CMD="build"
elif [ $DEPLOY_FOLDER_NAME == "prod" ]; then
    CMD="build-prod"
fi
if [ -z "$CMD" ]; then
    echo "Unknown DEPLOY_FOLDER, expected 'dev' or 'prod', got '$DEPLOY_FOLDER_NAME' ($DEPLOY_FOLDER)"
    exit 1
fi
# --------------------------------------------------
# cd to submodule, build, and copy dist to deploy folder
# --------------------------------------------------
{
    SCRIPTS_DIR="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
    cd "$SCRIPTS_DIR/../font-designer"
    make $CMD
    cp -r web/dist "$DEPLOY_FOLDER/font-designer"
    cd -
} >/dev/null 2>&1
