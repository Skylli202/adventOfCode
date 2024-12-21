package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

type Multiplication struct {
	a, b int
}

func (m Multiplication) Product() int {
	return m.a * m.b
}

func main() {
	b, _ := os.ReadFile("./day03/input.txt")
	s := string(b)

	r := regexp.MustCompile(`mul\((\d{1,3}),(\d{1,3})\)`)
	ops := r.FindAllStringSubmatch(s, -1)
	res := make([]Multiplication, 0, len(ops))
	for _, op := range ops {
		a, _ := strconv.Atoi(op[1])
		b, _ := strconv.Atoi(op[2])
		res = append(res, Multiplication{a, b})
	}

	result := 0
	for _, m := range res {
		result += m.Product()
	}
	fmt.Printf("Part 1: %d\n", result)
}
