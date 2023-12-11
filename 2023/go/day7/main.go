package main

import (
	"cmp"
	"slices"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 7,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

/*
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
*/

func part1(input string) int {
	return solve(input, partOneRankings, hand.rank)
}

func part2(input string) int {
	return solve(input, partTwoRankings, hand.rankWithJokers)
}

func solve(input string, rankings [numCards]int, rankFunc func(h hand) handRank) int {
	hands := make([]hand, 0)

	var h hand
	for len(input) > 0 {
		h, input = parseHand(input)
		hands = append(hands, h)
	}

	slices.SortFunc(hands, func(a, b hand) int {
		return a.cmp(b, rankings, rankFunc)
	})

	sum := 0
	for i, h := range hands {
		sum += (i + 1) * h.bid
	}

	return sum
}

type hand struct {
	cards        [5]card
	countPerCard [numCards]int
	bid          int
}

func (h hand) cmp(other hand, cardRankings [numCards]int, rankFunc func(h hand) handRank) int {
	r := cmp.Compare(rankFunc(h), rankFunc(other))
	if r != 0 {
		return r
	}

	for c1, c2 := range common.Zip(h.cards[:], other.cards[:]) {
		if r := cmp.Compare(cardRankings[c1], cardRankings[c2]); r != 0 {
			return r
		}
	}
	return 0
}

func (h hand) rank() handRank {
	hasTriple := false
	numPairs := 0
	for _, ct := range h.countPerCard {
		switch ct {
		case 5:
			return fiveOfAKind
		case 4:
			return fourOfAKind
		case 3:
			hasTriple = true
		case 2:
			numPairs++
		}
	}

	if hasTriple {
		if numPairs > 0 {
			return fullHouse
		}
		return threeOfAKind
	}
	switch numPairs {
	case 2:
		return twoPair
	case 1:
		return onePair
	}
	return highCard
}

func (h hand) rankWithJokers() handRank {
	numJokers := h.countPerCard[JackJoker]
	hasTriple := false
	numPairs := 0
	mostOfOneCard := 0
	for c, ct := range h.countPerCard {
		if card(c) == JackJoker {
			continue
		}
		mostOfOneCard = max(mostOfOneCard, ct)
		switch ct {
		case 3:
			hasTriple = true
		case 2:
			numPairs++
		}
	}

	if hasTriple && numPairs == 1 {
		// Full house; there can't be any jokers, this is the best.
		return fullHouse
	}
	if numPairs == 2 && numJokers == 1 {
		// Joker makes this a full house.
		return fullHouse
	} else if numPairs == 2 {
		// If there wasn't a joker, two pairs is the best.
		return twoPair
	}

	// From here we have our normal cases.
	switch mostOfOneCard + numJokers {
	case 5:
		return fiveOfAKind
	case 4:
		return fourOfAKind
	case 3:
		return threeOfAKind
	case 2:
		return onePair
	default:
		return highCard
	}
}

func parseHand(str string) (hand, string) {
	var h hand
	h.countPerCard, h.cards, str = parseCards(str)
	str = parsing.DiscardSpaces(str)
	n, str, err := parsing.ParseNumber(str)
	if err != nil {
		panic(err)
	}
	h.bid = n
	return h, parsing.DiscardLine(str)
}

func parseCards(str string) ([numCards]int, [5]card, string) {
	counts := [numCards]int{}
	cs := [5]card{}
	for i := 0; i < 5; i++ {
		c := cardFromChar(str[i])
		counts[c]++
		cs[i] = c
	}
	return counts, cs, str[5:]
}

type card int

const (
	Two card = iota
	Three
	Four
	Five
	Six
	Seven
	Eight
	Nine
	Ten
	JackJoker
	Queen
	King
	Ace

	numCards
)

var partOneRankings = [...]int{
	Two:       0,
	Three:     1,
	Four:      2,
	Five:      3,
	Six:       4,
	Seven:     5,
	Eight:     6,
	Nine:      7,
	Ten:       8,
	JackJoker: 9,
	Queen:     10,
	King:      11,
	Ace:       12,
}

var partTwoRankings = [...]int{
	JackJoker: 0,
	Two:       1,
	Three:     2,
	Four:      3,
	Five:      4,
	Six:       5,
	Seven:     6,
	Eight:     7,
	Nine:      8,
	Ten:       9,
	Queen:     10,
	King:      11,
	Ace:       12,
}

func cardFromChar(ch byte) card {
	switch ch {
	case 'A':
		return Ace
	case 'K':
		return King
	case 'Q':
		return Queen
	case 'J':
		return JackJoker
	case 'T':
		return Ten
	default:
		return card(ch - '2')
	}
}

type handRank int

const (
	highCard handRank = iota
	onePair
	twoPair
	threeOfAKind
	fullHouse
	fourOfAKind
	fiveOfAKind
)
