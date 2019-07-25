package main

import "fmt"

func removeKdigits(num string, k int) string {
	for i := 0; i < k; i++ {
		idx := 0
		for ; idx < len(num)-1; idx++ {
			if num[idx] > num[idx+1] {
				break
			}
		}
		num = num[:idx] + num[idx+1:]
	}
	for len(num) > 0 && num[0] == '0' {
		num = num[1:]
	}
	if len(num) == 0 {
		return "0"
	}
	return num
}

func main() {
	fmt.Println(removeKdigits("1432219", 3))
	fmt.Println(removeKdigits("10200", 1))
	fmt.Println(removeKdigits("10", 2))
}
