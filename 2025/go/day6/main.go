package main

import (
	"bytes"

	"github.com/cszczepaniak/go-aoc/aoc"

	"github.com/cszczepaniak/advent_of_code/2025/go/utils"
)

func main() {
	err := aoc.Main(2025, 6, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	input = bytes.TrimRight(input, "\n")
	idx := bytes.LastIndexByte(input, '\n')
	nums := input[:idx]

	ops := parseOps(input[idx+1:])

	results := make([]int, len(ops))
	sum := 0

	for line := range utils.ByteLines(nums) {
		i := 0
		n := 0
		for _, b := range line {
			if b == ' ' {
				if n > 0 {
					// We previously summed up a number; include it in our
					// results.
					switch ops[i] {
					case '+':
						sum += n
					case '*':
						results[i] = max(1, results[i]) * n
					}
					i++
					n = 0
				}
				continue
			}

			n *= 10
			n += int(b - '0')
		}
		sum += n
	}

	for _, r := range results {
		sum += r
	}
	return sum
}

func parseOps(l []byte) [1024]byte {
	var ops [1024]byte
	i := 0
	for _, b := range l {
		switch b {
		case '+', '*':
			ops[i] = b
			i++
		}
	}
	return ops
}

func partB(input []byte) int {
	input = bytes.TrimRight(input, "\n")
	idx := bytes.LastIndexByte(input, '\n')

	ops := parseOps(input[idx+1:])

	lines := bytes.Split(input[:idx], []byte{'\n'})

	col := 0
	sum := 0

	var currRes int
	resetRes := func() {
		switch ops[col] {
		case '+':
			currRes = 0
		case '*':
			currRes = 1
		}
	}
	resetRes()

	for len(lines[0]) > 0 {
		num, ok := numFromCol(lines)
		if !ok {
			sum += currRes
			col++
			resetRes()
			continue
		}

		switch ops[col] {
		case '+':
			currRes += num
		case '*':
			currRes *= num
		}
	}

	return sum + currRes
}

func numFromCol(lines [][]byte) (int, bool) {
	allSpaces := true
	num := 0
	for i, l := range lines {
		if len(l) == 0 {
			return 0, false
		}

		c := l[0]
		lines[i] = l[1:]
		if c == ' ' {
			continue
		}

		allSpaces = false
		num *= 10
		num += int(c - '0')
	}

	if allSpaces {
		return 0, false
	}

	return num, true
}
