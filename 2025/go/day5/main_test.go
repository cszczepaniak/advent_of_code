package main

import (
	"testing"

	"github.com/cszczepaniak/go-aoc/aoc"
	"github.com/stretchr/testify/assert"
)

func BenchmarkSolutions(b *testing.B) {
	aoc.BenchmarkSolutions(b, 2025, 5, partA, partB)
}

func TestRangeList(t *testing.T) {
	rl := make(rngList, 0)

	rl = rl.insert(2, 3)
	assert.EqualValues(t, []rng{{2, 3}}, rl)

	rl = rl.insert(1, 2)
	assert.EqualValues(t, []rng{{1, 3}}, rl)

	rl = rl.insert(3, 4)
	assert.EqualValues(t, []rng{{1, 4}}, rl)

	rl = rl.insert(7, 9)
	assert.EqualValues(t, []rng{{1, 4}, {7, 9}}, rl)

	rl = rl.insert(4, 8)
	assert.EqualValues(t, []rng{{1, 9}}, rl)

	rl = rl.insert(10, 11)
	rl = rl.insert(12, 13)
	rl = rl.insert(14, 15)
	assert.EqualValues(t, []rng{{1, 9}, {10, 11}, {12, 13}, {14, 15}}, rl)

	rl = rl.insert(7, 14)
	assert.EqualValues(t, []rng{{1, 15}}, rl)

	rl = rl.insert(1, 15)
	assert.EqualValues(t, []rng{{1, 15}}, rl)

	rl = rl.insert(1, 2)
	assert.EqualValues(t, []rng{{1, 15}}, rl)

	rl = rl.insert(14, 15)
	assert.EqualValues(t, []rng{{1, 15}}, rl)

	rl = rl.insert(5, 7)
	assert.EqualValues(t, []rng{{1, 15}}, rl)

	rl = rl.insert(16, 17)
	rl = rl.insert(18, 19)
	rl = rl.insert(20, 21)
	assert.EqualValues(t, []rng{{1, 15}, {16, 17}, {18, 19}, {20, 21}}, rl)

	rl = rl.insert(19, 30)
	assert.EqualValues(t, []rng{{1, 15}, {16, 17}, {18, 30}}, rl)

	rl = rl.insert(-1, 16)
	assert.EqualValues(t, []rng{{-1, 17}, {18, 30}}, rl)

	rl = rl.insert(-100, 100)
	assert.EqualValues(t, []rng{{-100, 100}}, rl)

	rl = rl.insert(200, 210)
	rl = rl.insert(220, 230)
	rl = rl.insert(240, 250)
	assert.EqualValues(t, []rng{{-100, 100}, {200, 210}, {220, 230}, {240, 250}}, rl)

	rl = rl.insert(105, 231)
	assert.EqualValues(t, []rng{{-100, 100}, {105, 231}, {240, 250}}, rl)
}
