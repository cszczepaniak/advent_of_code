package main

import (
	"bytes"
	"cmp"
	"slices"

	"github.com/cszczepaniak/go-aoc/aoc"

	"github.com/cszczepaniak/advent_of_code/2025/go/utils"
)

func main() {
	err := aoc.Main(2025, 5, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	rl, input := buildList(input)

	sum := 0
	for line := range utils.ByteLines(input) {
		num := utils.SimplerAtoi(line)
		for _, rng := range rl {
			if rng.contains(num) {
				sum++
				break
			}
		}
	}

	return sum
}

func partB(input []byte) int {
	rl, input := buildList(input)

	sum := 0
	for _, rng := range rl {
		sum += rng.count()
	}
	return sum
}

func buildList(input []byte) (rngList, []byte) {
	rl := make(rngList, 0)
	for {
		newline := bytes.IndexByte(input, '\n')
		var line []byte
		if newline == -1 {
			line = input
			input = input[len(input):]
		} else {
			line = input[:newline]
			input = input[newline+1:]
		}
		if len(line) == 0 {
			break
		}

		start, end, _ := bytes.Cut(line, []byte{'-'})
		rl = rl.insert(utils.SimplerAtoi(start), utils.SimplerAtoi(end))
	}
	return rl, input
}

type rng struct {
	start, end int
}

func (r rng) contains(n int) bool {
	return n >= r.start && n <= r.end
}

func (r rng) count() int {
	return r.end - r.start + 1
}

type rngList []rng

func (rl rngList) insert(start, end int) rngList {
	startIdx, startExact := slices.BinarySearchFunc(rl, start, func(a rng, targ int) int {
		if a.contains(targ) {
			// Report an exact match if this range contains the start
			return 0
		}
		return cmp.Compare(a.start, targ)
	})
	endIdx, endExact := slices.BinarySearchFunc(rl, end, func(a rng, targ int) int {
		if a.contains(targ) {
			// Report an exact match if this range contains the end
			return 0
		}
		return cmp.Compare(a.end, targ)
	})

	rStartIdx := startIdx
	rStart := start
	if startExact {
		// If the starting range contains start, bump down our start to that range's
		rStart = rl[startIdx].start
	}

	rEndIdx := endIdx
	rEnd := end
	if endExact {
		// If the ending range contains end, we need to include the end range in the replace
		// and we need to use the existing end value.
		rEndIdx = endIdx + 1
		rEnd = rl[endIdx].end
	}

	return slices.Replace(rl, rStartIdx, rEndIdx, rng{rStart, rEnd})
}
