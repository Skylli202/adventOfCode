package internal_test

import (
	"testing"

	"github.com/Skylli202/adventOfCode/internal"
	"github.com/stretchr/testify/assert"
)

func TestHandFromString(t *testing.T) {
	testCases := []struct {
		Expected *internal.Hand
		Input    string
	}{
		{
			Input: "32T3K",
			Expected: &internal.Hand{Cards: [5]internal.Card{
				{Label: "3"},
				{Label: "2"},
				{Label: "T"},
				{Label: "3"},
				{Label: "K"},
			}},
		},
	}

	for _, tc := range testCases {
		assert.Equal(t, tc.Expected, internal.FromString(tc.Input))
	}

	{
		testCases := []string{"", "123", "123456"}
		for _, tc := range testCases {
			assert.Panics(t, func() {
				internal.FromString(tc)
			}, "internal.FromString did not panic when given a bad input")
		}
	}
}

func TestDetectHandType(t *testing.T) {
	testCases := []struct {
		Hand     *internal.Hand
		Expected int
	}{
		{Hand: internal.FromString("32T3K"), Expected: internal.ONE_PAIR},
		{Hand: internal.FromString("T55J5"), Expected: internal.THREE_OF_A_KIND},
		{Hand: internal.FromString("KK677"), Expected: internal.TWO_PAIR},
		{Hand: internal.FromString("KTJJT"), Expected: internal.TWO_PAIR},
		{Hand: internal.FromString("QQQJA"), Expected: internal.THREE_OF_A_KIND},
	}

	for _, tc := range testCases {
		assert.Equal(t, tc.Expected, tc.Hand.ComputeType())
	}
}
