#!/bin/bash

# --------------------------------------------------
# Finds style-tags in comments of code
# --------------------------------------------------
# What is a style tag? Good question future Arpad. It
# is something I made up to tag any location in any source
# file (Rust, TS, HTML, etc) to make it easier to find
# specific parts of code which contribute to the "style",
# "layout" or other visuals to this statically generated
# website.
# --------------------------------------------------
# Therefore, if quick visual changes are required, can 
# run this script to find exactly where this occurs if you
# havent seen the source in years.
# --------------------------------------------------
PARENT_DIR="$(dirname "$(realpath "$0")")"
bash $PARENT_DIR/find.sh "<<STYLE+TAG>>"