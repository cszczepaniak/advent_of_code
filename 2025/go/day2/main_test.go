package main

import (
	"testing"

	"github.com/cszczepaniak/go-aoc/aoc"
	"github.com/stretchr/testify/assert"
)

func BenchmarkSolutions(b *testing.B) {
	aoc.BenchmarkSolutions(b, 2025, 2, partA, partB)
}

func TestFindFirstRepeatSequenceN(t *testing.T) {
	num := nextID(54891, 2)
	assert.Equal(t, 101010, num)

	num = nextID(54891, 3)
	assert.Equal(t, 100100, num)

	num = nextID(541917, 2)
	assert.Equal(t, 545454, num)

	num = nextID(548917, 3)
	assert.Equal(t, 549549, num)

	num = nextID(828283, 2)
	assert.Equal(t, 838383, num)

	num = nextID(3131219357, 2)
	assert.Equal(t, 3131313131, num)

	num = nextID(100, 3)
	assert.Equal(t, 100100, num)

	num = nextID(3, 2)
	assert.Equal(t, 1010, num)

	num = nextID(3, 1)
	assert.Equal(t, 11, num)
}

func TestFoo(t *testing.T) {
	aoc.Main(2025, 2, partB, partB)
}
