package main

import (
	"fmt"
	"math"
)

func splitIntoFibonacci(S string) []int {
	nums := make([]int, len(S))
	for i, c := range S {
		nums[i] = int(c - '0')
	}

	for n2Start := 1; n2Start < len(nums)-1; n2Start++ {
		n1, ok := combineInt(nums, 0, n2Start-1)
		if !ok {
			break
		}
		for n3Start := n2Start + 1; n3Start < len(nums); n3Start++ {
			n2, ok := combineInt(nums, n2Start, n3Start-1)
			if !ok {
				break
			}

			if result, ok := extractFibonacci(nums, n3Start, n1, n2); ok {
				return result
			}

		}
	}

	return nil
}

func extractFibonacci(nums []int, start int, n1 int, n2 int) ([]int, bool) {
	result := make([]int, 0, 2)
	result = append(result, n1)
	result = append(result, n2)

	for start < len(nums) {
		end := start
		for end <= len(nums) {
			if end == len(nums) {
				return nil, false
			}
			n3, ok := combineInt(nums, start, end)
			if !ok || n3 > n1+n2 {
				return nil, false
			}
			if n3 == n1+n2 {
				result = append(result, n3)
				n1, n2 = n2, n3
				start = end + 1
				break
			}
			if n3 < n1+n2 {
				end++
				continue
			}
		}
	}

	return result, true
}

func combineInt(nums []int, start int, end int) (int, bool) {
	if nums[start] == 0 && start < end {
		return 0, false
	}

	var rs int64
	for i := start; i <= end; i++ {
		rs = rs*10 + int64(nums[i])
		if rs > math.MaxInt32 {
			return 0, false
		}
	}
	return int(rs), true
}

func main() {
	fmt.Println(splitIntoFibonacci("123456579"))

}
