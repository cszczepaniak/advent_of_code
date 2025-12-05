package main

import (
	"bytes"
	"iter"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(2025, 4, partA, partB)
	if err != nil {
		panic(err)
	}
}

func partA(input []byte) int {
	grid := make([][]byte, 0)
	for line := range bytes.Lines(input) {
		grid = append(grid, bytes.TrimRight(line, "\n"))
	}

	return len(replaceAccessibleRolls(grid, nil))
}

func partB(input []byte) int {
	grid := make([][]byte, 0)
	for line := range bytes.Lines(input) {
		grid = append(grid, bytes.TrimRight(line, "\n"))
	}

	total := 0
	var canReplace []point
	for {
		canReplace = replaceAccessibleRolls(grid, canReplace)
		if len(canReplace) == 0 {
			break
		}
		total += len(canReplace)
		canReplace = canReplace[:0]
	}

	return total
}

func replaceAccessibleRolls(grid [][]byte, canReplace []point) []point {
	for r := range grid {
		for c := range grid[r] {
			if grid[r][c] != '@' {
				continue
			}

			minR, maxR := max(r-1, 0), min(r+1, len(grid)-1)
			minC, maxC := max(c-1, 0), min(c+1, len(grid[r])-1)

			numNeighbors := 0
			for nr := range numRange(minR, maxR) {
				for nc := range numRange(minC, maxC) {
					if nr == r && nc == c {
						continue
					}

					if grid[nr][nc] == '@' {
						numNeighbors++
					}
				}
			}

			if numNeighbors < 4 {
				canReplace = append(canReplace, point{row: r, col: c})
			}
		}
	}

	for r, c := range points(canReplace) {
		grid[r][c] = 'x'
	}

	return canReplace
}

type point struct {
	row, col int
}

func points(pts []point) iter.Seq2[int, int] {
	return func(yield func(int, int) bool) {
		for _, pt := range pts {
			if !yield(pt.row, pt.col) {
				return
			}
		}
	}
}

func numRange(low, hi int) iter.Seq[int] {
	return func(yield func(int) bool) {
		for i := low; i <= hi; i++ {
			if !yield(i) {
				break
			}
		}
	}
}
