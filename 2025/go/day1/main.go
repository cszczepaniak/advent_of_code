package main

import (
	"iter"

	"github.com/cszczepaniak/go-aoc/aoc"

	"github.com/cszczepaniak/advent_of_code/2025/go/utils"
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
		for line := range utils.StringLines(input) {
			sign := signs[line[0]]

			val := utils.SimplerAtoi([]byte(line[1:]))
			if !yield(sign * val) {
				return
			}
		}
	}
}
