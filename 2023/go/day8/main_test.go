package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPrimeFactors(t *testing.T) {
	assert.Equal(t, []int{2, 2, 2}, primeFactors(8))
	assert.Equal(t, []int{2, 2, 3}, primeFactors(12))
	assert.Equal(t, []int{2, 2, 3, 5, 7, 7, 7}, primeFactors(20580))
}

func TestLeastCommonMultiple(t *testing.T) {
	assert.Equal(t, 2, leastCommonMultiple(1, 2))
	assert.Equal(t, 6, leastCommonMultiple(1, 2, 3))
	assert.Equal(t, 12, leastCommonMultiple(1, 2, 3, 4))
	assert.Equal(t, 3000, leastCommonMultiple(12, 24, 60, 1000))
}
