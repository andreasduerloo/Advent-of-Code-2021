package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
)

func main() {
	file, err := os.Open("./07_go/input.txt")

	if err != nil {
		fmt.Println("Problem with the file, bye.")
		return
	}

	defer file.Close()
	reader := bufio.NewReader(file)

	pos := make([]int, 0)

	for {
		val, err := reader.ReadString(',')
		if err != nil {
			num, err := strconv.Atoi(val[:len(val)-1])

			if err == nil {
				pos = append(pos, num)
			}

			break
		}

		num, err := strconv.Atoi(val[:len(val)-1])

		if err == nil {
			pos = append(pos, num)
		}
	}

	// nums := strings.Split(string(input), ",")
	/*
		pos := genMap(nums, func(s string) (int, error) {
			return strconv.Atoi(s)
		})
	*/

	// pos := toInt(nums)

	med := median(pos)

	distances := mapInts(pos, med, func(a, b int) int {
		if a <= b {
			return b - a
		} else {
			return a - b
		}
	})

	fmt.Println(sumInts(distances))

	// Second star
	avg := average(pos)

	distances = mapInts(pos, avg, func(a, b int) int { // Whether we need to round the average is inconsistent. Try the average and one higher, take the lowest.
		return gauss(a, b)
	})

	distancesRound := mapInts(pos, avg+1, func(a, b int) int {
		return gauss(a, b)
	})

	results := []int{sumInts(distances), sumInts(distancesRound)}

	fmt.Println(slices.Min(results))
}

func mapInts(ints []int, med int, f func(int, int) int) []int {
	out := make([]int, 0)
	for _, i := range ints {
		out = append(out, f(i, med))
	}

	return out
}

func genMap[T, U any](inslice []T, f func(T) (U, error)) []U {
	out := make([]U, 0)

	for _, elem := range inslice {
		val, err := f(elem)
		if err != nil {
			continue
		}
		out = append(out, val)
	}

	return out
}

func sumInts(ints []int) int {
	out := 0
	for _, i := range ints {
		out += i
	}
	return out
}

func median(ints []int) int {
	slices.Sort(ints)

	if len(ints)%2 == 0 {
		return int(math.Round((float64(ints[len(ints)/2]) + float64(ints[(len(ints)/2)-1])) / float64(2)))
	} else {
		return ints[len(ints)/2]
	}
}

func toInt(nums []string) []int {
	out := make([]int, 0)

	for _, val := range nums {
		ival, err := strconv.Atoi(val)
		if err == nil {
			out = append(out, ival)
		}
	}

	return out
}

func average(ints []int) int { // Second star
	sum := sumInts(ints)
	return sum / len(ints)
}

func gauss(a, b int) int {
	diff := int(math.Abs(float64(a - b)))

	if diff%2 == 0 {
		return (diff / 2) * (1 + diff)
	} else {
		return (diff/2)*(1+diff) + ((1 + diff) / 2)
	}
}
