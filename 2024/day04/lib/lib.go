package lib

import (
	"errors"
)

type (
	Coord struct{ L, C int }
	Grid  []string
)

func (g Grid) ChatAtCoord(c Coord) (rune, error) {
	if c.L < 0 || c.C < 0 || c.L >= len(g) || c.C >= len(g[0]) {
		return 0, errors.New("out of bounds")
	}

	return []rune(g[c.L])[c.C], nil
}

func (c Coord) Add(o Coord) Coord {
	return Coord{c.L + o.L, c.C + o.C}
}
