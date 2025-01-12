package lib_test

import (
	"testing"

	"github.com/Skylli202/adventOfCode/day05/lib"
	"github.com/stretchr/testify/assert"
)

var INPUT_ORDERING_RULES = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13`

var INPUT_UPDATES = `75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`

var (
	UPDATE_1 = []lib.Page{75, 47, 61, 53, 29}
	UPDATE_2 = []lib.Page{97, 61, 53, 29, 13}
	UPDATE_3 = []lib.Page{75, 29, 13}
	UPDATE_4 = []lib.Page{75, 97, 47, 61, 53}
	UPDATE_5 = []lib.Page{61, 13, 29}
	UPDATE_6 = []lib.Page{97, 13, 75, 29, 47}
)

func Test_NewRuleBook(t *testing.T) {
	actual := lib.NewRuleBook(INPUT_ORDERING_RULES)
	assert.Len(t, actual, 6)

	p47, ok := actual[47]
	assert.True(t, ok)
	assert.Len(t, p47.Rules, 4)
	assert.Equal(t, []lib.Page{53, 13, 61, 29}, p47.Rules)
}

func Test_ValidateUpdates(t *testing.T) {
	book := lib.NewRuleBook(INPUT_ORDERING_RULES)

	testcases := []struct {
		updates  []lib.Page
		expected bool
	}{
		{updates: UPDATE_1, expected: true},
		{updates: UPDATE_2, expected: true},
		{updates: UPDATE_3, expected: true},
		{updates: UPDATE_4, expected: false},
		{updates: UPDATE_5, expected: false},
		{updates: UPDATE_6, expected: false},
	}

	for _, tc := range testcases {
		actual := book.ValidateUpdates(tc.updates)
		assert.Equal(t, tc.expected, actual)
	}
}

func Test_RetrieveMiddlePage(t *testing.T) {
	testcases := []struct {
		updates  lib.UpdateRow
		expected lib.Page
	}{
		{updates: UPDATE_1, expected: lib.Page(61)},
		{updates: UPDATE_2, expected: lib.Page(53)},
		{updates: UPDATE_3, expected: lib.Page(29)},
		{updates: UPDATE_4, expected: lib.Page(47)},
		{updates: UPDATE_5, expected: lib.Page(13)},
		{updates: UPDATE_6, expected: lib.Page(75)},
	}

	for _, tc := range testcases {
		actual := tc.updates.RetrieveMiddlePage()
		assert.Equal(t, tc.expected, actual)
	}
}

func Test_NewUpdatesList(t *testing.T) {
	actual := lib.NewUpdatesList(INPUT_UPDATES)
	assert.Len(t, actual, 6)

	assert.Equal(t, lib.UpdateRow(UPDATE_1), actual[0])
	assert.Equal(t, lib.UpdateRow(UPDATE_2), actual[1])
	assert.Equal(t, lib.UpdateRow(UPDATE_3), actual[2])
	assert.Equal(t, lib.UpdateRow(UPDATE_4), actual[3])
	assert.Equal(t, lib.UpdateRow(UPDATE_5), actual[4])
	assert.Equal(t, lib.UpdateRow(UPDATE_6), actual[5])
}
