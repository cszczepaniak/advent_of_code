package main

import (
	"context"
	"fmt"
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

	solve(`L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
`)
	solve(input)
}

func solve(input string) {
	accum := 50
	numZeroes := 0
	for instr := range strings.FieldsSeq(input) {
		sign := 1
		if instr[0] == 'L' {
			sign = -1
		}

		val, _ := strconv.Atoi(instr[1:])
		accum += sign * int(val)
		if accum%100 == 0 {
			numZeroes++
		}
	}
	fmt.Println(numZeroes)
}
