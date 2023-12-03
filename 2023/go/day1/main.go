package main

import (
	"bufio"
	"slices"
	"strings"
	"sync"
	"sync/atomic"
	"unicode"

	"github.com/cszczepaniak/advent_of_code/2023/go/common"
	"github.com/cszczepaniak/go-aoc/aoc"
)

func main() {
	err := aoc.Main(
		2023, 1,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

func part1(input string) int {
	var n atomic.Int32

	wg := &sync.WaitGroup{}
	wg.Add(2)

	// Search for the tens digits and ones digits concurrently.
	go func() {
		defer wg.Done()
		n.Add(iterLines(input, iterString, 10))
	}()

	go func() {
		defer wg.Done()
		n.Add(iterLines(input, iterStringReverse, 1))
	}()

	wg.Wait()

	return int(n.Load())
}

func iterLines(
	text string,
	makeIter func(string) common.Seq1[rune],
	mult int,
) int32 {
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
	return int32(n)
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
