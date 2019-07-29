package main

import (
	"container/heap"
	"fmt"
	"sort"
)

func scheduleCourseWrong(courses [][]int) int {
	valids := make([][]int, 0, len(courses))
	for _, course := range courses {
		if course[0] <= course[1] {
			valids = append(valids, course)
		}
	}
	courses = valids

	if len(courses) == 0 {
		return 0
	}

	sort.SliceStable(courses, func(i, j int) bool {
		if courses[i][1] == courses[j][1] {
			return courses[i][0] > courses[j][0]
		}
		return courses[i][1] < courses[j][1]
	})

	totalDays := courses[0][1]
	for _, course := range courses {
		if course[1] > totalDays {
			totalDays = course[1]
		}
	}

	counts := make([]int, totalDays+1)
	positions := make([]int, totalDays+1)
	for idx, course := range courses {
		if counts[course[0]] == 0 {
			positions[course[0]] = idx
			counts[course[0]] = 1
		}
	}

	max := 1
	for i := 1; i <= totalDays; i++ {
		if counts[i] == 0 {
			continue
		}
		for pos := positions[i] + 1; pos < len(courses); pos++ {
			if i+courses[pos][0] <= courses[pos][1] {
				newCount := counts[i] + 1
				if newCount > counts[i+courses[pos][0]] {
					counts[i+courses[pos][0]] = newCount
					positions[i+courses[pos][0]] = pos
					if newCount > max {
						max = newCount
					}
				}
			}
		}
	}

	fmt.Println(courses)
	fmt.Println(counts)
	fmt.Println(positions)

	return max
}

type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}
func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func scheduleCourse(courses [][]int) int {
	valids := make([][]int, 0, len(courses))
	for _, course := range courses {
		if course[0] <= course[1] {
			valids = append(valids, course)
		}
	}
	courses = valids

	if len(courses) == 0 {
		return 0
	}

	sort.SliceStable(courses, func(i, j int) bool {
		if courses[i][1] == courses[j][1] {
			return courses[i][0] < courses[j][0]
		}
		return courses[i][1] < courses[j][1]
	})

	result := &IntHeap{}
	heap.Init(result)
	total := 0
	for _, course := range courses {
		if total+course[0] <= course[1] {
			heap.Push(result, course[0])
			total += course[0]
			continue
		}
		if (*result)[0] > course[0] && (total-(*result)[0]+course[0]) < course[1] {
			total = total - (*result)[0] + course[0]
			heap.Pop(result)
			heap.Push(result, course[0])
		}
	}
	return len(*result)
}

func main() {
	// fmt.Println(scheduleCourse([][]int{
	// 	[]int{100, 200},
	// 	[]int{200, 1300},
	// 	[]int{1000, 1250},
	// 	[]int{2000, 3200},
	// }))
	fmt.Println(scheduleCourse([][]int{
		[]int{5, 15},
		[]int{3, 19},
		[]int{6, 7},
		[]int{2, 10},
		[]int{5, 16},
		[]int{8, 14},
		[]int{10, 11},
		[]int{2, 19},
	}))
}
