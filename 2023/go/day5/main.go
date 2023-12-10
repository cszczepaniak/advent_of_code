package main

import (
	"github.com/cszczepaniak/advent_of_code/2023/go/common/parsing"
	"github.com/cszczepaniak/go-aoc/aoc"
	"math"
	"strings"
)

func main() {
	err := aoc.Main(
		2023, 5,
		part1, part2,
		aoc.WithDefaultHTTPClient(),
	)
	if err != nil {
		panic(err)
	}
}

/*
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/

type rng struct {
	// start is inclusive.
	start int
	// end is inclusive.
	end int
}

func newRange(start, end int) rng {
	return rng{
		start: start,
		end:   end,
	}
}

func (r rng) length() int {
	return r.end - r.start + 1
}

func (r rng) contains(i int) bool {
	return i >= r.start && i <= r.end
}

func (r rng) overlaps(other rng) bool {
	return r.contains(other.start) || r.contains(other.end)
}

type mappingRng struct {
	source    rng
	destStart int
}

// destEnd is inclusive.
func (mr mappingRng) destEnd() int {
	return mr.destStart + mr.source.length() - 1
}

type seedRange struct {
	rng

	wasTransformed bool
}

func newSeedRange(start, end int) seedRange {
	return seedRange{
		rng: newRange(start, end),
	}
}

func (sr seedRange) length() int {
	return sr.end - sr.start + 1
}

func part1(input string) int {
	var seeds []seedRange
	input = strings.TrimPrefix(input, `seeds: `)

	var n int
	var err error
	for input[0] != '\n' {
		n, input, err = parsing.ParseNumber(input)
		if err != nil {
			panic(err)
		}

		input = parsing.DiscardSpaces(input)

		seeds = append(seeds, newSeedRange(n, n))
	}

	// Discard the newline and the next blank line.
	input = parsing.DiscardLine(input)
	input = parsing.DiscardLine(input)

	minVal, err := processMappings(input, seeds)
	if err != nil {
		panic(err)
	}
	return minVal
}

func part2(input string) int {
	var seeds []seedRange
	input = strings.TrimPrefix(input, `seeds: `)

	for input[0] != '\n' {
		start, newInput, err := parsing.ParseNumber(input)
		if err != nil {
			panic(err)
		}
		newInput = parsing.DiscardSpaces(newInput)

		l, newInput, err := parsing.ParseNumber(newInput)
		if err != nil {
			panic(err)
		}
		newInput = parsing.DiscardSpaces(newInput)

		input = newInput

		seeds = append(seeds, newSeedRange(start, start+l-1))
	}

	// Discard the newline and the next blank line.
	input = parsing.DiscardLine(input)
	input = parsing.DiscardLine(input)

	minVal, err := processMappings(input, seeds)
	if err != nil {
		panic(err)
	}
	return minVal
}

func processMappings(input string, seeds []seedRange) (int, error) {
	// There are 7 total mappings.
	for range 7 {
		// Discard the title line.
		input = parsing.DiscardLine(input)

		var m mappingRng
		var err error
		// mapping := make(map[int]int)
		for {
			m, input, err = parseMappingLine(input)
			if err != nil {
				return 0, err
			}

			for j, s := range seeds {
				if s.wasTransformed {
					continue
				}

				newSeed, extras := mapSeedRange(s, m)
				seeds[j] = newSeed
				seeds = append(seeds, extras...)
			}

			if len(input) == 0 {
				break
			}
			if input[0] == '\n' {
				// Discard the trailing empty line.
				input = parsing.DiscardLine(input)

				// Reset "wasUpdated"
				for j, s := range seeds {
					s.wasTransformed = false
					seeds[j] = s
				}
				break
			}
		}
	}

	minVal := math.MaxInt
	for _, s := range seeds {
		minVal = min(minVal, s.start)
	}
	return minVal, nil
}

// mapSeedRange takes a seed range and a mapping range, finds the overlap between the two ranges, and returns the
// original seed range plus any additional seed ranges created. For example, if the seed has range 1-5 and the mapping
// has range 2-4, the seed range will be split in three; 1 is unshifted, 2-4 is shifted by the mapping range's length,
// and 5 is also unshifted. 1 and 5 are returned in the slice.
func mapSeedRange(sr seedRange, mr mappingRng) (seedRange, []seedRange) {
	transformedSeed := sr
	var extras []seedRange

	if mr.source.overlaps(sr.rng) {
		if sr.start < mr.source.start {
			ex := newSeedRange(sr.start, mr.source.start-1)
			ex.wasTransformed = false

			extras = append(extras, ex)
		}
		if sr.end > mr.source.end {
			ex := newSeedRange(mr.source.end-1, sr.end)
			ex.wasTransformed = false

			extras = append(extras, ex)
		}

		originalStart := max(sr.start, mr.source.start)
		originalEnd := min(sr.end, mr.source.end)

		diff := mr.destStart - mr.source.start

		transformedSeed = newSeedRange(originalStart+diff, originalEnd+diff)
		transformedSeed.wasTransformed = true
	}

	return transformedSeed, extras
}

func parseMappingLine(input string) (mappingRng, string, error) {
	destStart, input, err := parsing.ParseNumber(input)
	if err != nil {
		panic(err)
	}
	input = parsing.DiscardSpaces(input)

	srcStart, input, err := parsing.ParseNumber(input)
	if err != nil {
		panic(err)
	}
	input = parsing.DiscardSpaces(input)

	length, input, err := parsing.ParseNumber(input)
	if err != nil {
		panic(err)
	}

	return mappingRng{
		source:    newRange(srcStart, srcStart+length-1),
		destStart: destStart,
	}, parsing.DiscardLine(input), nil
}
