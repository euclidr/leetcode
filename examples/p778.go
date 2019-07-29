package main

import (
	"container/heap"
	"fmt"
)

type Item struct {
	col  int
	row  int
	time int
}

type Heap []*Item

func (a Heap) Len() int           { return len(a) }
func (a Heap) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a Heap) Less(i, j int) bool { return a[i].time < a[j].time }

func (h *Heap) Push(x interface{}) {
	*h = append(*h, x.(*Item))
}

func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func swimInWater(grid [][]int) int {
	size := len(grid)
	marks := make([][]int, size)
	for i := 0; i < size; i++ {
		marks[i] = make([]int, size)
		for j := 0; j < size; j++ {
			marks[i][j] = -1
		}
	}

	h := &Heap{}
	heap.Init(h)
	heap.Push(h, &Item{col: size - 1, row: size - 1, time: grid[size-1][size-1]})
	marks[size-1][size-1] = grid[size-1][size-1]

	addCheck := func(r, c, t int) {
		m := max(t, grid[r][c])
		heap.Push(h, &Item{col: c, row: r, time: m})
		marks[r][c] = m
	}

	for len(*h) > 0 {
		item := heap.Pop(h).(*Item)
		// for i := 0; i < size; i++ {
		// 	fmt.Println(marks[i])
		// }
		// top
		if item.row > 0 && marks[item.row-1][item.col] == -1 {
			c, r := item.col, item.row-1
			addCheck(r, c, item.time)
			if r == 0 && c == 0 {
				return marks[0][0]
			}
		}
		// right
		if item.col < size-1 && marks[item.row][item.col+1] == -1 {
			c, r := item.col+1, item.row
			addCheck(r, c, item.time)
			if r == 0 && c == 0 {
				return marks[0][0]
			}
		}
		// bottom
		if item.row < size-1 && marks[item.row+1][item.col] == -1 {
			c, r := item.col, item.row+1
			addCheck(r, c, item.time)
			if r == 0 && c == 0 {
				return marks[0][0]
			}
		}
		// left
		if item.col > 0 && marks[item.row][item.col-1] == -1 {
			c, r := item.col-1, item.row
			addCheck(r, c, item.time)
			if r == 0 && c == 0 {
				return marks[0][0]
			}
		}
	}
	// for i := 0; i < size; i++ {
	// 	fmt.Println(marks[i])
	// }
	return marks[0][0]
}

func main() {
	// fmt.Println(swimInWater([][]int{
	// 	[]int{0, 1, 2, 3, 4},
	// 	[]int{24, 23, 22, 21, 5},
	// 	[]int{12, 13, 14, 15, 16},
	// 	[]int{11, 17, 18, 19, 20},
	// 	[]int{10, 9, 8, 7, 6},
	// }))
	fmt.Println(swimInWater([][]int{
		[]int{7, 23, 21, 9, 5},
		[]int{3, 20, 8, 18, 15},
		[]int{14, 13, 1, 0, 22},
		[]int{2, 10, 24, 17, 12},
		[]int{6, 16, 19, 4, 11},
	}))
}
