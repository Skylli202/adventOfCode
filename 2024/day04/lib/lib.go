package lib

import (
	"errors"
	"fmt"
)

var (
	DIRECTIONS = []Direction{DOWN_LEFT, LEFT, UP_LEFT, UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN}
	UP         = Direction{-1, 0}
	DOWN       = Direction{1, 0}
	LEFT       = Direction{0, -1}
	RIGHT      = Direction{0, 1}

	DOWN_LEFT  = Direction{1, -1}
	UP_LEFT    = Direction{-1, -1}
	DOWN_RIGHT = Direction{1, 1}
	UP_RIGHT   = Direction{-1, 1}
)

// A direction
// Under the hood its just a Coord.
// This package export all the possible directions as const
// ↖ ↑ ↗
// ← x →
// ↙ ↓ ↘
type Direction Coord

type (
	Coord struct{ L, C int }
	Grid  []string
)

var debug = false

func (d Direction) ToCoord() Coord {
	return Coord(d)
}

func (g Grid) FindCross() []Coord {
	res := make([]Coord, 0)

	head := 'A'
	// Both diagonals of the cross can be written forward, or reversed: 4 scenarios
	// MAS MAS
	// MAS SAM
	// SAM MAS
	// SAM SAM
	scenarios := [][]struct {
		direction Direction
		expected  rune
	}{
		// MAS MAS
		{
			{direction: UP_LEFT, expected: 'M'},
			{direction: DOWN_RIGHT, expected: 'S'},
			{direction: UP_RIGHT, expected: 'M'},
			{direction: DOWN_LEFT, expected: 'S'},
		},
		// MAS SAM
		{
			{direction: UP_LEFT, expected: 'M'},
			{direction: DOWN_RIGHT, expected: 'S'},
			{direction: UP_RIGHT, expected: 'S'},
			{direction: DOWN_LEFT, expected: 'M'},
		},
		// SAM MAS
		{
			{direction: UP_LEFT, expected: 'S'},
			{direction: DOWN_RIGHT, expected: 'M'},
			{direction: UP_RIGHT, expected: 'M'},
			{direction: DOWN_LEFT, expected: 'S'},
		},
		// SAM SAM
		{
			{direction: UP_LEFT, expected: 'S'},
			{direction: DOWN_RIGHT, expected: 'M'},
			{direction: UP_RIGHT, expected: 'S'},
			{direction: DOWN_LEFT, expected: 'M'},
		},
	}

	for l, line := range g {
		for c, col := range line {
			if col != head {
				continue
			}

			for _, scenario := range scenarios {
				correctRuneCounter := 0
				for _, tc := range scenario {
					rune, err := g.ChatAtCoord(Coord{l, c}.Add(tc.direction.ToCoord()))
					if err != nil {
						continue
					}
					if rune != tc.expected {
						break
					}
					correctRuneCounter += 1
				}

				if correctRuneCounter == len(scenarios) {
					res = append(res, Coord{l, c})
					break
				}
			}
		}
	}

	return res
}

func (g Grid) Find(word string) []Coord {
	res := make([]Coord, 0)

	head := rune(word[0])
	remaining := word[1:]

	for l, line := range g {
		for c, col := range line {
			if col != head {
				continue
			}

			if debug {
				fmt.Printf("%#U found at Coord{%d, %d}\n", head, l, c)
				if l-1 >= 0 {
					fmt.Printf("%s\n", g[l-1])
				}
				fmt.Printf("%s\n", line)
				if l+1 < len(g) {
					fmt.Printf("%s\n", g[l+1])
				}
			}

			for _, d := range DIRECTIONS {
				if debug {
					fmt.Printf("\tReading in direction: %#v\n", d)
				}
				if g.ReadAtCoord(Coord{l, c}, d, len(remaining)) == remaining {
					res = append(res, Coord{l, c})
				}
			}
		}
	}

	return res
}

// NOTE: Watch out Grid position is not like Maths (x,y)
// It is (line, column) which correspond to Maths (-y,x).
// (0,0) is the first character at the top left of the grid.
func (g Grid) ReadAtCoord(coord Coord, direction Direction, length int) string {
	res := ""

	for range make([]int, length) {
		coord = coord.Add(direction.ToCoord())
		v, err := g.ChatAtCoord(coord)
		if err != nil {
			return ""
		}
		res += string(v)
	}

	return res
}

func (g Grid) ChatAtCoord(c Coord) (rune, error) {
	if c.L < 0 || c.C < 0 || c.L >= len(g) || c.C >= len(g[0]) {
		return 0, errors.New("out of bounds")
	}

	if debug {
		fmt.Printf("%#v\n", c)
	}

	return []rune(g[c.L])[c.C], nil
}

func (c Coord) Add(o Coord) Coord {
	return Coord{c.L + o.L, c.C + o.C}
}
