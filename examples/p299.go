package main

import "fmt"

func getHint(secret string, guess string) string {
	bulls := 0
	cows := 0
	secretStat := make([]int, 10)
	guessStat := make([]int, 10)
	zero := int('0')

	for i := 0; i < len(secret); i++ {
		if secret[i] == guess[i] {
			bulls++
			continue
		}
		secretStat[int(secret[i])-zero]++
		guessStat[int(guess[i])-zero]++
	}

	for i := 0; i < 10; i++ {
		if secretStat[i] < guessStat[i] {
			cows += secretStat[i]
		} else {
			cows += guessStat[i]
		}
	}

	return fmt.Sprintf("%dA%dB", bulls, cows)
}

func main() {
	fmt.Println(getHint("1807", "7810"))
}
