package main

import (
	"fmt"
)

func canIWin(maxChoosableInteger int, desiredTotal int) bool {
	// 100 89 78 67 56 45 34 23 12 3
	if desiredTotal == 0 {
		return true
	}

	maxSum := (maxChoosableInteger + 1) * maxChoosableInteger / 2
	if desiredTotal > maxSum {
		return false
	}

	if desiredTotal == maxSum {
		if maxChoosableInteger%2 == 1 {
			return true
		}
		return false
	}

	marks := make([]int, maxChoosableInteger+1)
	records := make(map[int]bool)
	return hasWinStep(marks, 0, desiredTotal, records)
}

func markToInt(marks []int) int {
	result := 0
	for i := 1; i < len(marks); i++ {
		result <<= 1
		result |= marks[i]
	}
	return result
}

func hasWinStep(marks []int, current int, total int, records map[int]bool) bool {
	if current >= total {
		return false
	}

	markInt := markToInt(marks)

	if rs, ok := records[markInt]; ok {
		return rs
	}

	// fmt.Println(marks)

	canWin := false
	for i := 1; i < len(marks); i++ {
		if marks[i] == 1 {
			continue
		}
		marks[i] = 1
		opponentCanWin := hasWinStep(marks, current+i, total, records)
		marks[i] = 0

		if !opponentCanWin {
			canWin = true
			break
		}
	}

	records[markInt] = canWin

	return canWin
}

func main() {
	for i := 70; i < 252; i++ {
		fmt.Println(i, canIWin(20, i))
	}
}
