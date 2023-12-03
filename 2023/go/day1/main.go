package main

import (
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
	return solution(input, matchDigit, matchDigitRight)
}

func part2(input string) int {
	return solution(
		input,
		matchFirst(matchDigit, matchAlphabeticalDigit(strings.HasPrefix)),
		matchFirst(matchDigitRight, matchAlphabeticalDigit(strings.HasSuffix)),
	)
}

func solution(input string, matchDigitLeft, matchDigitRight func(string) int) int {
	var n atomic.Int32

	wg := &sync.WaitGroup{}
	wg.Add(2)

	// Search for the tens digits and ones digits concurrently.
	go func() {
		defer wg.Done()
		n.Add(iterLines(input, iterString, 10, matchDigitLeft))
	}()

	go func() {
		defer wg.Done()
		n.Add(iterLines(input, iterStringRight, 1, matchDigitRight))
	}()

	wg.Wait()

	return int(n.Load())
}

func matchDigit(str string) int {
	for _, r := range str {
		if unicode.IsDigit(r) {
			return int((r - '0'))
		}
		break
	}
	return 0
}

func matchDigitRight(str string) int {
	if len(str) == 0 {
		return 0
	}

	rs := []rune(str)
	r := rs[len(rs)-1]
	if unicode.IsDigit(r) {
		return int((r - '0'))
	}

	return 0
}

func matchSpecificAlphabeticalDigit(hasPrefix func(string, string) bool, prefix string, num int) func(string) int {
	return func(s string) int {
		if hasPrefix(s, prefix) {
			return num
		}
		return 0
	}
}

func matchAlphabeticalDigit(hasPrefix func(string, string) bool) func(string) int {
	return matchFirst(
		matchSpecificAlphabeticalDigit(hasPrefix, `one`, 1),
		matchSpecificAlphabeticalDigit(hasPrefix, `two`, 2),
		matchSpecificAlphabeticalDigit(hasPrefix, `three`, 3),
		matchSpecificAlphabeticalDigit(hasPrefix, `four`, 4),
		matchSpecificAlphabeticalDigit(hasPrefix, `five`, 5),
		matchSpecificAlphabeticalDigit(hasPrefix, `six`, 6),
		matchSpecificAlphabeticalDigit(hasPrefix, `seven`, 7),
		matchSpecificAlphabeticalDigit(hasPrefix, `eight`, 8),
		matchSpecificAlphabeticalDigit(hasPrefix, `nine`, 9),
	)
}

func matchFirst(ms ...func(string) int) func(string) int {
	return func(s string) int {
		for _, m := range ms {
			if n := m(s); n > 0 {
				return n
			}
		}
		return 0
	}
}

func iterLines(
	text string,
	makeIter func(string) common.Seq1[string],
	mult int,
	matcher func(string) int,
) int32 {
	n := 0
	for str := range common.IterLines(text) {
		for s := range makeIter(str) {
			if dig := matcher(s); dig > 0 {
				n += mult * dig
				break
			}
		}
	}
	return int32(n)
}

// iterString produces a sequence of strings that shrink the given string from the left.
// For "abc", this will produce "abc", "bc", "c", ""
func iterString(str string) common.Seq1[string] {
	return func(yield func(string) bool) {
		for i := range len(str) {
			if !yield(str[i:]) {
				return
			}
		}
	}
}

// iterString produces a sequence of strings that shrink the given string from the right.
// For "abc", this will produce "abc", "ab", "a", ""
func iterStringRight(str string) common.Seq1[string] {
	return func(yield func(string) bool) {
		for i := range len(str) {
			if !yield(str[:len(str)-i]) {
				return
			}
		}
	}
}
