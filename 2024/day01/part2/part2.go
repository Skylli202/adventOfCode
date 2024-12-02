package part2

func Part2(left []int, right []int) int {
	m := make(map[int]int)

	for _, v := range left {
		_, b := m[v]
		if !b {
			m[v] = 0
		}

		// Skip left values already counted in right
		if m[v] != 0 {
			continue
		}

		for _, vv := range right {
			// fmt.Printf("left is %d, right is %d\n", v, vv)
			if v == vv {
				// fmt.Printf("\tleft and right are equals, m[%d]=%d\n", v, m[v])
				m[v] = m[v] + 1
				// fmt.Printf("\tadded one... m[%d]=%d\n", v, m[v])
			}
		}
	}

	// for k, v := range m {
	// 	fmt.Printf("k: %d: %d\n", k, v)
	// }

	sum := 0
	for _, v := range left {
		sum += v * m[v]
	}

	return sum
}
