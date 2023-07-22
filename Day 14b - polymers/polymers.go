package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	// Get and parse the input
	input, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(input), "\n")

	rules := make(map[string][2]string)
	pairs := parsepolymer(&(lines[0])) // Counter
	head := strings.Split(lines[0], "")[0]
	tail := strings.Split(lines[0], "")[len(lines[0])-1]

	for _, rule := range lines[2:] {
		if rule != "" {
			parserule(&rule, rules)
		}
	}

	for i := 0; i < 40; i++ {
		pairs = applyrules(pairs, rules)
	}

	fmt.Println(difference(pairs, head, tail))
}

func parsepolymer(line *string) map[string]int {
	pairs := make(map[string]int)
	elements := strings.Split(*line, "")

	for i := 0; i < len(elements)-1; i++ {
		pairs[(elements[i] + elements[i+1])] = pairs[(elements[i]+elements[i+1])] + 1
	}

	return pairs
}

func parserule(line *string, rules map[string][2]string) {
	elements := strings.Split(*line, "")

	rules[(elements[0] + elements[1])] = [2]string{(elements[0] + elements[6]), (elements[6] + elements[1])}
}

func applyrules(polymer map[string]int, rules map[string][2]string) map[string]int {
	newpolymer := make(map[string]int)

	for k, v := range polymer {
		newpolymer[(rules[k])[0]] = newpolymer[(rules[k])[0]] + v
		newpolymer[(rules[k])[1]] = newpolymer[(rules[k])[1]] + v
	}

	return newpolymer
}

func difference(polymer map[string]int, head string, tail string) int {
	elements := make(map[string]int) // We have pairs of elements and need the individual elements

	for k, v := range polymer {
		e := strings.Split(k, "")

		elements[e[0]] = elements[e[0]] + v
		elements[e[1]] = elements[e[1]] + v
	}

	for k, v := range elements {
		elements[k] = v / 2
		if k == head || k == tail {
			elements[k] = elements[k] + 1
		}
	}

	lowest := 0
	highest := 0

	for _, v := range elements {
		if v > highest {
			highest = v
		}

		if v < lowest || lowest == 0 {
			lowest = v
		}
	}

	return highest - lowest
}
