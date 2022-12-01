package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func main() {
	f, err := os.Open("data.txt")
	defer f.Close()

	if err != nil {
		panic(err)
	}

	scanner := bufio.NewScanner(f)
	foods := []int{}

	//maxValue := 0

	currentValue := 0
	txt := ""
	val := 0

	for scanner.Scan() {
		txt = scanner.Text()
		if txt == "" {
			foods = append(foods, currentValue)
			currentValue = 0
		} else {
			val, _ = strconv.Atoi(txt)
			currentValue += val
		}
	}

	sort.Ints(foods)

	sum := foods[len(foods)-1] + foods[len(foods)-2] + foods[len(foods)-3]

	fmt.Println(sum)

}
