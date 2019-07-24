package main

import (
	"fmt"
	"sort"
)

func maxChunksToSorted(arr []int) int {
	sorted := append([]int{}, arr...)
	sort.Sort(sort.IntSlice(sorted))
	mapper := make(map[int]int)
	for i, n := range sorted {
		if _, ok := mapper[n]; !ok {
			mapper[n] = i
		}
	}
	arr2 := make([]int, len(arr))
	for i, n := range arr {
		arr2[i] = mapper[n]
		mapper[n]++
	}

	return maxChunksToSortedEncoded(arr2)
}

func maxChunksToSortedEncoded(arr []int) int {
	curMax := 0
	chunks := 0
	for idx := range arr {
		if arr[idx] > curMax {
			curMax = arr[idx]
		}

		if idx == curMax {
			curMax++
			chunks++
			continue
		}
	}
	return chunks
}

func main() {
	fmt.Println(maxChunksToSorted([]int{1, 0, 2, 3, 4}))
	fmt.Println(maxChunksToSorted([]int{4, 0, 2, 3, 1}))
	fmt.Println(maxChunksToSorted([]int{2, 1, 3, 4, 4}))
}
