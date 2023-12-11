package main

import (
	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
	"strconv"
	"strings"
)

func main() {
	err := aoc.Main(
		2023, 6,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

/*
Time:      7  15   30
Distance:  9  40  200
*/

func part1(input string) int {
	input = strings.TrimPrefix(input, `Time:`)
	times, input, err := parsing.ParseSpaceSeparatedNumbers(input)
	if err != nil {
		panic(err)
	}
	input = parsing.DiscardLine(input)

	input = strings.TrimPrefix(input, `Distance:`)
	records, input, err := parsing.ParseSpaceSeparatedNumbers(input)
	if err != nil {
		panic(err)
	}

	return computeMarginOfError(times, records)
}

func part2(input string) int {
	input = strings.TrimPrefix(input, `Time:`)

	nums := ``
	for len(input) > 0 && input[0] != '\n' {
		if input[0] == ' ' {
			input = input[1:]
			continue
		}
		nums += input[0:1]
		input = input[1:]
	}
	input = parsing.DiscardLine(input)

	t, err := strconv.Atoi(nums)
	if err != nil {
		panic(err)
	}

	input = strings.TrimPrefix(input, `Distance:`)

	nums = ``
	for len(input) > 0 && input[0] != '\n' {
		if input[0] == ' ' {
			input = input[1:]
			continue
		}
		nums += input[0:1]
		input = input[1:]
	}

	record, err := strconv.Atoi(nums)
	if err != nil {
		panic(err)
	}

	return computeMarginOfError([]int{t}, []int{record})
}

func computeMarginOfError(times, records []int) int {
	ans := 1
	for time, record := range common.Zip(times, records) {
		nWays := 0
		for i := range time {
			speed := i
			timeRemaining := time - i
			myDist := speed * timeRemaining
			if myDist > record {
				nWays++
			}
		}

		ans *= nWays
	}

	return ans
}
