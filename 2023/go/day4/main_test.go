package main

import (
	"context"
	"net/http"
	"os"
	"testing"

	"github.com/cszczepaniak/go-aoc/aoc"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func BenchmarkPart2(b *testing.B) {
	b.Run(`sample input`, func(b *testing.B) {
		for range b.N {
			ans := part2(
				`Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
				Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
				Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
				Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
				Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
				Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`,
			)
			assert.Equal(b, 30, ans)
		}
	})

	k, err := os.ReadFile(`../session_key`)
	require.NoError(b, err)

	input, err := aoc.GetInputString(
		context.Background(),
		http.DefaultClient,
		aoc.NewRequest(2023, 4).WithSessionKey(string(k)).BuildGetInputRequest(),
	)
	require.NoError(b, err)

	b.Run(`real input`, func(b *testing.B) {
		for range b.N {
			ans := part2(input)
			assert.Equal(b, 15455663, ans)
		}
	})
}

/*
INITIAL ANSWER:
BenchmarkPart2/sample_input
BenchmarkPart2/sample_input-16         	  227347	      5213 ns/op	    4832 B/op	      46 allocs/op
BenchmarkPart2/real_input
BenchmarkPart2/real_input-16           	       1	3839726209 ns/op	3356037608 B/op	    3561 allocs/op

ONLY LOOPING THROUGH ONCE AFTER PARSING:
BenchmarkPart2/sample_input
BenchmarkPart2/sample_input-16         	  415584	      2827 ns/op	    3120 B/op	      34 allocs/op
BenchmarkPart2/real_input
BenchmarkPart2/real_input-16           	    5247	    219883 ns/op	  233054 B/op	    1037 allocs/op

ONLY LOOPING THROUGH ONCE INCLUDING PARSING:
BenchmarkPart2/sample_input
BenchmarkPart2/sample_input-16         	  401860	      2921 ns/op	    3120 B/op	      34 allocs/op
BenchmarkPart2/real_input
BenchmarkPart2/real_input-16           	    5102	    228390 ns/op	  233064 B/op	    1037 allocs/op

ONLY KEEPING TRACK OF NUMBER OF COPIES, NOT CARDS:
BenchmarkPart2/sample_input
BenchmarkPart2/sample_input-16         	  429928	      2760 ns/op	    2520 B/op	      34 allocs/op
BenchmarkPart2/real_input
BenchmarkPart2/real_input-16           	    5458	    216354 ns/op	  205071 B/op	    1037 allocs/op
*/
