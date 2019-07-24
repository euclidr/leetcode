package main

import (
	"fmt"
)

func maxChunksToSorted(arr []int) int {
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
}
