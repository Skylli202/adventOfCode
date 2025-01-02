package lib

import (
	"errors"
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

func (d Direction) ToCoord() Coord {
	return Coord(d)
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

			for _, d := range DIRECTIONS {
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

	return []rune(g[c.L])[c.C], nil
}

func (c Coord) Add(o Coord) Coord {
	return Coord{c.L + o.L, c.C + o.C}
}