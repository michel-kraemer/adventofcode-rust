# based on script.sh from HyperNeutrino:
# https://github.com/hyper-neutrino/advent-of-code/blob/main/scripts.sh

export AOC_COOKIE=$(cat $(dirname $0)/cookie.txt) # get this from the cookies tab in network tools on the AOC website

function aoc-load () {
    if [ $1 ]
    then
        curl --cookie "session=$AOC_COOKIE" https://adventofcode.com/$1/day/$2/input > input.txt
    else
        curl --cookie "session=$AOC_COOKIE" "$(echo `date +https://adventofcode.com/%Y/day/%d/input` | sed 's/\/0/\//g')" > input.txt
    fi
    cat input.txt
}
