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
	inst, input := parseInstructions(input)
	input = parsing.DiscardLine(input)

	var n node
	nodes := make(map[string]node)
	for len(input) > 0 {
		n, input = parseNode(input)
		nodes[n.name] = n
	}

	curr, ok := nodes[`AAA`]
	if !ok {
		panic(`starting node not found`)
	}

	steps := 0
	for ; ; steps++ {
		if curr.name == `ZZZ` {
			break
		}

		var next string
		i := inst.next()
		switch i {
		case 'R':
			next = curr.right
		case 'L':
			next = curr.left
		}

		curr, ok = nodes[next]
		if !ok {
			panic(`no mapped node with name "` + next + `"`)
		}

		if steps%1e6 == 0 {
			fmt.Printf("%d steps taken so far\n", steps)
		}
	}

	return steps
}

func part2(input string) int {
	return 0
}

type instructions struct {
	stream string
	ptr    int
}

func (inst *instructions) next() byte {
	b := inst.stream[inst.ptr]
	inst.ptr = (inst.ptr + 1) % len(inst.stream)
	return b
}

func parseInstructions(str string) (*instructions, string) {
	stream, str := parsing.TakeUntilChar(str, '\n')
	return &instructions{
		stream: stream,
		ptr:    0,
	}, parsing.DiscardLine(str)
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
