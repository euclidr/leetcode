package main

import (
	"fmt"
)

type Node struct {
	A int
	B int
}

func soupServings(N int) float64 {
	if N > 30000 {
		return 1.0
	}
	node := Node{A: N, B: N}
	record := make(map[Node]float64)
	return soupServings2(node, record)
}

func soupServings2(node Node, record map[Node]float64) float64 {
	if node.A <= 0 && node.B <= 0 {
		return 0.5
	}

	if node.A <= 0 {
		return 1
	}

	if node.B <= 0 {
		return 0.0
	}

	if v, ok := record[node]; ok {
		return v
	}

	a := soupServings2(Node{node.A - 100, node.B}, record)
	b := soupServings2(Node{node.A - 75, node.B - 25}, record)
	c := soupServings2(Node{node.A - 50, node.B - 50}, record)
	d := soupServings2(Node{node.A - 25, node.B - 75}, record)

	v := (a + b + c + d) * 0.25
	record[node] = v
	return v
}

func main() {
	fmt.Println(soupServings(34533))
	fmt.Println(soupServings(13))
	fmt.Println(soupServings(0))
	fmt.Println(soupServings(30000))
}
