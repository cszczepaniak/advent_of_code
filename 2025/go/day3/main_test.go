package main

import (
	"testing"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func BenchmarkSolutions(b *testing.B) {
	aoc.BenchmarkSolutions(b, 2025, 3, partA, partB)
}
