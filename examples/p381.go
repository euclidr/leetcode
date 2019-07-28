package main

import (
	"fmt"
	"math/rand"
)

type RandomizedCollection struct {
	nums    []int
	records map[int][]int
}

func Constructor() RandomizedCollection {
	return RandomizedCollection{
		nums:    make([]int, 0, 1000),
		records: make(map[int][]int),
	}
}

func (this *RandomizedCollection) Insert(val int) bool {
	idx := len(this.nums)
	this.nums = append(this.nums, val)
	if _, ok := this.records[val]; ok {
		this.records[val] = append(this.records[val], idx)
		return false
	}
	this.records[val] = []int{idx}
	return true
}

/** Removes a value from the collection. Returns true if the collection contained the specified element. */
func (this *RandomizedCollection) Remove(val int) bool {
	indexes, ok := this.records[val]
	if !ok {
		return false
	}

	valIndex := indexes[len(indexes)-1]
	indexes = indexes[:len(indexes)-1]
	if len(indexes) == 0 {
		delete(this.records, val)
	} else {
		this.records[val] = indexes
	}

	if valIndex == len(this.nums)-1 {
		this.nums = this.nums[:len(this.nums)-1]
		return true
	}

	lastVal := this.nums[len(this.nums)-1]
	this.nums[valIndex] = lastVal
	this.nums = this.nums[:len(this.nums)-1]

	lastRecord := this.records[lastVal]
	lastRecord[len(lastRecord)-1] = valIndex
	for i := len(lastRecord) - 1; i > 0; i-- {
		if lastRecord[i] < lastRecord[i-1] {
			lastRecord[i], lastRecord[i-1] = lastRecord[i-1], lastRecord[i]
		} else {
			break
		}
	}
	return true
}

/** Get a random element from the collection. */
func (this *RandomizedCollection) GetRandom() int {
	if len(this.nums) == 0 {
		return 0
	}
	return this.nums[rand.Intn(len(this.nums))]
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Insert(val);
 * param_2 := obj.Remove(val);
 * param_3 := obj.GetRandom();
 */

func main() {
	obj := Constructor()
	obj.Insert(1)
	obj.Insert(2)
	obj.Insert(3)
	obj.Insert(1)
	obj.Insert(3)
	obj.Insert(2)
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.records)
	fmt.Println(obj.nums)
	fmt.Println("----")
	obj.Remove(5)
	obj.Remove(3)
	fmt.Println(obj.records)
	fmt.Println(obj.nums)
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println(obj.GetRandom())
	fmt.Println("----")
}
