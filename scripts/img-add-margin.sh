#!/bin/sh

# Add 50% margin to left and right side of an image
# Usage:
#   $0 <fn>.<suffix> [width]
# output to:
#   <fn>-margin.<suffix>
#   default width is 1600, while target width is 800

fn="${1}"

suffix="${fn##*.}"
output_fn="${fn%.*}-margin.$suffix"

width="${2-1600}"

convert -resize 800 -gravity center -extent $width "$fn" "$output_fn"
