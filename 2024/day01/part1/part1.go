package part1

import (
	"math"
	"sort"
)

func Part1(left []int, right []int) int {
	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})
	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})

	// fmt.Printf("Left: %v\n", left)
	// fmt.Printf("Right: %v\n", right)

	sum := 0

	if len(left) != len(right) {
		panic("Left and right slices must be of the same length")
	}

	for i := 0; i < len(left); i++ {
		sum += int(math.Abs(float64(left[i] - right[i])))
	}

	return sum
}
