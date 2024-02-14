#!/bin/sh

# Requirements: sed

DESCRIPTION="Initializes a rust crate for a specific day of Advent of Code

Reads a session cookie from standard input and initializes a rust crate
for solving a given day's Advent of Code puzzle"

USAGE="Usage: $(basename $0) [OPTIONS] <path> <year> <day>
 -h, --help           Print this help"

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    echo "$DESCRIPTION"
    echo ""
    echo "$USAGE"
    exit 0
fi

if [ "$#" -ne 3 ]; then
    echo "$USAGE" >&2
    exit 1
fi

OUTPATH=$1
YEAR=$2
DAY=$3
SESSION_COOKIE="$(cat -)"

if [ -e "$OUTPATH" ]; then
    echo "$OUTPATH exists already" >&2
    exit 1
fi

TEMPLATE_LOCATION="$(dirname $(readlink -f $0))"

cp -r "$TEMPLATE_LOCATION/day_crate_template" "$OUTPATH"
sed -i "s/^name = \"aoc_day_template\"\$/name = \"aoc_$YEAR_$DAY\"/" $OUTPATH/Cargo.toml
echo "$SESSION_COOKIE" | "$TEMPLATE_LOCATION/fetch_input.sh" $YEAR $DAY > "$OUTPATH/input"
