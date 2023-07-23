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

	// First star
	fmt.Println(dijkstra(&graph, graph[0], graph[9999]))

	// Second star

	/*
		The answer is right, but it's a dumb brute force that takes way too long (WELL over 10s). Ideas for optimization:
		- Use a min-heap to get the next node
			- Safe assumption that I'm wasting time looping through ALL 250K nodes, looking for the next one to pick, EVERY time)
		- Dijkstra the initial map, and only initialize the next part when we get to a border. Keep 'creating' the map as we reach borders
			- Sounds complicated, try the other idea first
	*/

	biggraph := buildbiggrid(lines)
	fmt.Println(bigdijkstra(&biggraph, biggraph[0], biggraph[249999]))

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

func buildbiggrid(lines []string) [250000]*node { // Return an array of nodes
	var graph [250000]*node

	for i := 0; i < 5; i++ { // Five times vertically
		for j := 0; j < 5; j++ { // Five times horizontally
			for k, line := range lines {
				risks := strings.Split(line, "")
				for l, val := range risks {
					risk, _ := strconv.Atoi(val)
					graph[(50000*i)+(100*j)+(500*k)+l] = newnode(increment(risk, i+j), (50000*i)+(100*j)+(500*k)+l)
				}
			}
		}
	}

	// Assign neighbors using indexes

	for i, n := range graph {
		if i%500 != 0 { // Left edge
			n.neighbors = append(n.neighbors, graph[i-1])
		}
		if i%500 != 499 { // Right edge
			n.neighbors = append(n.neighbors, graph[i+1])
		}
		if i >= 500 { // Top edge
			n.neighbors = append(n.neighbors, graph[i-500])
		}
		if i < 249500 { // Bottom edge
			n.neighbors = append(n.neighbors, graph[i+500])
		}
	}

	return graph
}

func increment(risk int, val int) int {
	if risk+val > 9 {
		return risk + val - 9
	} else {
		return risk + val
	}
}

func bigdijkstra(graph *[250000]*node, start *node, goal *node) int { // Returns the lowest possible risk for the goal node, starting form the start node
	current := start
	current.visited = true
	visited := 1

	for visited < 250000 { // Use min heap for next closest item?
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
