package main

import (
	"fmt"
	"unicode"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 3,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

func part1(input string) int {
	g := newGrid(input)

	sum := 0
	for num := range g.iterateNumbers() {
		if shouldInclude(g, num) {
			sum += num.val
		}
	}
	fmt.Println(sum)
	return sum
}

func shouldInclude(g grid, num number) bool {
	// For every number, we must look around the outside to see if there's an adjacent symbol.
	for col := num.startCol; col < num.stopCol; col++ {
		if num.row > 0 && g.isSymbolAt(num.row-1, col) {
			return true
		}
		if num.row < len(g.data) && g.isSymbolAt(num.row+1, col) {
			return true
		}
	}

	if num.startCol > 0 {
		for row := num.row - 1; row <= num.row+1; row++ {
			if g.isSymbolAt(row, num.startCol-1) {
				return true
			}
		}
	}

	if num.startCol < len(g.data[0])-1 {
		for row := num.row - 1; row <= num.row+1; row++ {
			if g.isSymbolAt(row, num.stopCol) {
				return true
			}
		}
	}

	return false
}

func part2(input string) int {
	g := newGrid(input)
	sum := common.Sum(g.iterateGears())
	fmt.Println(sum)
	return sum
}

/*
Example schematic:
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
*/

type grid struct {
	data [][]rune
}

func newGrid(str string) grid {
	g := grid{}
	for line := range common.IterLines(str) {
		g.data = append(g.data, []rune(line))
	}
	return g
}

func (g grid) neighborsOf(inputRow, inputCol int) common.Seq1[location] {
	startRow := max(0, inputRow-1)
	stopRow := min(len(g.data)-1, inputRow+1)
	startCol := max(0, inputCol-1)
	stopCol := min(len(g.data[0])-1, inputCol+1)

	inputLoc := location{
		row: inputRow,
		col: inputCol,
	}

	return func(yield func(location) bool) {
		for row := startRow; row <= stopRow; row++ {
			for col := startCol; col <= stopCol; col++ {
				loc := location{
					row: row,
					col: col,
				}
				if loc == inputLoc {
					continue
				}
				if !yield(loc) {
					return
				}
			}
		}
	}
}

func (g grid) isSymbolAt(row, col int) bool {
	if row < 0 || row >= len(g.data) {
		return false
	}
	if col < 0 || col >= len(g.data[0]) {
		return false
	}

	r := g.runeAt(location{
		row: row,
		col: col,
	})
	return !unicode.IsDigit(r) && r != '.'
}

func (g grid) left(loc location) (location, bool) {
	if loc.col == 0 {
		return location{}, false
	}
	return location{
		col: loc.col - 1,
		row: loc.row,
	}, true
}

func (g grid) right(loc location) (location, bool) {
	if loc.col >= len(g.data[0])-1 {
		return location{}, false
	}
	return location{
		col: loc.col + 1,
		row: loc.row,
	}, true
}

func (g grid) up(loc location) (location, bool) {
	if loc.row == 0 {
		return location{}, false
	}
	return location{
		col: loc.row - 1,
		row: loc.col,
	}, true
}

func (g grid) down(loc location) (location, bool) {
	if loc.col >= len(g.data) {
		return location{}, false
	}
	return location{
		col: loc.row + 1,
		row: loc.col,
	}, true
}

type location struct {
	row int
	col int
}

func (g grid) runeAt(loc location) rune {
	return g.data[loc.row][loc.col]
}

func (g grid) numberAt(loc location) (number, bool) {
	if loc.col < 0 || loc.col >= len(g.data[0]) {
		return number{}, false
	}
	if loc.row < 0 || loc.row >= len(g.data) {
		return number{}, false
	}

	if !unicode.IsDigit(g.runeAt(loc)) {
		return number{}, false
	}

	startCol := loc.col
	for l, ok := g.left(loc); ok && unicode.IsDigit(g.runeAt(l)); l, ok = g.left(l) {
		startCol = l.col
	}
	stopCol := loc.col
	for l, ok := g.right(loc); ok && unicode.IsDigit(g.runeAt(l)); l, ok = g.right(l) {
		stopCol = l.col
	}

	return number{
		val:      numFromRunes(g.data[loc.row][startCol : stopCol+1]),
		row:      loc.row,
		startCol: startCol,
		stopCol:  stopCol + 1,
	}, true
}

type gear struct {
	num1 int
	num2 int
}

func (g grid) iterateGears() common.Seq1[int] {
	return func(yield func(int) bool) {
		for row, data := range g.data {
			for col, r := range data {
				if r != '*' {
					continue
				}

				nums := make([]number, 0, 3)
				for neighbor := range g.neighborsOf(row, col) {
					if n, ok := g.numberAt(neighbor); ok {
						isDupe := false
						for _, num := range nums {
							if num == n {
								isDupe = true
								break
							}
						}
						if !isDupe {
							nums = append(nums, n)
						}
					}
				}
				if len(nums) == 2 && !yield(nums[0].val*nums[1].val) {
					return
				}
			}
		}
	}
}

type number struct {
	val      int
	row      int
	startCol int
	stopCol  int
}

func (g grid) iterateNumbers() common.Seq1[number] {
	return func(yield func(number) bool) {
		for row, data := range g.data {
			for col := 0; col < len(data); col++ {
				if unicode.IsDigit(data[col]) {
					start := col
					for col < len(data) && unicode.IsDigit(data[col]) {
						col++
					}
					if !yield(number{
						val:      numFromRunes(data[start:col]),
						row:      row,
						startCol: start,
						stopCol:  col,
					}) {
						return
					}
				}
			}
		}
	}
}

func numFromRunes(rs []rune) int {
	n := 0
	mult := 1
	for i := len(rs) - 1; i >= 0; i-- {
		n += mult * int(rs[i]-'0')
		mult *= 10
	}
	return n
}
