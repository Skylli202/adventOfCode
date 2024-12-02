package part1

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	result := Part1([]int{3, 4, 2, 1, 3, 3}, []int{4, 3, 5, 3, 9, 3})
	assert.Equal(t, result, 11)
}
