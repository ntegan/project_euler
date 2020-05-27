#!/bin/bash
url="https://projecteuler.net/minimal="


if [[ "$1" == "" ]]
then
    echo "Usage: $0 <prob number>"
else
    wget -o /dev/null -O "info.html$1" "$url""$1"
fi
