package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type Move int

const (
	Rock Move = 1 + iota
	Paper
	Scissors
)

const filename = "day2.input"

func read_file(filename string) string {
	// read entire file to mem
	data, err := os.ReadFile(filename)
	check(err)
	return string(data)
}

func get_move(s string) (Move, error) {
	switch s {
	case "A":
		return Move(Rock), nil
	case "B":
		return Move(Paper), nil
	case "C":
		return Move(Scissors), nil
	default:
		return Move(Rock), errors.New("unhandled move")
	}
}

func part_2(filename string) int {

	var output int

	data := read_file(filename)
	strings.Trim(data, " \n")

	for _, line := range strings.Split(data, "\n") {
		moves := strings.Split(line, " ")
		op_move, err := get_move(moves[0])
		if err != nil {
			continue
		}
		outcome := moves[1]

		switch outcome {
		case "Y":
			output += 3 + int(op_move)
		case "X":
			switch op_move {
			case Move(Rock):
				output += int(Move(Scissors))
			case Move(Paper):
				output += int(Move(Rock))
			case Move(Scissors):
				output += int(Move(Paper))
			}
		case "Z":
			switch op_move {
			case Move(Rock):
				output += int(Move(Paper)) + 6
			case Move(Paper):
				output += int(Move(Scissors)) + 6
			case Move(Scissors):
				output += int(Move(Rock)) + 6
			}
		}
		fmt.Println(op_move)

	}
	return output
}

func main() {
	output := part_2(filename)
	fmt.Printf("this is day2 part 2: %d \n", output)
}
