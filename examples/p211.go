package main

import (
	"fmt"
	"strings"
)

type WordDictionary struct {
	record     map[string]bool
	words      []string
	charMapper map[int][]int
}

/** Initialize your data structure here. */
func Constructor() WordDictionary {
	return WordDictionary{
		record:     make(map[string]bool),
		words:      make([]string, 0),
		charMapper: make(map[int][]int),
	}
}

/** Adds a word into the data structure. */
func (this *WordDictionary) AddWord(word string) {
	if _, ok := this.record[word]; ok {
		return
	}

	idx := len(this.words)
	this.record[word] = true
	this.words = append(this.words, word)
	for i, c := range word {
		key := i*26 + int(c)
		if _, ok := this.charMapper[key]; ok {
			this.charMapper[key] = append(this.charMapper[key], idx)
		} else {
			this.charMapper[key] = []int{idx}
		}
	}
}

func (this *WordDictionary) intersect(listA, listB []int) []int {
	listC := make([]int, 0)
	for _, a := range listA {
		for _, b := range listB {
			if a == b {
				listC = append(listC, a)
				break
			}
		}
	}
	return listC
}

/** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
func (this *WordDictionary) Search(word string) bool {
	if _, ok := this.record[word]; ok {
		return true
	}

	if !strings.Contains(word, ".") {
		return false
	}

	results := make([]int, 0)
	hasChar := false

	for i, c := range word {
		if c == '.' {
			continue
		}
		hasChar = true
		key := i*26 + int(c)
		if aSet, ok := this.charMapper[key]; !ok {
			results = make([]int, 0)
		} else {
			if len(results) == 0 {
				results = append(results, aSet...)
			} else {
				results = this.intersect(results, aSet)
			}
		}
		if len(results) == 0 {
			break
		}
	}

	if !hasChar {
		for _, w := range this.words {
			if len(w) == len(word) {
				return true
			}
		}
		return false
	}

	for _, r := range results {
		if len(this.words[r]) == len(word) {
			return true
		}
	}
	return false
}

func main() {
	wd := Constructor()
	wd.AddWord("bad")
	wd.AddWord("dad")
	wd.AddWord("mad")
	fmt.Println(wd.Search("pad"))
	fmt.Println(wd.Search("bad"))
	fmt.Println(wd.Search(".ad"))
	fmt.Println(wd.Search("b.."))
}
