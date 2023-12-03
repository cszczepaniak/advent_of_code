package common

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIterLines(t *testing.T) {
	tests := []struct {
		desc     string
		input    string
		expLines []string
	}{{
		desc:     `no newline`,
		input:    `a`,
		expLines: []string{`a`},
	}, {
		desc:     `trailing newline`,
		input:    "a\nb\nc\n",
		expLines: []string{`a`, `b`, `c`},
	}, {
		desc:     `no trailing newline`,
		input:    "aa\nbb\nc\ndef",
		expLines: []string{`aa`, `bb`, `c`, `def`},
	}}

	for _, tc := range tests {
		t.Run(tc.desc, func(t *testing.T) {
			res := make([]string, 0)
			for s := range IterLines(tc.input) {
				res = append(res, s)
			}
			assert.Equal(t, tc.expLines, res)
		})
	}
}

func TestIterSplitString(t *testing.T) {
	tests := []struct {
		desc     string
		input    string
		delim    byte
		expLines []string
	}{{
		desc:     `no delim`,
		input:    `a`,
		delim:    ':',
		expLines: []string{`a`},
	}, {
		// This probably should include an empty string at the end, but I don't want to do that
		desc:     `trailing delim`,
		input:    "a:b:c:",
		delim:    ':',
		expLines: []string{`a`, `b`, `c`},
	}, {
		desc:     `no trailing delim`,
		input:    "aa;bb;c;def",
		delim:    ';',
		expLines: []string{`aa`, `bb`, `c`, `def`},
	}}

	for _, tc := range tests {
		t.Run(tc.desc, func(t *testing.T) {
			res := make([]string, 0)
			for s := range IterSplitString(tc.input, tc.delim) {
				res = append(res, s)
			}
			assert.Equal(t, tc.expLines, res)
		})
	}
}
