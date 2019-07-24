package main

import (
	"fmt"
)

func findMinStep(board string, hand string) int {
	if len(board) == 0 {
		return 0
	}
	handSet := make(map[byte]int)
	for _, h := range []byte(hand) {
		handSet[h]++
	}
	return find([]byte(board), handSet, make([]byte, 0), len(hand))

}

func find(board []byte, hand map[byte]int, pre []byte, max int) int {
	if len(board) == 0 {
		return len(pre)
	}
	if len(pre) == max {
		return -1
	}

	fmt.Println(hand, pre)

	min := -1

	for h, c := range hand {
		if c == 0 {
			continue
		}

		hand[h]--
		pre = append(pre, h)

		for left := 0; left < len(board); {
			if board[left] != h {
				left++
				continue
			}
			right := left + 1
			for right < len(board) && board[right] == h {
				right++
			}
			newBoard := insert(board, right, h)
			left = right

			newBoard = shrink(newBoard)
			if len(newBoard) == 0 {
				min = len(pre)
				break
			}
			subResult := find(newBoard, hand, pre, max)
			if min == -1 {
				min = subResult
				continue
			}

			if subResult != -1 && subResult < min {
				min = subResult
				max = subResult
			}
		}

		pre = pre[:len(pre)-1]
		hand[h]++
	}

	return min
}

func insert(board []byte, r int, c byte) []byte {
	newBoard := make([]byte, len(board)+1)
	copy(newBoard, board[:r])
	newBoard[r] = c
	copy(newBoard[r+1:], board[r:])
	return newBoard
}

func shrink(board []byte) []byte {
	if len(board) == 0 {
		return make([]byte, 0)
	}
	newBoard := make([]byte, len(board))
	copy(newBoard, board)
	for l := 0; l < len(newBoard); {
		r := l + 1
		for r < len(newBoard) && newBoard[r] == newBoard[l] {
			r++
		}
		if r-l >= 3 {
			newBoard = append(newBoard[:l], newBoard[r:]...)
			l = 0
		} else {
			l = r
		}
	}
	return newBoard
}

func main() {
	//fmt.Println(string(shrink([]byte("WWRRRWW"))))
	//fmt.Println(string(shrink([]byte("WWRRWW"))))
	//fmt.Println(findMinStep("WRYYRWWRRWW", "WYBR"))
	fmt.Println(findMinStep("WWRRBBWW", "WRBRW"))
}
