package main

import (
	"iter"
	"strings"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(2025, 1, solveA, solveB, aoc.WithDefaultHTTPClient())
	if err != nil {
		panic(err)
	}
}

// 42303 ns
func solveA(input string) int {
	accum := 50
	numZeroes := 0
	for val := range nums(input) {
		accum += val
		if accum%100 == 0 {
			numZeroes++
		}
	}
	return numZeroes
}

func solveB(input string) int {
	accum := 50
	numZeroes := 0
	for val := range nums(input) {
		lastAccum := accum
		hundreds := val / 100
		if hundreds < 0 {
			hundreds = -hundreds
		}
		numZeroes += hundreds
		rest := val % 100
		accum += rest
		if accum == 0 {
			numZeroes++
		} else if accum < 0 {
			accum += 100
			if lastAccum != 0 {
				numZeroes++
			}
		} else if accum >= 100 {
			accum -= 100
			numZeroes++
		}
	}
	return numZeroes
}

var signs = [...]int{
	'L': -1,
	'R': 1,
}

func nums(input string) iter.Seq[int] {
	return func(yield func(int) bool) {
		for line := range strings.Lines(input) {
			sign := signs[line[0]]

			val := 0
			mul := 1

			// Start from the penultimate character because line contains the newline.
			for i := len(line) - 2; i >= 1; i-- {
				val += int(line[i]-'0') * mul
				mul *= 10
			}

			if !yield(sign * int(val)) {
				return
			}
		}
	}
}
