package main

import (
	"aoc202401/part1"
	"aoc202401/part2"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1Exec()
	part2Exec()
}

func part1Exec() {
	content, err := os.ReadFile("./inputs/part1.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	left := make([]int, 0)
	right := make([]int, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		// fmt.Printf("parts: %v\n", parts)
		if len(parts) != 2 {
			continue
		}

		leftNum, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}
		rightNum, err := strconv.Atoi(parts[1])
		if err != nil {
			panic(err)
		}

		left = append(left, leftNum)
		right = append(right, rightNum)
	}

	result := part1.Part1(left, right)
	fmt.Printf("Result: %d\n", result)
}

func part2Exec() {
	content, err := os.ReadFile("./inputs/part1.txt")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	left := make([]int, 0)
	right := make([]int, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		// fmt.Printf("parts: %v\n", parts)
		if len(parts) != 2 {
			continue
		}

		leftNum, err := strconv.Atoi(parts[0])
		if err != nil {
			panic(err)
		}
		rightNum, err := strconv.Atoi(parts[1])
		if err != nil {
			panic(err)
		}

		left = append(left, leftNum)
		right = append(right, rightNum)
	}

	result := part2.Part2(left, right)
	fmt.Printf("Result: %d\n", result)
}
