package main

import (
	"context"
	"net/http"
	"testing"

	"github.com/cszczepaniak/go-aoc/aoc"
	"github.com/stretchr/testify/require"
)

func BenchmarkSolutions(b *testing.B) {
	ctx := context.Background()
	input, err := aoc.GetInputString(
		ctx,
		http.DefaultClient,
		aoc.NewRequest(2025, 1).WithSessionKeyFromEnv("AOC_SESSION").BuildGetInputRequest(),
	)
	require.NoError(b, err)

	b.Run(`part a`, func(b *testing.B) {
		for b.Loop() {
			solveA(input)
		}
	})
	b.Run(`part b`, func(b *testing.B) {
		for b.Loop() {
			solveB(input)
		}
	})
}
