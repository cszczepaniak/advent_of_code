package main

import (
	"bytes"
	"iter"

	"github.com/cszczepaniak/go-aoc/aoc"

	"github.com/cszczepaniak/advent_of_code/2025/go/utils"
)

func main() {
	err := aoc.Main(2025, 3, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	sum := 0
	for line := range bytes.Lines(input) {
		line = bytes.TrimRight(line, "\n")
		sum += findJoltage(line, 2)
	}
	return sum
}

func partB(input []byte) int {
	sum := 0
	for line := range bytes.Lines(input) {
		line = bytes.TrimRight(line, "\n")
		sum += findJoltage(line, 12)
	}
	return sum

}

func findJoltage(s []byte, digits int) int {
	currIdx := 0
	joltage := 0

	mul := 1
	for range digits - 1 {
		mul *= 10
	}

	for i := range digits {
		// Leave n-1 digits to the right because we need at least this many
		// more digits to make a number of the required size.
		rightPad := digits - 1 - i

		val, idx := findGreatestDigit(s[currIdx : len(s)-rightPad])
		joltage += val * mul
		mul /= 10
		currIdx = currIdx + idx + 1
	}

	return joltage
}

func findGreatestDigit(s []byte) (int, int) {
	answer := -1
	idx := -1
	for i, dig := range digits(s) {
		if dig > answer {
			answer = dig
			idx = i
		}
	}
	return answer, idx
}

func digits(s []byte) iter.Seq2[int, int] {
	return func(yield func(int, int) bool) {
		for i, b := range s {
			if !yield(i, utils.AsciiToDigit(b)) {
				return
			}
		}
	}
}
