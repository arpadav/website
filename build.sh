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
    echo "Folder-to-generate-to does not exist, making directory w parents...: $FOLDER"
    mkdir -p $FOLDER
elif [ ! -d "$STATIC_DIR" ]; then
    echo "Static folder (with templates + static resources) does not exist: $STATIC_DIR"
    exit
fi

# --------------------------------------------------
# ensure all deps exist
# --------------------------------------------------
bash $ROOT_DIR/.requirements/getreqs.sh

# --------------------------------------------------
# "compile" ts -> js
# --------------------------------------------------
# maybe in the future, the current `index.js` has some magic in it
# --------------------------------------------------
# tsc ts/index.ts --target ES2016 --lib ES2016,DOM --outDir /deploy/$FOLDER/static/js

# --------------------------------------------------
# rsync with replacement on folder to deliver static
# site to
# --------------------------------------------------
rsync -aq --delete --exclude '.git' $STATIC_DIR $FOLDER

# --------------------------------------------------
# cargo, check if vendor folder exists
# --------------------------------------------------
if [ ! -d "vendor" ]; then
    echo "Vendor folder does not exist, attempting to make now..."
    cargo vendor
fi


# --------------------------------------------------
# cargo with deployment folder flag
# --------------------------------------------------
cargo run --release -- --deploy $FOLDER
if [ $? -ne 0 ]; then
    exit
fi

# --------------------------------------------------
# minify all .html files. pass in all HTML_FILES in
# one command for parallel processing
# --------------------------------------------------
HTML_FILES=$(find $FOLDER -name "*.html")
NUM_FILES=$(wc -w <<< "$HTML_FILES")
if [ $NUM_FILES -gt 1 ]; then
    minhtml --do-not-minify-doctype $HTML_FILES > /dev/null 2>&1
elif [ $NUM_FILES -eq 1 ]; then
    # --------------------------------------------------
    # have to specify output on 1 input, weird
    # --------------------------------------------------
    minhtml --do-not-minify-doctype $HTML_FILES -o $HTML_FILES > /dev/null 2>&1
fi

# --------------------------------------------------
# python server
# --------------------------------------------------
python3 -m http.server 8005 --directory $FOLDER
