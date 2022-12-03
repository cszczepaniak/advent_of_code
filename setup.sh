#!/bin/bash

year=$(date +"%Y")
day=$1
curl -b $(cat ../cookie.txt) "https://adventofcode.com/$year/day/$day/input" > "input/day$day.txt"