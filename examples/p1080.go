package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Item struct {
	parent   *Item
	current  *TreeNode
	hasValid bool
	checked  bool
	total    int
}

func sufficientSubset(root *TreeNode, limit int) *TreeNode {
	if root == nil {
		return root
	}

	stack := []*Item{&Item{nil, root, false, false, root.Val}}
	toDeletes := make(map[*TreeNode]bool)
	for len(stack) > 0 {
		item := stack[len(stack)-1]
		if item.checked {
			if !item.hasValid {
				toDeletes[item.current] = true
			} else {
				if item.parent != nil {
					item.parent.hasValid = true
				}
			}
			stack = stack[:len(stack)-1]
			continue
		}

		item.checked = true
		if item.current.Left == nil && item.current.Right == nil {
			if item.total >= limit {
				item.hasValid = true
			}
			continue
		}

		if item.current.Left != nil {
			left := item.current.Left
			stack = append(stack, &Item{item, left, false, false, item.total + left.Val})
		}

		if item.current.Right != nil {
			right := item.current.Right
			stack = append(stack, &Item{item, right, false, false, item.total + right.Val})
		}
	}

	if toDeletes[root] {
		return nil
	}

	stack2 := []*TreeNode{root}

	for len(stack2) > 0 {
		top := stack2[len(stack2)-1]
		stack2 = stack2[:len(stack2)-1]

		if top.Left != nil {
			if toDeletes[top.Left] {
				top.Left = nil
			} else {
				stack2 = append(stack2, top.Left)
			}
		}

		if top.Right != nil {
			if toDeletes[top.Right] {
				top.Right = nil
			} else {
				stack2 = append(stack2, top.Right)
			}
		}
	}

	return root
}

func main() {
	fmt.Println("hello world")
}
