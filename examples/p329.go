package main

import "fmt"

func longestIncreasingPath(matrix [][]int) int {
	rows := len(matrix)
	if rows == 0 {
		return 0
	}
	cols := len(matrix[0])
	if cols == 0 {
		return 0
	}

	marks := make([][]int, rows)
	for i := 0; i < len(matrix); i++ {
		marks[i] = make([]int, cols)
	}

	max := 1
	for startRow := 0; startRow < rows; startRow++ {
		for startCol := 0; startCol < cols; startCol++ {
			if marks[startRow][startCol] != 0 {
				continue
			}
			pathLen := searchMaxPath(matrix, marks, startRow, startCol, rows, cols)
			if pathLen > max {
				max = pathLen
			}
		}
	}

	// fmt.Println(marks)
	return max
}

func searchMaxPath(matrix [][]int, marks [][]int, startRow, startCol, rows, cols int) int {
	type Pos struct {
		row int
		col int
	}
	stack := []Pos{Pos{startRow, startCol}}
	max := 1
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		row, col := top.row, top.col
		if marks[row][col] != 0 {
			stack = stack[:len(stack)-1]
			continue
		}

		ready := true
		tmpMax := 1
		// top
		if row-1 >= 0 {
			if matrix[row-1][col] > matrix[row][col] {
				if marks[row-1][col] == 0 {
					stack = append(stack, Pos{row - 1, col})
					ready = false
				} else {
					tmpMax = marks[row-1][col] + 1
				}
			}
		}

		// right
		if col+1 < cols {
			if matrix[row][col+1] > matrix[row][col] {
				if marks[row][col+1] == 0 {
					stack = append(stack, Pos{row, col + 1})
					ready = false
				} else {
					if tmpMax < marks[row][col+1]+1 {
						tmpMax = marks[row][col+1] + 1
					}
				}
			}
		}

		// bottom
		if row+1 < rows {
			if matrix[row+1][col] > matrix[row][col] {
				if marks[row+1][col] == 0 {
					stack = append(stack, Pos{row + 1, col})
					ready = false
				} else {
					if tmpMax < marks[row+1][col]+1 {
						tmpMax = marks[row+1][col] + 1
					}
				}
			}
		}

		// left
		if col-1 >= 0 {
			if matrix[row][col-1] > matrix[row][col] {
				if marks[row][col-1] == 0 {
					stack = append(stack, Pos{row, col - 1})
					ready = false
				} else {
					if tmpMax < marks[row][col-1]+1 {
						tmpMax = marks[row][col-1] + 1
					}
				}
			}
		}

		if ready {
			marks[row][col] = tmpMax
			if tmpMax > max {
				max = tmpMax
			}
			stack = stack[:len(stack)-1]
		}
	}

	return max
}

func main() {
	fmt.Println(longestIncreasingPath([][]int{
		[]int{9, 9, 4},
		[]int{6, 6, 8},
		[]int{2, 1, 1},
	}))
}
