package main

import (
	"fmt"
)

func pathInZigZagTree(label int) []int {
	result := make([]int, 0)
	for label > 0 {
		result = append(result, label)

		var higher uint32
		for (1 << higher) <= label {
			higher++
		}

		lower := higher - 1
		label = ((1 << higher) - 1 - label) + (1 << lower)
		label /= 2
	}

	for i, j := 0, (len(result) - 1); i < j; {
		result[i], result[j] = result[j], result[i]
		i++
		j--
	}
	return result
}

func main() {
	fmt.Println(pathInZigZagTree(26))
	fmt.Println(pathInZigZagTree(1))
	fmt.Println(pathInZigZagTree(14))
}
