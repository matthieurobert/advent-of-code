package main

import (
	"bufio"
	"os"
)

func main() {
	f, err := os.Open("data.txt")
	defer f.Close()

	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(f)

	score := 0
	sumScore := 0
	txt := ""

	for scanner.Scan() {
		score = 0
		txt = scanner.Text()

		me := string(txt[2])
		him := string(txt[0])

		println("me : ", me)
		println("him : ", him)

		meVal := getValue(me)
		himVal := getValue(him)

		// switch meVal {
		// case 1:
		// 	switch himVal {
		// 	case 1:
		// 		score = 1 + 3
		// 	case 2:
		// 		score = 1
		// 	case 3:
		// 		score = 1 + 6
		// 	}
		// case 2:
		// 	switch himVal {
		// 	case 1:
		// 		score = 2 + 6
		// 	case 2:
		// 		score = 2 + 3
		// 	case 3:
		// 		score = 2
		// 	}
		// case 3:
		// 	switch himVal {
		// 	case 1:
		// 		score = 3
		// 	case 2:
		// 		score = 3 + 6
		// 	case 3:
		// 		score = 3 + 3
		// 	}
		// }

		switch meVal {
		case 1:
			switch himVal {
			case 1:
				score = 3
			case 2:
				score = 1
			case 3:
				score = 2
			}
		case 2:
			switch himVal {
			case 1:
				score = 1 + 3
			case 2:
				score = 2 + 3
			case 3:
				score = 3 + 3
			}
		case 3:
			switch himVal {
			case 1:
				score = 2 + 6
			case 2:
				score = 3 + 6
			case 3:
				score = 1 + 6
			}
		}
		println("score : ", score)

		sumScore += score
	}

	println(sumScore)
}

func getValue(txt string) int {
	if txt == "C" || txt == "Z" {
		return 3
	} else if txt == "B" || txt == "Y" {
		return 2
	} else if txt == "A" || txt == "X" {
		return 1
	}

	panic("erreur")
}

// Scissors = C = Z = 3
// Paper = B = Y = 2
// Rock = A = X = 1

// Draw
