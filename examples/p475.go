package main

import (
	"fmt"
	"sort"
)

func findRadius(houses []int, heaters []int) int {
	sort.Ints(heaters)
	max := 0
	for _, pos := range houses {
		hIdx := sort.Search(len(heaters), func(i int) bool { return heaters[i] >= pos })
		if hIdx == len(heaters) {
			a := pos - heaters[hIdx-1]
			if a > max {
				max = a
			}
			continue
		}
		if hIdx == 0 {
			b := heaters[0] - pos
			if b > max {
				max = b
			}
			continue
		}
		a := pos - heaters[hIdx-1]
		b := heaters[hIdx] - pos
		if a > b {
			a = b
		}
		if a > max {
			max = a
		}
	}
	return max
}

func main() {
	fmt.Println(findRadius([]int{1, 2, 3, 4}, []int{1, 4}))
}
