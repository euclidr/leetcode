package main

import "fmt"

func sumOfDistancesInTree(N int, edges [][]int) []int {
	if N == 0 {
		return make([]int, 0)
	}

	graph := make(map[int][]int)
	for _, edge := range edges {
		if _, ok := graph[edge[0]]; !ok {
			graph[edge[0]] = []int{edge[1]}
		} else {
			graph[edge[0]] = append(graph[edge[0]], edge[1])
		}

		if _, ok := graph[edge[1]]; !ok {
			graph[edge[1]] = []int{edge[0]}
		} else {
			graph[edge[1]] = append(graph[edge[1]], edge[0])
		}
	}

	counts := make([]int, N)
	sums := make([]int, N)
	initiate(0, 0, graph, counts, sums)
	results := make([]int, N)
	solve(0, 0, graph, counts, sums, results)
	return results
}

func initiate(from int, cur int, graph map[int][]int, counts []int, sums []int) {
	count := 1
	sum := 0
	for _, next := range graph[cur] {
		if next == from {
			continue
		}
		initiate(cur, next, graph, counts, sums)
		count += counts[next]
		sum += sums[next] + counts[next]
	}
	counts[cur] = count
	sums[cur] = sum
}

func solve(from int, cur int, graph map[int][]int, counts []int, sums []int, results []int) {
	results[cur] = sums[cur]
	for _, next := range graph[cur] {
		if next == from {
			continue
		}
		orgSum, orgCnt := sums[cur], counts[cur]
		orgNextSum, orgNextCnt := sums[next], counts[next]
		sums[cur] = sums[cur] - sums[next] - counts[next]
		counts[cur] = counts[cur] - counts[next]
		sums[next] = sums[cur] + sums[next] + counts[cur]
		counts[next] = counts[cur] + counts[next]
		solve(cur, next, graph, counts, sums, results)
		sums[cur], counts[cur] = orgSum, orgCnt
		sums[next], counts[next] = orgNextSum, orgNextCnt
	}
}

func main() {
	fmt.Println(sumOfDistancesInTree(6, [][]int{
		[]int{0, 1},
		[]int{0, 2},
		[]int{2, 3},
		[]int{2, 4},
		[]int{2, 5},
	}))
}
