package internal

import "fmt"

const (
	// NOTE: Hands types
	FIVE_OF_A_KIND  = 7
	FOUR_OF_A_KIND  = 6
	FULL_HOUSE      = 5
	THREE_OF_A_KIND = 4
	TWO_PAIR        = 3
	ONE_PAIR        = 2
	HIGH_CARD       = 1
)

type Hand struct {
	Cards [5]Card
}

func (*Hand) New() *Hand {
	h := &Hand{
		Cards: [5]Card{},
	}
	return h
}

func FromString(str string) *Hand {
	if len(str) != 5 {
		panic(fmt.Sprintf("String representation of an Hand is 5 cards. Each Card are represented by a single Character (A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2). Got: %q", str))
	}

	h := &Hand{
		Cards: [5]Card{
			{Label: string(str[0])},
			{Label: string(str[1])},
			{Label: string(str[2])},
			{Label: string(str[3])},
			{Label: string(str[4])},
		},
	}
	return h
}

func (*Hand) ComputeType() int {
	return ONE_PAIR
}
