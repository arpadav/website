# --------------------------------------------------
# first, checks if lines is inside of bashrc. if so, exit
# --------------------------------------------------
if [ -f ~/.bashrc ]; then
    if grep -q "lines()" ~/.bashrc; then
        echo "lines() already exists in ~/.bashrc"
        exit
    fi
fi
# --------------------------------------------------
# adds the following to bashrc and re-sources it
# --------------------------------------------------
TEXT="lines() { count=${1:-50}; python3 -c \"print('-' * $count)\"; }"
echo $TEXT >> ~/.bashrc
source ~/.bashrc