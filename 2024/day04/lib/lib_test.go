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
