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

func Test_GridReadAtCoord(t *testing.T) {
	tests := []struct {
		expected  string
		coord     lib.Coord
		direction lib.Direction
	}{
		{coord: lib.Coord{0, 0}, direction: lib.DOWN_LEFT, expected: ""},
		{coord: lib.Coord{0, 0}, direction: lib.LEFT, expected: ""},
		{coord: lib.Coord{0, 0}, direction: lib.UP_LEFT, expected: ""},
		{coord: lib.Coord{0, 0}, direction: lib.UP, expected: ""},
		{coord: lib.Coord{0, 0}, direction: lib.UP_RIGHT, expected: ""},
		{coord: lib.Coord{0, 0}, direction: lib.RIGHT, expected: "MMS"},
		{coord: lib.Coord{0, 0}, direction: lib.DOWN_RIGHT, expected: "SXM"},
		{coord: lib.Coord{0, 0}, direction: lib.DOWN, expected: "MAM"},

		{coord: lib.Coord{4, 5}, direction: lib.DOWN_LEFT, expected: "MMX"},
		{coord: lib.Coord{4, 5}, direction: lib.LEFT, expected: "ASA"},
		{coord: lib.Coord{4, 5}, direction: lib.UP_LEFT, expected: "ASA"},
		{coord: lib.Coord{4, 5}, direction: lib.UP, expected: "SMM"},
		{coord: lib.Coord{4, 5}, direction: lib.UP_RIGHT, expected: "MAS"},
		{coord: lib.Coord{4, 5}, direction: lib.RIGHT, expected: "XAM"},
		{coord: lib.Coord{4, 5}, direction: lib.DOWN_RIGHT, expected: "XXA"},
		{coord: lib.Coord{4, 5}, direction: lib.DOWN, expected: "XAA"},
	}

	for _, tc := range tests {
		actual := example.ReadAtCoord(tc.coord, tc.direction)
		assert.Equal(t, tc.expected, actual)
	}
}

func Test_GridOutOfBound(t *testing.T) {
	tests := []lib.Coord{
		{-1, 0},
		{0, -1},
		{len(example), 0},
		{0, len(example[0])},
	}
	for _, tc := range tests {
		_, err := example.ChatAtCoord(tc)
		assert.EqualError(t, err, "out of bounds")
	}
}

func Test_GridCharAtCoord(t *testing.T) {
	tests := []struct {
		coord    lib.Coord
		expected rune
	}{
		{
			coord:    lib.Coord{0, 0},
			expected: 'M',
		},
		{
			coord:    lib.Coord{3, 7},
			expected: 'S',
		},
	}

	for _, tc := range tests {
		actual, err := example.ChatAtCoord(tc.coord)
		assert.Nil(t, err, "Unexpected error")
		assert.Equal(t, tc.expected, actual)
	}
}

func Test_CoordAdd(t *testing.T) {
	c := lib.Coord{3, 3}
	tests := []struct {
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

	for _, tc := range tests {
		actual := c.Add(tc.c)
		assert.Equal(t, tc.expected, actual)
	}
}
