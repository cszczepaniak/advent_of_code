package main

import (
	"context"
	"fmt"
	"iter"
	"net/http"
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

var signs = [...]int{
	'L': -1,
	'R': 1,
}

func nums(input string) iter.Seq[int] {
	return func(yield func(int) bool) {
		for instr := range strings.FieldsSeq(input) {
			sign := signs[instr[0]]

			val := 0
			mul := 1
			for i := len(instr) - 1; i >= 1; i-- {
				val += int(instr[i]-'0') * mul
				mul *= 10
			}

			if !yield(sign * int(val)) {
				return
			}
		}
	}
}
