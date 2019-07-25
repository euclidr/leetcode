package main

import "fmt"

func findSubstring(s string, words []string) []int {
	n := len(words)
	result := make([]int, 0)
	if n == 0 || len(s) == 0 {
		return result
	}
	k := len(words[0])
	for start := 0; start < k; start++ {
		if (len(s) - start) < (k * n) {
			continue
		}

		record := make(map[string]int)
		for _, word := range words {
			record[word]++
		}
		matches := 0

		for wordStart := start; wordStart < (start + k*n); wordStart += k {
			word := s[wordStart : wordStart+k]
			if org, ok := record[word]; ok {
				record[word]--
				if org == 1 {
					matches++
				}
			}
		}

		if matches == len(record) {
			result = append(result, start)
		}

		for start2 := start + k; start2+(k*n) <= len(s); start2 += k {
			preWord := s[start2-k : start2]
			if org, ok := record[preWord]; ok {
				record[preWord]++
				if org == 0 {
					matches--
				}
			}
			lastWord := s[start2+k*(n-1) : start2+k*n]
			if org, ok := record[lastWord]; ok {
				record[lastWord]--
				if org == 1 {
					matches++
				}
			}
			// fmt.Println("-----------")
			// fmt.Println("start2 ", start2, preWord, lastWord)
			// fmt.Println(record)
			// fmt.Println("-----------")
			if matches == len(record) {
				result = append(result, start2)
			}
		}
	}
	return result
}

func main() {
	fmt.Println(findSubstring("wordgoodgoodgoodbestword", []string{
		"word", "good", "best", "good"}))
}
