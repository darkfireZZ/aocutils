#!/bin/sh

# A script to fetch Advent of Code puzzle input
# Requirements: curl

DESCRIPTION="Shell script to fetch Advent of Code puzzle input

Reads a session cookie from standard input and writes the
puzzle input of the given day to standard output."

USAGE="Usage: $(basename $0) [OPTIONS] <year> <day>
 -h, --help           Print this help"

if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    echo "$DESCRIPTION"
    echo ""
    echo "$USAGE"
    exit 0
fi

if [ "$#" -ne 2 ]; then
    echo "$USAGE" >&2
    exit 2
fi

SESSION_COOKIE=$(cat -)

YEAR=$1
DAY=$2

curl -b "session=$SESSION_COOKIE" "https://adventofcode.com/$YEAR/day/$DAY/input"
