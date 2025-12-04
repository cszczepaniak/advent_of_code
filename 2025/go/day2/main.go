package main

import (
	"bytes"
	"iter"
	"slices"
	"strconv"
	"strings"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(2025, 2, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input string) int {
	sum := 0
	for id := range ids(input) {
		if hasRepeatsForPartA(id) {
			sum += id
		}
	}
	return sum
}

var powersOfTen = [...]int{
	0:  1e0,
	1:  1e1,
	2:  1e2,
	3:  1e3,
	4:  1e4,
	5:  1e5,
	6:  1e6,
	7:  1e7,
	8:  1e8,
	9:  1e9,
	10: 1e10,
	11: 1e11,
	12: 1e12,
	13: 1e13,
	14: 1e14,
	15: 1e15,
	16: 1e16,
	17: 1e17,
	18: 1e18,
}

func tenTo(n int) int {
	return powersOfTen[n]
}

func hasRepeatsForPartA(i int) bool {
	// In order to split the number of digits in half, divide by 10^(numDigits/2)
	divisorPow := numDigs(i) / 2
	divisor := tenTo(divisorPow)

	upper := i / divisor
	lower := i % divisor
	return upper == lower
}

func numDigs(i int) int {
	n := 0
	for i > 0 {
		n++
		i /= 10
	}
	return n
}

func partB(input string) int {
	sum := 0
	for id := range ids(input) {
		strID := []byte(strconv.Itoa(id))
		if lookForRepeats(strID) {
			sum += id
		}
	}
	return sum
}

func lookForRepeats(id []byte) bool {
	// Very naive and slow!
	for i := 1; i <= len(id)/2; i++ {
		var first []byte
		all := true
		for ch := range slices.Chunk(id, i) {
			if len(first) == 0 {
				first = ch
				continue
			}

			if !bytes.Equal(first, ch) {
				all = false
				break
			}
		}
		if all {
			return true
		}
	}

	return false
}

func ids(input string) iter.Seq[int] {
	return func(yield func(int) bool) {
		for idRange := range strings.SplitSeq(input, ",") {
			idRange = strings.TrimRight(idRange, "\n")
			part1, part2, ok := strings.Cut(idRange, "-")
			if !ok {
				panic("invalid ID range: " + idRange)
			}

			// Very naive and slow!
			start := atoi(part1)
			end := atoi(part2)

			for i := start; i <= end; i++ {
				if !yield(i) {
					return
				}
			}
		}
	}
}

func atoi(s string) int {
	mul := 1
	val := 0
	for _, dig := range slices.Backward([]byte(s)) {
		val += mul * int(dig-'0')
		mul *= 10
	}
	return val
}
