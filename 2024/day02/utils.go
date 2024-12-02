package main

import (
	"os"
	"strings"
)

func readInput1() []string {
	return readInput("./inputs/part1.txt")
}

func readInput(path string) []string {
	content, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(content), "\n")
	return lines
}
