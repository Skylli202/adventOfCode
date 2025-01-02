package main

import (
	"log"
	"os"
	"strings"

	"github.com/Skylli202/adventOfCode/day04/lib"
)

func main() {
	log.Println("Day 04:")

	content, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(content), "\n")
	grid := lib.Grid(lines)

	log.Printf("Solution to part 1: %d", len(grid.Find("XMAS")))
	log.Printf("Solution to part 2: %d", len(grid.FindCross()))
}
