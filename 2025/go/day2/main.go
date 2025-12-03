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
		if hasRepeatsForPartA(strconv.Itoa(id)) {
			sum += id
		}
	}
	return sum
}

func hasRepeatsForPartA(s string) bool {
	mid := len(s) / 2
	return s[:mid] == s[mid:]
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
	n, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return n
}
