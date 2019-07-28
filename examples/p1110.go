package main

import (
	"fmt"
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func delNodes(root *TreeNode, to_delete []int) []*TreeNode {
	type Item struct {
		parent       *TreeNode
		child        *TreeNode
		shouldDelete bool
	}

	sort.Sort(sort.IntSlice(to_delete))

	shouldDelete := func(val int) bool {
		idx := sort.Search(len(to_delete), func(i int) bool { return to_delete[i] >= val })
		shouldDelete := false
		if idx < len(to_delete) && to_delete[idx] == val {
			shouldDelete = true
		}
		return shouldDelete
	}

	result := make([]*TreeNode, 0)
	if root == nil {
		return result
	}

	stack := []Item{Item{nil, root, shouldDelete(root.Val)}}
	for len(stack) > 0 {
		item := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if item.shouldDelete {
			// push left right without parent
			if item.child.Right != nil {
				node := item.child.Right
				stack = append(stack, Item{nil, node, shouldDelete(node.Val)})
			}
			if item.child.Left != nil {
				node := item.child.Left
				stack = append(stack, Item{nil, node, shouldDelete(node.Val)})
			}
		} else {
			if item.parent == nil {
				result = append(result, item.child)
			}
			if item.child.Right != nil {
				node := item.child.Right
				shouldDel := shouldDelete(node.Val)
				if shouldDel {
					item.child.Right = nil
				}
				stack = append(stack, Item{item.child, node, shouldDel})
			}
			if item.child.Left != nil {
				node := item.child.Left
				shouldDel := shouldDelete(node.Val)
				if shouldDel {
					item.child.Left = nil
				}
				stack = append(stack, Item{item.child, node, shouldDel})
			}
		}
	}
	return result
}

func main() {
	fmt.Println("Hello world.")
}
