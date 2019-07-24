package main

import (
	"fmt"
)

func reorderedPowerOf2(N int) bool {
	calMarks := func(num int, marks []int) int {
		for i := range marks {
			marks[i] = 0
		}

		digits := 0
		for num > 0 {
			digits++
			marks[num%10]++
			num /= 10
		}
		return digits
	}

	marksEqual := func(marks1, marks2 []int) bool {
		for i := range marks1 {
			if marks1[i] != marks2[i] {
				return false
			}
		}
		return true
	}

	nMarks := make([]int, 10)
	digits := calMarks(N, nMarks)

	pow2 := 1
	pMarks := make([]int, 10)
	for pow2 <= 1000*1000*1000 {
		digits2 := calMarks(pow2, pMarks)
		if digits2 > digits {
			break
		}

		pow2 <<= 1
		if digits2 < digits {
			continue
		}
		if marksEqual(nMarks, pMarks) {
			return true
		}
	}
	return false
}

func main() {
	fmt.Println(reorderedPowerOf2(821))
}
