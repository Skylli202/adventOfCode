package part1

func IsIncreasing(l []int) bool {
	return l[0] < l[1]
}

func IsReportSafe(l []int) bool {
	increasing := IsIncreasing(l)
	if increasing {
		return isIncreasingReportSafe(l)
	} else {
		return isDecreasingReportSafe(l)
	}
}

func isIncreasingReportSafe(l []int) bool {
	for i := 0; i < len(l)-1; i++ {
		// fmt.Println("1: ", l[i], l[i+1], l[i+1] > (l[i]+1), l[i+1] <= (l[i]+3))
		if !(l[i+1] >= (l[i]+1) && l[i+1] <= (l[i]+3)) {
			// fmt.Println("2: ", l[i+1], l[i]+1)
			// fmt.Println("2: ", l[i+1], l[i]+3)
			return false
		}
	}

	return true
}

func isDecreasingReportSafe(l []int) bool {
	// fmt.Printf("levels: %v\n", l)
	for i := 0; i < len(l)-1; i++ {
		// fmt.Println("2: ", l[i]-1, l[i+1], l[i]-3, l[i+1] < l[i]-1, l[i+1] >= l[i]-3)
		if !(l[i+1] <= l[i]-1 && l[i+1] >= l[i]-3) {
			return false
		}
	}

	return true
}
