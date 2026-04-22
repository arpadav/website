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
    RELEASE=false
    FOLDER="$ROOT_DIR/deploy/dev"
elif [ "$1" == "prod" ]; then
    echo "Building prod version"
    RELEASE=true
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
# build all external documentation
# --------------------------------------------------
export DEPLOY_FOLDER=$FOLDER
echo "Building external documentation..."
for script in external-repos/.scripts/*.sh; do
    bash "$script"
done

# --------------------------------------------------
# if release, then minify
# --------------------------------------------------
if [ "$RELEASE" = true ]; then
    echo "Minifying..."
    # --------------------------------------------------
    # collect all .html, .css, and .js files
    # --------------------------------------------------
    FILES=()
    while IFS= read -r -d '' file; do
        FILES+=("$file")
    done < <(
        find "$FOLDER" -type f \
            \( \
            -name "*.html" \
            -o -name "*.css" \
            -o -name "*.js" \
            -o -name "*.mjs" \
            -o -name "*.min.js" \
            -o -name "*.json" \
            -o -name "*.svg" \
            \) \
            -print0
    )
    # --------------------------------------------------
    # minify each file with the appropriate tool
    # --------------------------------------------------
    for file in "${FILES[@]}"; do
        case "$file" in
            *.html)
                minhtml "$file" \
                    --minify-js \
                    --minify-css \
                    -o "$file" \
                    >/dev/null || exit
                ;;
            *.css)
                lightningcss \
                    --minify \
                    "$file" \
                    -o "$file" \
                    >/dev/null || exit
                ;;
            # --------------------------------------------------
            # this is some shit third-party vibe coded tool,
            # but it does exactly what i needed: cli to swc
            # and other minification, so props to author thank you
            # --------------------------------------------------
            *.js | *.mjs | *.min.js)
                mni "$file" --preset aggressive -o "$file" >/dev/null || exit
                ;;
            *.json | *.svg)
                mni "$file" -o "$file" >/dev/null || exit
                ;;
        esac
    done
fi

# --------------------------------------------------
# python server
# --------------------------------------------------
python3 -m http.server 8005 --directory $FOLDER
