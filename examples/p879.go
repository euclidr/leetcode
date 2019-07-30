package main

import "fmt"

func profitableSchemes(G int, P int, group []int, profit []int) int {
	crimes := len(group)
	table := make([][][]int, crimes+1)
	for i := 0; i <= crimes; i++ {
		table[i] = make([][]int, G+1)
		for g := 0; g <= G; g++ {
			table[i][g] = make([]int, P+1)
		}
	}

	mod := 1000*1000*1000 + 7
	max := func(a, b int) int {
		if a > b {
			return a
		}
		return b
	}

	for c := crimes - 1; c >= 0; c-- {
		for g := 1; g <= G; g++ {
			for p := 0; p <= P; p++ {
				cnt := table[c+1][g][p]
				if g >= group[c] {
					cnt += table[c+1][g-group[c]][max(0, p-profit[c])]
					if profit[c] >= p {
						cnt++
					}
				}
				table[c][g][p] = cnt % mod
			}
		}
	}
	return table[0][G][P]
}

func main() {
	fmt.Println(profitableSchemes(10, 5, []int{2, 3, 5}, []int{6, 7, 8}))
}
