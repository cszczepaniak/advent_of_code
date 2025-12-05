package utils

import (
	"bytes"
	"iter"
	"slices"
	"strings"
)

func ByteLines(s []byte) iter.Seq[[]byte] {
	return func(yield func([]byte) bool) {
		for line := range bytes.Lines(s) {
			if !yield(bytes.TrimRight(line, "\n")) {
				return
			}
		}
	}
}

func StringLines(s string) iter.Seq[string] {
	return func(yield func(string) bool) {
		for line := range strings.Lines(s) {
			if !yield(strings.TrimRight(line, "\n")) {
				return
			}
		}
	}
}

func SimplerAtoi(s []byte) int {
	mul := 1
	val := 0
	for _, dig := range slices.Backward(s) {
		val += mul * AsciiToDigit(dig)
		mul *= 10
	}
	return val
}

func AsciiToDigit(c byte) int {
	return int(c - '0')
}
