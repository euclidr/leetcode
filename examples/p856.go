package main

import "fmt"

func scoreOfParentheses(S string) int {
	if len(S) == 0 {
		return 0
	}

	stack := make([]int, 0, len(S))
	for _, c := range S {
		if c == '(' {
			stack = append(stack, 0)
			continue
		}
		top := 1
		if stack[len(stack)-1] != 0 {
			top = stack[len(stack)-1] * 2
			stack = stack[:len(stack)-1]
		}
		stack = stack[:len(stack)-1]
		if len(stack) > 0 && stack[len(stack)-1] != 0 {
			top += stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, top)
	}

	return stack[0]
}

func main() {
	fmt.Println(scoreOfParentheses("(())()"))
}
