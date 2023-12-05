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

		numWinners := card.countWinningNumbers()
		if numWinners > 0 {
			totalScore += 1 << (numWinners - 1)
		}
	}

	return totalScore
}

type card struct {
	id      int
	winners common.Set[int]
	mine    []int
}

func (c card) countWinningNumbers() int {
	n := 0
	for _, num := range c.mine {
		if c.winners.Contains(num) {
			n++
		}
	}
	return n
}

func parseCard(input string) (card, error) {
	colon := strings.Index(input, `:`)
	if colon < 0 {
		return card{}, errors.New(`missing colon in card`)
	}

	idStr := strings.TrimSpace(strings.TrimPrefix(strings.TrimSpace(input[:colon]), `Card `))
	id, err := strconv.Atoi(idStr)
	if err != nil {
		return card{}, err
	}

	input = input[colon+1:]
	winners, mine, ok := strings.Cut(input, ` | `)
	if !ok {
		return card{}, errors.New(`no pipe separator in card`)
	}

	// TODO strings.Fields could be replaced with an iterator to save the allocation.

	winnerStrs := strings.Fields(winners)
	winnerSet := common.NewSetWithCapacity[int](len(winnerStrs))
	for _, str := range winnerStrs {
		n, err := strconv.Atoi(str)
		if err != nil {
			return card{}, err
		}
		winnerSet.Insert(n)
	}

	myStrs := strings.Fields(mine)
	myNums := make([]int, 0, len(myStrs))
	for _, str := range myStrs {
		n, err := strconv.Atoi(str)
		if err != nil {
			return card{}, err
		}
		myNums = append(myNums, n)
	}

	return card{
		id:      id,
		winners: winnerSet,
		mine:    myNums,
	}, nil
}

type cardWithCopies struct {
	c       card
	nCopies int
}

func part2(input string) int {
	var cards []cardWithCopies

	for line := range common.IterLines(input) {
		card, err := parseCard(line)
		if err != nil {
			panic(`malformed card: ` + err.Error())
		}
		cards = append(cards, cardWithCopies{
			c:       card,
			nCopies: 1,
		})
	}

	// TODO probably could do this with less looping... it's _really really_ slow.
	n := 0
	for i, c := range cards {
		n += c.nCopies
		nWinners := c.c.countWinningNumbers()
		for j := range nWinners {
			cards[i+j+1].nCopies += c.nCopies
		}
	}
	return n
}
