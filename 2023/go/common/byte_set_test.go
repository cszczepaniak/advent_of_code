package common

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestByteSet(t *testing.T) {
	s := NewByteSet()

	nums := map[byte]struct{}{
		23:  {},
		64:  {},
		122: {},
		177: {},
		200: {},
		255: {},
	}
	for n := range nums {
		s.Insert(n)
	}

	i := byte(0)
	for {
		_, shouldContain := nums[i]
		assert.Equal(t, shouldContain, s.Contains(i), i)
		if i == 255 {
			break
		}
		i++
	}
}
