package main

import (
	"aoc202402/part1"
	"fmt"
	"strconv"
	"strings"
)

func main() {
	part1Exec()
}

func part1Exec() {
	lines := readInput1()

	reports := make([][]int, 0, len(lines))
	for i, l := range lines {
		if l == "" {
			continue
		}

		levelsStr := strings.Fields(l)
		levels := make([]int, 0, len(levelsStr))
		for _, ll := range levelsStr {
			level, err := strconv.Atoi(ll)
			if err != nil {
				panic(fmt.Sprintf("err while Atoi %s at row %d - %v\n", ll, i, err))
			}
			levels = append(levels, level)
		}

		reports = append(reports, levels)
	}

	safeReportCounter := 0
	for _, r := range reports {
		if part1.IsReportSafe(r) {
			safeReportCounter++
		}
	}

	fmt.Printf("%d report are safe.\n", safeReportCounter)
}
