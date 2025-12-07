package main

import (
	"bytes"
	"slices"

	"github.com/cszczepaniak/go-aoc/aoc"

	"github.com/cszczepaniak/advent_of_code/2025/go/utils"
)

func main() {
	err := aoc.Main(2025, 2, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	sum := 0
	for rng := range bytes.SplitSeq(input, []byte{','}) {
		rng = bytes.TrimRight(rng, "\n")
		lo, hi, _ := bytes.Cut(rng, []byte{'-'})
		hiN := utils.SimplerAtoi(hi)
		start, startUpper := findFirstRepeatSequence(lo)
		if start > hiN {
			continue
		}

		n := startUpper
		for {
			num := repeat(n)
			if num > hiN {
				break
			}
			sum += num
			n++
		}
	}
	return sum
}

func repeat(n int) int {
	digs := numDigs(n)
	return n*tenTo(digs) + n
}

func findFirstRepeatSequence(bs []byte) (int, int) {
	if len(bs)%2 == 1 {
		// Odd number of digits: the next chance of a repeat is a sequence of e.g. 100100 if
		// the number was 68928
		digs := len(bs) + 1
		half := tenTo(digs/2 - 1)
		return half*tenTo(digs/2) + half, half
	} else {
		// Even number of digits: the next chance of a repeat is either the upper half
		// repeated or the upper half + 1 repeated.
		//   12341111 -> 12341234
		//   12342111 -> 12351235
		digs := len(bs)
		hi := bs[:len(bs)/2]
		lo := bs[len(bs)/2:]
		hiN := utils.SimplerAtoi(hi)
		if hiN < utils.SimplerAtoi(lo) {
			hiN++
		}
		return hiN*tenTo(digs/2) + hiN, hiN
	}
}

func partB(input []byte) int {
	sum := 0

	dedup := make(map[int]struct{})
	for rng := range bytes.SplitSeq(input, []byte{','}) {
		rng = bytes.TrimRight(rng, "\n")
		lo, hi, _ := bytes.Cut(rng, []byte{'-'})
		loN := utils.SimplerAtoi(lo)
		hiN := utils.SimplerAtoi(hi)

		clear(dedup)
		for i := 1; i <= len(hi)/2; i++ {
			for id := nextID(loN, i); id <= hiN; id = nextID(id+1, i) {
				_, ok := dedup[id]
				if ok {
					continue
				}
				dedup[id] = struct{}{}
				sum += id
			}
		}
	}

	return sum
}

func nextID(inputID int, seqLen int) int {
	numInputDigits := numDigs(inputID)
	nDigitsInNextID := numInputDigits

	for nDigitsInNextID%seqLen != 0 {
		nDigitsInNextID++
	}

	// SPECIAL CASE: if the input ID is a single digit and the seqLen is 1, the next ID has to
	// be two digits even though numInputDigits%seqLen == 0
	if numInputDigits == 1 && seqLen == 1 {
		nDigitsInNextID++
	}

	if nDigitsInNextID > numInputDigits {
		// Bump up number of digits: the next chance of a repeat is a sequence of e.g. 100100 if
		// the number was 68928
		rep := tenTo(seqLen - 1)
		mul := tenTo(seqLen)
		res := rep
		for {
			res *= mul
			res += rep
			if numDigs(res) >= nDigitsInNextID {
				break
			}
		}
		return res
	} else {
		// Search for the next chance of a repeat; 828284 -> 848484, e.g.

		// Extract top N digits
		top := inputID
		div := tenTo(seqLen)
		for numDigs(top) != seqLen {
			top /= div
		}

		// Collect groups of digits
		// TODO: no allocs
		var groups []int
		for inputID > 0 {
			groups = append(groups, inputID%div)
			inputID /= div
		}

		// Start at the group nearest the top; if one is strictly less than the top, we can
		// repeat the top digits; if one is strictly more than the top, we need to use the
		// top + 1
		for _, group := range slices.Backward(groups) {
			if group < top {
				break
			} else if group > top {
				top++
				break
			}
		}

		mul := tenTo(seqLen)
		res := top
		for {
			res *= mul
			res += top
			if numDigs(res) >= nDigitsInNextID {
				break
			}
		}
		return res
	}
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
