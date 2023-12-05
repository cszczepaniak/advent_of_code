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
	n := 0
	var cards []cardWithCopies

	for i, line := range common.Enumerate(common.IterLines(input)) {
		card, err := parseCard(line)
		if err != nil {
			panic(`malformed card: ` + err.Error())
		}

		if i < len(cards) {
			// If this card is already in the list,  we need to set the card itself. Also, we need to increment the
			// count to include this particular card (any existing count is just due to copies being added).
			cards[i].c = card
			cards[i].nCopies++
		} else {
			// If this card hasn't been added to the list yet, add it.
			cards = append(cards, cardWithCopies{
				c:       card,
				nCopies: 1,
			})
		}

		n += cards[i].nCopies

		nWinners := card.countWinningNumbers()
		for j := range nWinners {
			idx := i + j + 1
			if idx < len(cards) {
				cards[idx].nCopies += cards[i].nCopies
			} else {
				cards = append(cards, cardWithCopies{
					nCopies: cards[i].nCopies,
				})
			}
		}
	}

	return n
}
