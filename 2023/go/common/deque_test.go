package common

import (
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDeque(t *testing.T) {
	d := NewDeque[int]()

	assertContents := func(d *Deque[int], vals ...int) {
		t.Helper()

		if len(vals) == 0 {
			assert.Empty(t, d.Collect())
			assert.Empty(t, Collect(d.IterateBackwards()))
			assert.Zero(t, d.Len())
			return
		}

		expRev := slices.Clone(vals)
		slices.Reverse(expRev)

		assert.Equal(t, vals, d.Collect())
		assert.Equal(t, expRev, Collect(d.IterateBackwards()))
		assert.Equal(t, len(vals), d.Len())
	}

	d.PushLeft(1)
	assertContents(d, 1)

	d.PushLeft(2)
	assertContents(d, 2, 1)

	d.PushRight(3)
	assertContents(d, 2, 1, 3)

	assert.Equal(t, 2, d.PeekLeft())
	assertContents(d, 2, 1, 3)

	assert.Equal(t, 2, d.PopLeft())
	assertContents(d, 1, 3)

	assert.Equal(t, 3, d.PeekRight())
	assertContents(d, 1, 3)

	assert.Equal(t, 3, d.PopRight())
	assertContents(d, 1)

	assert.Equal(t, 1, d.PopRight())
	assertContents(d)

	d.PushLeft(2)
	assertContents(d, 2)

	assert.Equal(t, 2, d.PopLeft())
	assertContents(d)

	assert.Panics(t, func() {
		d.PeekLeft()
	})

	assert.Panics(t, func() {
		d.PopLeft()
	})

	assert.Panics(t, func() {
		d.PeekRight()
	})

	assert.Panics(t, func() {
		d.PopRight()
	})
}
