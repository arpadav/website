#!/bin/bash

# --------------------------------------------------
# Usage: ./build.sh [prod]
# --------------------------------------------------
# defaults to dev
# --------------------------------------------------
ROOT_DIR="$(dirname "$(realpath "$0")")"
STATIC_DIR="$ROOT_DIR/static/"
if [ -z "$1" ]; then
    echo "Building dev version"
    FOLDER="$ROOT_DIR/deploy/dev"
elif [ "$1" == "prod" ]; then
    echo "Building prod version"
    FOLDER="$ROOT_DIR/deploy/prod"
else
    echo "Usage: ./build.sh [prod]"
    exit
fi

# --------------------------------------------------
# ensure all expected folders exists
# --------------------------------------------------
if [ ! -d "$FOLDER" ]; then
    echo "Folder-to-generate-to does not exist: $FOLDER"
    exit
elif [ ! -d "$STATIC_DIR" ]; then
    echo "Static folder (with templates + static resources) does not exist: $STATIC_DIR"
    exit
fi

# --------------------------------------------------
# ensure all deps exist
# --------------------------------------------------
bash $ROOT_DIR/.requirements/getreqs.sh

# --------------------------------------------------
# rsync with replacement on folder to deliver static
# site to
# --------------------------------------------------
rsync -av --delete $STATIC_DIR $FOLDER

# --------------------------------------------------
# cargo
# !!!this will change later!!!
# --------------------------------------------------
cargo run --release > $FOLDER/index.html

# --------------------------------------------------
# minify all .html files. pass in all HTML_FILES in
# one command for parallel processing
# --------------------------------------------------
HTML_FILES=$(find $FOLDER -name "*.html")
NUM_FILES=$(echo $HTML_FILES | wc -w)
if [ $NUM_FILES -gt 1 ]; then
    minhtml --do-not-minify-doctype $HTML_FILES
elif [ $NUM_FILES -eq 1 ]; then
    # --------------------------------------------------
    # have to specify output on 1 input, weird
    # --------------------------------------------------
    minhtml --do-not-minify-doctype $HTML_FILES -o $HTML_FILES
fi

# --------------------------------------------------
# python server
# --------------------------------------------------
python3 -m http.server 8000 --directory $FOLDER