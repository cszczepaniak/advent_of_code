package main

import (
	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 9,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

/*
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
*/

func part1(input string) int {
	sum := 0
	for len(input) > 0 {
		nums, newInput, err := parsing.ParseSpaceSeparatedNumbers(input)
		if err != nil {
			panic(err)
		}
		newInput = parsing.DiscardLine(newInput)

		sum += newReport(nums...).analyzeForward()

		input = newInput
	}

	return sum
}

func part2(input string) int {
	sum := 0
	for len(input) > 0 {
		nums, newInput, err := parsing.ParseSpaceSeparatedNumbers(input)
		if err != nil {
			panic(err)
		}
		newInput = parsing.DiscardLine(newInput)

		sum += newReport(nums...).analyzeBackward()

		input = newInput
	}

	return sum
}

type report struct {
	values []int
	sign   int
	sum    int
}

func newReport(values ...int) *report {
	return &report{
		sign:   1,
		values: values,
	}
}

func (r *report) iteratePairs() common.Seq2[int, int] {
	return common.Zip(r.values[:len(r.values)-1], r.values[1:])
}

func (r *report) addLastValue() {
	r.sum += r.values[len(r.values)-1]
}

func (r *report) analyzeForward() int {
	for {
		allZero := true
		r.addLastValue()

		for i := 0; i < len(r.values)-1; i++ {
			n1 := r.values[i]
			n2 := r.values[i+1]

			r.values[i] = n2 - n1
			if r.values[i] != 0 {
				allZero = false
			}
		}
		r.values = r.values[:len(r.values)-1]

		if allZero {
			break
		}
	}

	return r.sum
}

func (r *report) addFirstValue() {
	r.sum += r.sign * r.values[0]
	r.sign *= -1
}

func (r *report) analyzeBackward() int {
	for {
		allZero := true
		r.addFirstValue()

		for i := 0; i < len(r.values)-1; i++ {
			n1 := r.values[i]
			n2 := r.values[i+1]

			r.values[i] = n2 - n1
			if r.values[i] != 0 {
				allZero = false
			}
		}
		r.values = r.values[:len(r.values)-1]

		if allZero {
			break
		}
	}

	return r.sum
}
