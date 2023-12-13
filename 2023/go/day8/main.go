package main

import (
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
	"strings"
	"sync"
)

func main() {
	// 	ans := part2(`LR
	//
	// 11A = (11B, XXX)
	// 11B = (XXX, 11Z)
	// 11Z = (11B, XXX)
	// 22A = (22B, XXX)
	// 22B = (22C, 22C)
	// 22C = (22Z, 22Z)
	// 22Z = (22B, 22B)
	// XXX = (XXX, XXX)`)
	// 	fmt.Println(ans)
	// 	return
	err := aoc.Main(
		2023, 8,
		part1, part2,
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

	var parsedNode node
	nodes := newNodeMap()
	for len(input) > 0 {
		parsedNode, input = parseNode(input)
		nodes.add(parsedNode)
	}

	return newNodeWalker(
		`AAA`,
		inst,
		nodes,
	).stepsUntil(func(s string) bool {
		return s == `ZZZ`
	})
}

func part2(input string) int {
	inst, input := parseInstructions(input)
	input = parsing.DiscardLine(input)

	nodes := make([]*nodeWalker, 0)
	nm := newNodeMap()
	for len(input) > 0 {
		n, newInput := parseNode(input)
		nm.add(n)
		if strings.HasSuffix(n.name, `A`) {
			nodes = append(
				nodes,
				newNodeWalker(n.name, inst, nm),
			)
		}
		input = newInput
	}

	wg := &sync.WaitGroup{}
	wg.Add(len(nodes))

	outputs := make([]int, len(nodes))

	for i, nw := range nodes {
		i := i
		nw := nw
		go func() {
			defer wg.Done()

			outputs[i] = nw.stepsUntil(func(s string) bool {
				return strings.HasSuffix(s, `Z`)
			})
		}()
	}

	wg.Wait()

	return leastCommonMultiple(outputs...)
}

func leastCommonMultiple(ns ...int) int {
	facts := make(map[int]int)
	for _, n := range ns {
		pfs := primeFactors(n)

		theseFactors := make(map[int]int)
		for _, pf := range pfs {
			theseFactors[pf]++
		}

		for fact, ct := range theseFactors {
			facts[fact] = max(facts[fact], ct)
		}
	}

	res := 1
	for fact, ct := range facts {
		for range ct {
			res *= fact
		}
	}
	return res
}

func primeFactors(i int) []int {
	var factors []int

	fact := 2
	for i > 1 {
		for i%fact == 0 {
			factors = append(factors, fact)
			i /= fact
		}
		fact++
	}

	return factors
}

type instructions struct {
	stream string
	ptr    int
}

// next returns the next instruction in the stream
// Note: this is a pointer receiver even though instructions is usually referenced by value. That's because we must
// mutate the instruction pointer.
func (inst *instructions) next() byte {
	b := inst.stream[inst.ptr]
	inst.ptr = (inst.ptr + 1) % len(inst.stream)
	return b
}

func parseInstructions(str string) (instructions, string) {
	stream, str := parsing.TakeUntilChar(str, '\n')
	return instructions{
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

	n := node{
		name:  name,
		left:  left,
		right: right,
	}

	return n, parsing.DiscardLine(str)
}

type nodeWalker struct {
	curr string
	inst instructions
	nm   nodeMap
}

// Note: it's important that we pass instructions by value here (i.e. as a copy), because each *nodeWalker will mutate
// its own instructions.
func newNodeWalker(start string, inst instructions, nm nodeMap) *nodeWalker {
	return &nodeWalker{
		curr: start,
		inst: inst,
		nm:   nm,
	}
}

func (nw *nodeWalker) stepsUntil(fn func(s string) bool) int {
	steps := 0
	for ; !fn(nw.curr); steps++ {
		nw.curr = nw.nm.next(nw.curr, nw.inst.next())
	}
	return steps
}

// nodeMap maps a node's name to its R and L paths.
type nodeMap map[string]map[byte]string

func newNodeMap() nodeMap {
	return make(nodeMap)
}

func (m nodeMap) add(n node) {
	m[n.name] = map[byte]string{
		'R': n.right,
		'L': n.left,
	}
}

func (m nodeMap) next(name string, inst byte) string {
	return m[name][inst]
}
