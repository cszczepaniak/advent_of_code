package main

import (
	"bytes"
	"iter"
	"slices"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(2025, 2, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	sum := 0
	for start, end := range idRange(input) {
		for id := start; id <= end; {
			if has, div := hasRepeatsForPartA(id); has {
				sum += id
				id += div
			} else {
				id += 1
			}
		}
	}
	return sum
}

func hasRepeatsForPartA(i int) (bool, int) {
	// In order to split the number of digits in half, divide by 10^(numDigits/2)
	divisorPow := numDigs(i) / 2
	divisor := tenTo(divisorPow)

	upper := i / divisor
	lower := i % divisor
	return upper == lower, divisor
}

func partB(input []byte) int {
	sum := 0
	for id := range ids(input) {
		if lookForRepeats(id) {
			sum += id
		}
	}
	return sum
}

func lookForRepeats(id int) bool {
	idLen := numDigs(id)
	for i := 1; i <= idLen/2; i++ {
		var ints [16]int
		all := true
		divisor := tenTo(i)

		idCpy := id
		idx := 0
		for idCpy > 0 {
			ints[idx] = idCpy % divisor
			if idx > 0 && ints[0] != ints[idx] {
				all = false
				break
			}
			idx++
			idCpy /= divisor
		}

		// The modulo check here rejects numbers like 1001001; without the modulo check and
		// with i = 3, this number appears to have three repeated digits: 1, 1, 1.
		if all && idLen%i == 0 {
			return true
		}
	}

	return false
}

func idRange(input []byte) iter.Seq2[int, int] {
	return func(yield func(int, int) bool) {
		for idRange := range bytes.SplitSeq(input, []byte{','}) {
			idRange = bytes.TrimRight(idRange, "\n")
			part1, part2, ok := bytes.Cut(idRange, []byte{'-'})
			if !ok {
				panic("invalid ID range")
			}

			// Very naive and slow!
			start := atoi(part1)
			end := atoi(part2)

			if !yield(start, end) {
				return
			}
		}
	}
}

func ids(input []byte) iter.Seq[int] {
	return func(yield func(int) bool) {
		for start, end := range idRange(input) {
			for i := start; i <= end; i++ {
				if !yield(i) {
					return
				}
			}
		}
	}
}

func atoi(s []byte) int {
	mul := 1
	val := 0
	for _, dig := range slices.Backward(s) {
		val += mul * int(dig-'0')
		mul *= 10
	}
	return val
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

func numDigs(i int) int {
	// Instead of using an iterative approach or a log10, doing this if/else chain is a bit
	// faster.
	switch {
	case i < 1e1:
		return 1
	case i < 1e2:
		return 2
	case i < 1e3:
		return 3
	case i < 1e4:
		return 4
	case i < 1e5:
		return 5
	case i < 1e6:
		return 6
	case i < 1e7:
		return 7
	case i < 1e8:
		return 8
	case i < 1e9:
		return 9
	case i < 1e10:
		return 10
	case i < 1e11:
		return 11
	case i < 1e12:
		return 12
	case i < 1e13:
		return 13
	case i < 1e14:
		return 14
	case i < 1e15:
		return 15
	case i < 1e16:
		return 16
	case i < 1e17:
		return 17
	case i < 1e18:
		return 18
	}
	return 19
}
