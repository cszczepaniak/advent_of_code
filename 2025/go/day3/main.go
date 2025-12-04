package main

import (
	"iter"
	"strings"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	// 	fmt.Println(partB(`987654321111111
	// 811111111111119
	// 234234234234278
	// 818181911112111`))
	// 	return
	err := aoc.Main(2025, 3, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input string) int {
	sum := 0
	for line := range strings.Lines(input) {
		line = strings.TrimRight(line, "\n")
		sum += findJoltage(line, 2)
	}
	return sum
}

func partB(input string) int {
	sum := 0
	for line := range strings.Lines(input) {
		line = strings.TrimRight(line, "\n")
		sum += findJoltage(line, 12)
	}
	return sum

}

func findJoltage(s string, digits int) int {
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

func findGreatestDigit(s string) (int, int) {
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

func digits(s string) iter.Seq2[int, int] {
	return func(yield func(int, int) bool) {
		for i, b := range []byte(s) {
			if !yield(i, int(b-'0')) {
				return
			}
		}
	}
}
