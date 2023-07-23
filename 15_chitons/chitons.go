package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input, _ := os.ReadFile("./input.txt")
	lines := strings.Split(string(input), "\n")

	graph := buildgrid(lines)

	fmt.Println(dijkstra(&graph, graph[0], graph[9999]))
}

type node struct {
	risk      int
	index     int
	visited   bool
	totalrisk int
	neighbors []*node
}

func newnode(risk int, index int) *node {
	n := node{risk: risk,
		index:     index,
		visited:   false,
		totalrisk: 0,
		neighbors: make([]*node, 0)}

	return &n
}

func buildgrid(lines []string) [10000]*node { // Return an array of nodes
	var graph [10000]*node

	for i, line := range lines {
		risks := strings.Split(line, "") // Slice of strings

		for j, val := range risks {
			risk, _ := strconv.Atoi(val)
			graph[(100*i)+j] = newnode(risk, (100*i)+j)
		}
	}

	// Assign neighbors using indexes

	for i, n := range graph {
		if i%100 != 0 { // Left edge
			n.neighbors = append(n.neighbors, graph[i-1])
		}
		if i%100 != 99 { // Right edge
			n.neighbors = append(n.neighbors, graph[i+1])
		}
		if i >= 100 { // Top edge
			n.neighbors = append(n.neighbors, graph[i-100])
		}
		if i < 9900 { // Bottom edge
			n.neighbors = append(n.neighbors, graph[i+100])
		}
	}

	return graph
}

func dijkstra(graph *[10000]*node, start *node, goal *node) int { // Returns the lowest possible risk for the goal node, starting form the start node
	current := start
	current.visited = true
	visited := 1

	for visited < 10000 { // Use min heap for next closest item?
		for _, neighbor := range current.neighbors { // Set or update tentative distances for current neighbors
			if neighbor != start && neighbor.totalrisk == 0 || neighbor.totalrisk > current.totalrisk+neighbor.risk {
				neighbor.totalrisk = current.totalrisk + neighbor.risk
			}
		}

		closest := newnode(0, 0)
		for _, candidate := range graph { // Find the closest, unvisited node
			if candidate.visited == false && (closest.totalrisk == 0 && candidate.totalrisk != 0 || candidate.totalrisk != 0 && candidate.totalrisk < closest.totalrisk) {
				closest = candidate
			}
		}

		current = closest
		current.visited = true
		visited += 1
	}

	return goal.totalrisk
}
