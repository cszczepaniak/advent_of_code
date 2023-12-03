package main

import (
	"strconv"
	"strings"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 2,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

func part1(input string) int {
	nCubesInBag := [3]int{
		red:   12,
		green: 13,
		blue:  14,
	}

	pullIsPossible := func(p pull) bool {
		for color, count := range p.cubesByColor {
			if count > nCubesInBag[color] {
				return false
			}
		}
		return true
	}

	gameIsPossible := func(g game) bool {
		for _, p := range g.pulls {
			if !pullIsPossible(p) {
				return false
			}
		}
		return true
	}

	res := 0
	for g := range iterGames(input) {
		if gameIsPossible(g) {
			res += g.id
		}
	}
	return res
}

func part2(input string) int {
	power := func(cubes [3]int) int {
		res := 1
		for _, count := range cubes {
			res *= count
		}
		return res
	}

	res := 0
	for g := range iterGames(input) {
		minSet := g.minimumCubeSet()
		res += power(minSet)
	}

	return res
}

// Games look like this:
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
type game struct {
	id    int
	pulls []pull
}

func iterGames(str string) common.Seq1[game] {
	return func(yield func(game) bool) {
		for line := range common.IterLines(str) {
			g := parseGame(line)
			if !yield(g) {
				return
			}
		}
	}
}

func (g game) minimumCubeSet() [3]int {
	cubes := [3]int{
		red:   0,
		blue:  0,
		green: 0,
	}

	for _, p := range g.pulls {
		for color, count := range p.cubesByColor {
			cubes[color] = max(cubes[color], count)
		}
	}

	return cubes
}

const (
	blue  int = 0
	red   int = 1
	green int = 2
)

type pull struct {
	cubesByColor [3]int
}

func parseGame(str string) game {
	str = strings.TrimPrefix(str, `Game `)
	idStr, list, ok := strings.Cut(str, `: `)
	if !ok {
		panic(`malformed game!`)
	}

	id, err := strconv.Atoi(idStr)
	if err != nil {
		panic(`error parsing game ID: ` + err.Error())
	}

	g := game{
		id: id,
	}

	for pullStr := range common.IterSplitString(list, ';') {
		pull := parsePull(strings.TrimSpace(pullStr))
		g.pulls = append(g.pulls, pull)
	}

	return g
}

func parsePull(str string) pull {
	p := pull{}
	for countStr := range common.IterSplitString(str, ',') {
		numStr, color, ok := strings.Cut(strings.TrimSpace(countStr), ` `)
		if !ok {
			panic(`malformed game!`)
		}

		num, err := strconv.Atoi(numStr)
		if err != nil {
			panic(`error parsing pull count: ` + err.Error())
		}

		switch color {
		case `blue`:
			p.cubesByColor[blue] = num
		case `red`:
			p.cubesByColor[red] = num
		case `green`:
			p.cubesByColor[green] = num
		}
	}
	return p
}
