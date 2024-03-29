package main

import (
	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 4,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

func part1(input string) int {
	totalScore := 0
	for line := range common.IterLines(input) {
		numWinners, _ := parseCard(line)
		if numWinners > 0 {
			totalScore += 1 << (numWinners - 1)
		}
	}

	return totalScore
}

func codeFromStr(s string) byte {
	switch len(s) {
	case 1:
		return byte(s[0] - '0')
	case 2:
		return byte(s[0]-'0')*10 + byte(s[1]-'0')
	}

	panic(`unexpected str len`)
}

func numberFields(str string) common.Seq1[string] {
	return func(yield func(string) bool) {
		for i := 0; i < len(str); i++ {
			if str[i] == ' ' {
				continue
			}
			start := i
			for i < len(str) && str[i] != ' ' {
				i++
			}
			if !yield(str[start:i]) {
				return
			}
		}
	}
}

// A card looks like this:
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

func parseCard(input string) (int, string) {
	for input[0] != ':' {
		input = input[1:]
	}
	input = input[1:]

	input = parsing.DiscardSpaces(input)

	winnerSet := common.NewByteSet()
	var num byte
	for {
		if input[0] == '|' {
			input = input[1:]
			input = parsing.DiscardSpaces(input)
			break
		}

		num, input = parseNumber(input)
		winnerSet.Insert(num)

		input = parsing.DiscardSpaces(input)
	}

	nWinners := 0

	for len(input) > 0 && input[0] != '\n' {
		num, input = parseNumber(input)
		if winnerSet.Contains(num) {
			nWinners++
		}

		input = parsing.DiscardSpaces(input)
	}

	return nWinners, input
}

func parseNumber(str string) (byte, string) {
	l := 0
	for l < len(str) && str[l] != ' ' && str[l] != '\n' {
		l++
	}

	return codeFromStr(str[:l]), str[l:]
}

func part2(input string) int {
	n := 0

	nextIdx := 0
	var copies [256]int

	var nWinners int
	i := 0
	for {
		nWinners, input = parseCard(input)
		if i < nextIdx {
			// If this card is already in the list,  we need to set the card itself. Also, we need to increment the
			// count to include this particular card (any existing count is just due to copies being added).
			copies[i] = copies[i] + 1
		} else {
			// If this card hasn't been added to the list yet, add it.
			copies[nextIdx] = 1
			nextIdx++
		}

		n += copies[i]

		for j := 0; j < nWinners; j++ {
			idx := i + j + 1
			if idx < nextIdx {
				copies[idx] = copies[idx] + copies[i]
			} else {
				copies[nextIdx] = copies[i]
				nextIdx++
			}
		}

		if len(input) > 1 {
			input = input[1:]
			i++
		} else {
			break
		}
	}

	return n
}
