package main

import (
	"bufio"
	"context"
	"fmt"
	"net/http"
	"slices"
	"strconv"
	"strings"
	"unicode"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	day1 := aoc.NewRequest(2023, 1)
	input, err := aoc.GetInputString(
		context.Background(),
		http.DefaultClient,
		day1.BuildGetInputRequest(),
	)
	if err != nil {
		panic(err)
	}

	ans1 := part1(input)

	err = aoc.SubmitAnswer(
		context.Background(),
		http.DefaultClient,
		day1.BuildSubmitAnswerRequest(aoc.AnswerPartOne, strconv.Itoa(ans1)),
	)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(`part 1 complete!`)
	}

	ans2 := part2(input)

	err = aoc.SubmitAnswer(
		context.Background(),
		http.DefaultClient,
		day1.BuildSubmitAnswerRequest(aoc.AnswerPartTwo, strconv.Itoa(ans2)),
	)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(`part 2 complete!`)
	}
}

func part1(input string) int {
	ch := make(chan int)
	defer close(ch)

	// Search for the tens digits and ones digits concurrently.
	go iterLines(ch, input, iterString, 10)
	go iterLines(ch, input, iterStringReverse, 1)

	res := 0
	for range 2 {
		n := <-ch
		res += n
	}

	return res
}

func iterLines(
	ch chan<- int,
	text string,
	makeIter func(string) common.Seq1[rune],
	mult int,
) {
	n := 0

	sc := bufio.NewScanner(strings.NewReader(text))
	for sc.Scan() {
		for r := range makeIter(sc.Text()) {
			if unicode.IsDigit(r) {
				n += mult * int((r - '0'))
				break
			}
		}
	}

	ch <- n
}

// iterString isn't strictly necessary because strings are already iterable, but it makes for a homogenous interface
// with the reverse iterator.
func iterString(str string) common.Seq1[rune] {
	return func(yield func(rune) bool) {
		for _, r := range str {
			if !yield(r) {
				return
			}
		}
	}
}

func iterStringReverse(str string) common.Seq1[rune] {
	return func(yield func(rune) bool) {
		rs := []rune(str)
		for i := len(rs) - 1; i >= 0; i-- {
			if !yield(rs[i]) {
				return
			}
		}
	}
}

func part2(input string) int {
	num := 0

	sc := bufio.NewScanner(strings.NewReader(input))
	for sc.Scan() {
		tens := 10 * matchFirstDigit(sc.Text(), digits)

		rs := []rune(sc.Text())
		slices.Reverse(rs)
		ones := matchFirstDigit(string(rs), reverseDigits)

		num += tens + ones
	}

	return num
}

var digits = map[string]int{
	`one`:   1,
	`two`:   2,
	`three`: 3,
	`four`:  4,
	`five`:  5,
	`six`:   6,
	`seven`: 7,
	`eight`: 8,
	`nine`:  9,
}

var reverseDigits = map[string]int{
	`eno`:   1,
	`owt`:   2,
	`eerht`: 3,
	`ruof`:  4,
	`evif`:  5,
	`xis`:   6,
	`neves`: 7,
	`thgie`: 8,
	`enin`:  9,
}

func matchFirstDigit(str string, digits map[string]int) int {
	for i := 0; i < len(str); i++ {
		if unicode.IsDigit(rune(str[i])) {
			return int(rune(str[i]) - '0')
		}
		for d, num := range digits {
			n, ok := tryToMatchWordAsDigit(str[i:], d, num)
			if ok {
				return n
			}
		}
	}

	panic(`malformed input!`)
}

func tryToMatchWordAsDigit(str, digit string, ifMatch int) (int, bool) {
	if digit == `` {
		return ifMatch, true
	}

	if str[0] == digit[0] {
		return tryToMatchWordAsDigit(str[1:], digit[1:], ifMatch)
	}

	return 0, false
}
