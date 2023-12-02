#!/bin/bash

export AOC_SESSION=$(cat session_key)
GOEXPERIMENT=rangefunc gotip run "day$1/main.go"