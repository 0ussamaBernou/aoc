package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	s "strings"
)

var p = fmt.Println

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type elf struct {
	position int
	total    int
}

func sum(nums ...int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	return sum
}
func main() {
	// read entire file to mem
	data, err := os.ReadFile("day1.input")
	check(err)
	content := string(data)

	// split by blankline
	food := s.Split(content, "\n\n")

	calories := make([][]int, len(food))
	for i := range food {
		foods := food[i]
		newCals := s.Split(foods, "\n")
		mappedCals := make([]int, len(newCals))
		for j, val := range newCals {
			cal, err := strconv.Atoi(val)
			if err != nil {
				continue
			}
			mappedCals[j] = cal
		}
		calories[i] = append(calories[i], mappedCals...)
	}
	elves := make([]elf, len(calories))
	for i, elfCals := range calories {
		elves[i] = elf{
			position: i + 1,
			total:    sum(elfCals...),
		}
	}

	sort.Slice(elves, func(i, j int) bool {
		return elves[i].total > elves[j].total
	})

	fmt.Printf("the %dth elf that has the most calories: %d\n", elves[0].position, elves[0].total)

	var top3 []int
	for _, elve := range elves[0:3] {
		top3 = append(top3, elve.total)
	}
	sum3 := sum(top3...)
	fmt.Printf("the sum of top 3 is %d\n", sum3)
}
