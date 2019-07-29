package main

import (
	"container/heap"
	"fmt"
)

type Item struct {
	cnt int
	num int
}

type Heap []*Item

func (h Heap) Len() int           { return len(h) }
func (h Heap) Less(i, j int) bool { return h[i].cnt > h[j].cnt }
func (h Heap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *Heap) Push(val interface{}) {
	*h = append(*h, val.(*Item))
}
func (h *Heap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func rearrangeBarcodes(barcodes []int) []int {
	counter := make(map[int]int)
	for _, num := range barcodes {
		counter[num]++
	}
	h := &Heap{}
	heap.Init(h)
	for num, cnt := range counter {
		heap.Push(h, &Item{cnt, num})
	}
	prev := (*h)[0].num + 1
	result := make([]int, 0, len(barcodes))
	for len(*h) > 0 {
		// fmt.Println(h, prev)
		next := heap.Pop(h).(*Item)
		// fmt.Println("next", next)
		var toPush *Item
		if next.num == prev {
			if len(*h) == 0 {
				panic("invalid input")
			}
			toPush = next
			next = heap.Pop(h).(*Item)
		}
		// fmt.Println("toPush", toPush)
		result = append(result, next.num)
		prev = next.num
		next.cnt--
		// fmt.Println("next2", next)
		if next.cnt != 0 {
			heap.Push(h, next)
		}
		if toPush != nil {
			heap.Push(h, toPush)
		}
		// fmt.Println("-----")
	}
	return result
}

func main() {
	fmt.Println(rearrangeBarcodes([]int{1, 1, 1, 1, 2, 2, 3, 3}))
}
