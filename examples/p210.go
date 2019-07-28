package main

import "fmt"

func findOrder(numCourses int, prerequisites [][]int) []int {
	type Node struct {
		num        int
		requireCnt int
		prev       *Node
		next       *Node
	}

	head := &Node{-1, 0, nil, nil}
	last := head
	mapper := make(map[int]*Node)
	prereqs := make(map[int][]int)
	for _, req := range prerequisites {
		if _, ok := prereqs[req[1]]; ok {
			prereqs[req[1]] = append(prereqs[req[1]], req[0])
		} else {
			prereqs[req[1]] = []int{req[0]}
		}

		if _, ok := mapper[req[1]]; !ok {
			last.next = &Node{req[1], 0, last, nil}
			last = last.next
			mapper[req[1]] = last
		}
		if node, ok := mapper[req[0]]; ok {
			node.requireCnt++
		} else {
			last.next = &Node{req[0], 1, last, nil}
			last = last.next
			mapper[req[0]] = last
		}
	}
	// fmt.Println(mapper)
	// node := head
	// for node != nil {
	// 	fmt.Println(node)
	// 	node = node.next
	// }
	// fmt.Println("-----")

	result := make([]int, 0, numCourses)
	for head.next != nil {
		next := head.next
		for next != nil {
			if next.requireCnt == 0 {
				break
			}
			next = next.next
		}
		// fmt.Println(next)
		if next == nil {
			return []int{}
		}
		result = append(result, next.num)
		next.prev.next = next.next
		if next.next != nil {
			next.next.prev = next.prev
		}

		for _, reqn := range prereqs[next.num] {
			// fmt.Println(reqn)
			mapper[reqn].requireCnt--
		}
	}

	for i := 0; i < numCourses; i++ {
		if _, ok := mapper[i]; !ok {
			result = append(result, i)
		}
	}

	return result
}

func main() {
	fmt.Println(findOrder(4, [][]int{
		[]int{1, 0},
		[]int{2, 0},
		[]int{3, 1},
		[]int{3, 2},
	}))
}
