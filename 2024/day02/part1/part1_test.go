package part1

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
)

func Test_Part1(t *testing.T) {
	assert.True(t, IsReportSafe(case_one), "case_one")
	assert.False(t, IsReportSafe(case_two), "case_two")
	assert.False(t, IsReportSafe(case_three), "case_three")
	assert.False(t, IsReportSafe(case_four), "case_four")
	assert.False(t, IsReportSafe(case_five), "case_five")
	assert.True(t, IsReportSafe(case_six), "case_six")
}

func Test_isIncreasingReportSafe(t *testing.T) {
	assert.False(t, isIncreasingReportSafe(case_two), "case_two")
	assert.False(t, isIncreasingReportSafe(case_four), "case_four")
	assert.True(t, isIncreasingReportSafe(case_six), "case_six")
}

func Test_isDecreaseingReportSafe(t *testing.T) {
	assert.True(t, isDecreasingReportSafe(case_one), "case_one")
	assert.False(t, isDecreasingReportSafe(case_three), "case_three")
	assert.False(t, isDecreasingReportSafe(case_five), "case_five")
}

func Test_IsIncreasing(t *testing.T) {
	assert.False(t, IsIncreasing(case_one), "case_one")
	assert.True(t, IsIncreasing(case_two), "case_two")
	assert.False(t, IsIncreasing(case_three), "case_three")
	assert.True(t, IsIncreasing(case_four), "case_four")
	assert.False(t, IsIncreasing(case_five), "case_five")
	assert.True(t, IsIncreasing(case_six), "case_six")
}
