package main

import (
	"errors"
	"strconv"
	"strings"

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

// A card looks like this:
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

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
	winnersSet        [256]int
	numWinningNumbers int

	winners common.Set[int]
	mine    []int
}

func codeFromStr(s string) int {
	switch len(s) {
	case 1:
		return int(s[0] - '0')
	case 2:
		return int(s[0]-'0')*10 + int(s[1]-'0')
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

func parseCard(input string) (card, error) {
	colon := strings.Index(input, `:`)
	if colon < 0 {
		return card{}, errors.New(`missing colon in card`)
	}

	input = input[colon+1:]
	winners, mine, ok := strings.Cut(input, ` | `)
	if !ok {
		return card{}, errors.New(`no pipe separator in card`)
	}

	winnerSet := [256]int{}
	for str := range numberFields(winners) {
		winnerSet[codeFromStr(str)] = 1
	}

	c := card{
		winnersSet: winnerSet,
	}

	for str := range numberFields(mine) {
		code := codeFromStr(str)
		c.numWinningNumbers += c.winnersSet[code]
	}

	return c, nil
}

func part2(input string) int {
	n := 0
	var copies []int

	for i, line := range common.Enumerate(common.IterLines(input)) {
		card, err := parseCard(line)
		if err != nil {
			panic(`malformed card: ` + err.Error())
		}

		if i < len(copies) {
			// If this card is already in the list,  we need to set the card itself. Also, we need to increment the
			// count to include this particular card (any existing count is just due to copies being added).
			copies[i] = copies[i] + 1
		} else {
			// If this card hasn't been added to the list yet, add it.
			copies = append(copies, 1)
		}

		n += copies[i]

		nWinners := card.numWinningNumbers
		for j := range nWinners {
			idx := i + j + 1
			if idx < len(copies) {
				copies[idx] = copies[idx] + copies[i]
			} else {
				copies = append(copies, copies[i])
			}
		}
	}

	return n
}
