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
# cargo flags: check if vendor folder exists
# --------------------------------------------------
CARGO_FLAGS="--config source.crates-io.replace-with='vendored-sources' --config source.vendored-sources.directory='vendor'"
if [ ! -d "vendor" ]; then
    echo "Vendor folder does not exist, attempting to make now..."
    cargo vendor
    if [ $? -eq 0 ]; then
        echo "Vendor folder created successfully."
    else
        echo "Failed to create vendor folder."
        CARGO_FLAGS=""
    fi
fi

# --------------------------------------------------
# rust flags: check if `mold` linker exists, for faster building
# --------------------------------------------------
RUSTFLAGS=""
if command -v mold >/dev/null 2>&1; then
    RUSTFLAGS="-C link-arg=-fuse-ld=mold"
fi

# --------------------------------------------------
# cargo with deployment folder flag
# --------------------------------------------------
export RUSTFLAGS
cmd="cargo run --profile fast-build $CARGO_FLAGS -- --deploy $FOLDER"
echo -e "\n\tRUSTFLAGS=\"$RUSTFLAGS\" $cmd\n"
$cmd
if [ $? -ne 0 ]; then
    exit
fi
unset RUSTFLAGS

# --------------------------------------------------
# minify all .html files
# --------------------------------------------------
HTML_FILES=()
while IFS= read -r -d '' file; do
    HTML_FILES+=("$file")
done < <(find "$FOLDER" -name "*.html" -print0)
# --------------------------------------------------
# need to loop, since `minhtml` can't handle the
# combination of:
# 1. multiple files
# 2. file names/paths with spaces
# if i try to do that using quotes, the paths with spaces do
# not get properly minified
# --------------------------------------------------
# no time to debug this, this will be fine
# --------------------------------------------------
for html_file in "${HTML_FILES[@]}"; do
    minhtml "$html_file" \
        --minify-js \
        --minify-css \
        --do-not-minify-doctype \
        --ensure-spec-compliant-unquoted-attribute-values \
        -o "$html_file" \
        >/dev/null || exit
done

# --------------------------------------------------
# python server
# --------------------------------------------------
python3 -m http.server 8005 --directory $FOLDER
