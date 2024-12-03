package part2

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

var (
	case_one   []int = []int{7, 6, 4, 2, 1}
	case_two   []int = []int{1, 2, 7, 8, 9}
	case_three []int = []int{9, 7, 6, 2, 1}
	case_four  []int = []int{1, 3, 2, 4, 5}
	case_five  []int = []int{8, 6, 4, 4, 1}
	case_six   []int = []int{1, 3, 6, 7, 9}

	case_seven []int = []int{1, 1, 2, 3, 4}
)

func Test_Damp(t *testing.T) {
	input := []int{1, 2, 3, 4}
	expected := [][]int{
		{2, 3, 4},
		{1, 3, 4},
		{1, 2, 4},
		{1, 2, 3},
	}

	assert.Equal(t, expected, Damp(input))
}

func Test_Part2(t *testing.T) {
	assert.True(t, IsReportSafeDampener(case_one), "case_one")
	assert.False(t, IsReportSafeDampener(case_two), "case_two")
	assert.False(t, IsReportSafeDampener(case_three), "case_three")
	assert.True(t, IsReportSafeDampener(case_four), "case_four")
	assert.True(t, IsReportSafeDampener(case_five), "case_five")
	assert.True(t, IsReportSafeDampener(case_six), "case_six")
}
