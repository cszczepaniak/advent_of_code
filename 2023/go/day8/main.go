package main

import (
	"fmt"

	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 8,
		part1, nil,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

/*
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
*/

func part1(input string) int {
	instructions, input := parsing.TakeUntilChar(input, '\n')
	input = parsing.DiscardLine(input)
	input = parsing.DiscardLine(input)

	instructionPtr := 0
	nextInstruction := func() byte {
		b := instructions[instructionPtr%len(instructions)]
		instructionPtr++
		return b
	}

	var n node
	var start string
	nodes := make(map[string]node)
	for len(input) > 0 {
		n, input = parseNode(input)
		nodes[n.name] = n
		if start == `` {
			start = n.name
		}
	}

	curr := nodes[start]
	steps := 0
	for {
		if curr.name == `ZZZ` {
			break
		}

		switch nextInstruction() {
		case 'R':
			curr = nodes[curr.right]
		case 'L':
			curr = nodes[curr.left]
		}
		steps++
		if steps%1e6 == 0 {
			fmt.Printf("%d steps taken so far\n", steps)
		}
	}

	return steps
}

func part2(input string) int {
	return 0
}

type node struct {
	name        string
	left, right string
}

func parseNode(str string) (node, string) {
	name, str := parsing.TakeUntilChar(str, ' ')
	_, str = parsing.TakeUntilChar(str, '(')
	str = str[1:]
	left, str := parsing.TakeUntilChar(str, ',')
	str = str[2:]
	right, str := parsing.TakeUntilChar(str, ')')

	return node{
		name:  name,
		left:  left,
		right: right,
	}, parsing.DiscardLine(str)
}
