package main

import (
	"context"
	"fmt"
	"iter"
	"net/http"
	"strconv"
	"strings"

	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	ctx := context.Background()
	input, err := aoc.GetInputString(
		ctx,
		http.DefaultClient,
		aoc.NewRequest(2025, 1).WithSessionKeyFromEnv("AOC_SESSION").BuildGetInputRequest(),
	)
	if err != nil {
		panic(err)
	}

	fmt.Println(solveA(input))

	fmt.Println(solveB(`L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
`))
	fmt.Println(solveBVeryVeryNaive(input))
}

// 42303 ns
func solveA(input string) int {
	accum := 50
	numZeroes := 0
	for val := range nums(input) {
		accum += val
		if accum%100 == 0 {
			numZeroes++
		}
	}
	return numZeroes
}

func solveB(input string) int {
	accum := 50
	numZeroes := 0
	for val := range nums(input) {
		accum += val
	}
	return numZeroes
}

// 609531 ns
func solveBVeryVeryNaive(input string) int {
	accum := 50
	numZeroes := 0
	for val := range nums(input) {
		if val > 0 {
			for range val {
				accum++
				if accum%100 == 0 {
					numZeroes++
				}
			}
		} else {
			for range -val {
				accum--
				if accum%100 == 0 {
					numZeroes++
				}
			}
		}
	}
	return numZeroes
}

func nums(input string) iter.Seq[int] {
	return func(yield func(int) bool) {
		for instr := range strings.FieldsSeq(input) {
			sign := 1
			if instr[0] == 'L' {
				sign = -1
			}

			val, _ := strconv.Atoi(instr[1:])
			if !yield(sign * int(val)) {
				return
			}
		}
	}
}
