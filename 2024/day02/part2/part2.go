package part2

import (
	"aoc202402/part1"
)

func IsReportSafeDampener(l []int) bool {
	for _, dampened := range Damp(l) {
		if part1.IsReportSafe(dampened) {
			return true
		}
	}
	return false
}

func Damp(l []int) [][]int {
	var res [][]int

	for i := range l {
		res = append(res, RemoveIndex(l, i))
	}

	return res
}

func RemoveIndex[T any](slice []T, index int) []T {
	ret := make([]T, 0)
	ret = append(ret, slice[:index]...)
	return append(ret, slice[index+1:]...)
}
