package main

import (
	"strconv"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
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
		card, err := parseCard(line)
		if err != nil {
			panic(`error parsing card: ` + err.Error())
		}

		numWinners := card.numWinningNumbers
		if numWinners > 0 {
			totalScore += 1 << (numWinners - 1)
		}
	}

	return totalScore
}

type card struct {
	winnersSet        common.ByteSet
	numWinningNumbers int
}

func codeFromStr(s string) byte {
	switch len(s) {
	case 1:
		return byte(s[0] - '0')
	case 2:
		return byte(s[0]-'0')*10 + byte(s[1]-'0')
	default:
		panic(s + ` had len ` + strconv.Itoa(len(s)))
	}
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

func parseCard(input string) card {
	for input[0] != ':' {
		input = input[1:]
	}
	input = input[1:]

	input = discardSpaces(input)

	winnerSet := common.NewByteSet()
	var num byte
	for {
		if input[0] == '|' {
			input = input[1:]
			input = discardSpaces(input)
			break
		}

		num, input = parseNumber(input)
		winnerSet.Insert(num)

		input = discardSpaces(input)
	}

	c := card{
		winnersSet: winnerSet,
	}

	for len(input) > 0 {
		num, input = parseNumber(input)
		if c.winnersSet.Contains(num) {
			c.numWinningNumbers++
		}

		input = discardSpaces(input)
	}

	return c
}

func discardSpaces(str string) string {
	for len(str) > 0 && str[0] == ' ' {
		str = str[1:]
	}
	return str
}

func parseNumber(str string) (byte, string) {
	l := 0
	for l < len(str) && str[l] != ' ' {
		l++
	}

	return codeFromStr(str[:l]), str[l:]
}

func part2(input string) int {
	n := 0

	nextIdx := 0
	var copies [256]int

	for i, line := range common.Enumerate(common.IterLines(input)) {
		card, err := parseCard(line)
		if err != nil {
			panic(`malformed card: ` + err.Error())
		}

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

		nWinners := card.numWinningNumbers
		for j := range nWinners {
			idx := i + j + 1
			if idx < nextIdx {
				copies[idx] = copies[idx] + copies[i]
			} else {
				copies[nextIdx] = copies[i]
				nextIdx++
			}
		}
	}

	return n
}
