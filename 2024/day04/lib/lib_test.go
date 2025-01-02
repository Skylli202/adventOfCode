package lib_test

import (
	"testing"

	"github.com/Skylli202/adventOfCode/day04/lib"
	"github.com/stretchr/testify/assert"
)

var example = lib.Grid{
	"MMMSXXMASM",
	"MSAMXMSMSA",
	"AMXSXMAAMM",
	"MSAMASMSMX",
	"XMASAMXAMM",
	"XXAMMXXAMA",
	"SMSMSASXSS",
	"SAXAMASAAA",
	"MAMMMXMMMM",
	"MXMXAXMASX",
}

func Test_GridOutOfBound(t *testing.T) {
	tCases := []lib.Coord{
		{-1, 0},
		{0, -1},
		{len(example), 0},
		{0, len(example[0])},
	}
	for _, tc := range tCases {
		_, err := example.ChatAtPos(tc)
		assert.EqualError(t, err, "out of bounds")
	}
}

func Test_CoordAdd(t *testing.T) {
	c := lib.Coord{3, 3}
	tCases := []struct {
		c        lib.Coord
		expected lib.Coord
	}{
		{
			c:        lib.Coord{0, 0},
			expected: lib.Coord{3, 3},
		},
		{
			c:        lib.Coord{1, 0},
			expected: lib.Coord{4, 3},
		},
		{
			c:        lib.Coord{-1, 0},
			expected: lib.Coord{2, 3},
		},
		{
			c:        lib.Coord{0, 1},
			expected: lib.Coord{3, 4},
		},
		{
			c:        lib.Coord{0, -1},
			expected: lib.Coord{3, 2},
		},
		{
			c:        lib.Coord{1, 1},
			expected: lib.Coord{4, 4},
		},
		{
			c:        lib.Coord{-1, -1},
			expected: lib.Coord{2, 2},
		},
	}

	for _, tc := range tCases {
		actual := c.Add(tc.c)
		assert.Equal(t, tc.expected, actual)
	}
}
