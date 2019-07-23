package main

import (
	"fmt"
)

func findKthLargest(nums []int, k int) int {
	k = len(nums) - k
	left, right := 0, len(nums)-1
	for {
		l, r := left+1, right
		for l <= r {
			for l <= r && nums[l] <= nums[left] {
				l++
			}
			for l <= r && nums[r] > nums[left] {
				r--
			}

			if l < r {
				nums[l], nums[r] = nums[r], nums[l]
			}
		}

		nums[left], nums[r] = nums[r], nums[left]

		if r == k {
			return nums[r]
		}

		if r > k {
			right = r - 1
		} else {
			left = r + 1
		}
	}

	panic("unreachable!")
}

func main() {
	fmt.Println(findKthLargest([]int{3, 2, 1, 5, 6, 4}, 2))
}
