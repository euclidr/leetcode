package main

func minKBitFlips(A []int, K int) int {
	count := 0
	exist := true
	for i, a := range A {
		if a == 1 {
			continue
		}
		if i+K > len(A) {
			exist = false
			break
		}
		for k := i; k < i+K; k++ {
			A[k] = 1 - A[k]
		}
		count++
	}
	if exist {
		return count
	}
	return -1
}
