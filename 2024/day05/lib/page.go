package lib

import (
	"fmt"
	"slices"
	"strconv"
	"strings"
)

type (
	Page         int
	PrintingRule struct {
		Rules []Page
		Page  Page
	}
	RuleBook  map[int]PrintingRule
	UpdateRow []Page
)

func NewRuleBook(s string) RuleBook {
	book := make(RuleBook)
	lines := strings.Split(s, "\n")
	for i, l := range lines {
		parts := strings.Split(l, "|")
		if len(parts) != 2 {
			panic(fmt.Errorf("line \"%d\" split on \"|\" result in a different length than 2 (actual: %d).", i, len(parts)))
		}
		// fmt.Printf("line: %d: %s must be print before %s.\n", i, parts[0], parts[1])

		p1, _ := strconv.Atoi(parts[0])
		p2, _ := strconv.Atoi(parts[1])

		x, ok := book[p1]
		if ok {
			// fmt.Printf("I append existin' element: (%d) p1:%d p2:%d x:%#v\n", i, p1, p2, x)
			x.Rules = append(x.Rules, Page(p2))
			book[p1] = x
		} else {
			// fmt.Println("I create a new element in the map")
			book[p1] = PrintingRule{
				Page:  Page(p1),
				Rules: []Page{Page(p2)},
			}
		}

	}

	return book
}

func (r RuleBook) ValidateUpdates(updates UpdateRow) bool {
	for _, p := range updates {
		// fmt.Printf("Analysing rules for page #%d.\n", p)

		printRule, ok := r[int(p)]
		if !ok {
			// no rules keep going
			continue
		}

		for _, v := range printRule.Rules {
			if slices.Contains(updates, v) {
				if slices.Index(updates, p) > slices.Index(updates, v) {
					// fmt.Printf("Page #%d is printed before #%d which is not allowed by the printing rules (%#v).\n", p, v, printRule.Rules)
					return false
				}
			}
		}
	}

	return true
}

func NewUpdatesList(s string) []UpdateRow {
	list := []UpdateRow{}

	lines := strings.Split(s, "\n")
	for _, l := range lines {
		if len(l) == 0 {
			continue
		}
		row := UpdateRow{}
		for _, p := range strings.Split(l, ",") {
			p, _ := strconv.Atoi(p)
			row = append(row, Page(p))
		}
		list = append(list, row)
	}

	return list
}

func (u UpdateRow) RetrieveMiddlePage() Page {
	return u[len(u)/2]
}
