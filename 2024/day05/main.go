package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/Skylli202/adventOfCode/day05/lib"
)

func main() {
	content, err := os.ReadFile("./day05/input.txt")
	if err != nil {
		panic(err)
	}

	contents := strings.Split(string(content), "\n\n")
	answer1 := solvePart1(contents[0], contents[1])
	fmt.Printf("Part 1: %d\n", answer1)
}

func solvePart1(rawOrderingRules string, rawUpdates string) int {
	ruleBook := lib.NewRuleBook(rawOrderingRules)
	updates := lib.NewUpdatesList(rawUpdates)

	sum := 0
	for _, update := range updates {
		if ruleBook.ValidateUpdates(update) {
			// fmt.Printf("Line %d is valid. Adding middle of %v to sum.\n", i, update)
			sum += int(update.RetrieveMiddlePage())
		}
	}

	return sum
}
