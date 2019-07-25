package main

import "fmt"

const (
	None = iota
	Black
	White
)

func isBipartite(graph [][]int) bool {
	if len(graph) < 2 {
		return true
	}

	marks := make([]int, len(graph))

	for i := range graph {
		if marks[i] == None {
			if !recursiveMark(i, graph, marks) {
				return false
			}
		}
	}
	return true
}

func recursiveMark(idx int, graph [][]int, marks []int) bool {
	stack := []int{idx}
	marks[idx] = White
	for len(stack) > 0 {
		last := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		for _, next := range graph[last] {
			if marks[next] == marks[last] {
				return false
			}
			if marks[next] == None {
				marks[next] = White
				if marks[last] == White {
					marks[next] = Black
				}
				stack = append(stack, next)
			}
		}
	}
	return true
}

func main() {
	fmt.Println(isBipartite([][]int{
		[]int{1, 2, 3},
		[]int{0, 2},
		[]int{0, 1, 3},
		[]int{0, 2},
	}))
}
