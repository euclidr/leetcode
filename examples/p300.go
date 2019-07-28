package main

func lengthOfLIS(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	records := make([]int, len(nums))
	result := 1
	for i := len(nums) - 1; i >= 0; i-- {
		l := 1
		for j := i + 1; j < len(nums); j++ {
			if nums[j] > nums[i] && records[j] >= l {
				l = records[j] + 1
			}
		}
		if l > result {
			result = l
		}
		records[i] = l
	}
	return result
}
