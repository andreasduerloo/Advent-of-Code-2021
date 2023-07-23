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

		Turns out that was it - it's near-instant now.
	*/

	biggraph := buildbiggrid(lines)
	fmt.Println(bigdijkstra(&biggraph, biggraph[0], biggraph[249999]))
}

type node struct {
	risk      int
	index     int
	visited   bool
	inheap    bool
	totalrisk int
	neighbors []*node
}

func newnode(risk int, index int) *node {
	n := node{risk: risk,
		index:     index,
		visited:   false,
		inheap:    false,
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

func buildbiggrid(lines []string) [250000]*node { // Switch to a slice so the length doesn't need to be known at compile time?
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

func bigdijkstra(graph *[250000]*node, start *node, goal *node) int {
	current := start
	current.visited = true
	visited := 1

	heap := heapinit()

	for visited < 250000 {
		for _, neighbor := range current.neighbors { // Set or update tentative distances for current neighbors
			if neighbor != start && neighbor.totalrisk == 0 || neighbor.totalrisk > current.totalrisk+neighbor.risk {
				neighbor.totalrisk = current.totalrisk + neighbor.risk

				if neighbor.inheap == true {
					heapdelete(&heap, neighbor)
					heapinsert(&heap, neighbor)
				}
			}

			if neighbor.visited == false && neighbor.inheap == false { // Insert unvisited neighbors into the minheap
				heapinsert(&heap, neighbor)
				neighbor.inheap = true
			}
		}

		current = heapremove(&heap)
		current.visited = true
		current.inheap = false
		visited += 1
	}

	return goal.totalrisk
}

// Min heap - there is also a heap package in the standard library

// The min heap lives in memory as an array of pointers to nodes

type minheap struct {
	heap *[10000]*node // A slice will be slower for insertions, so we go for an oversized array. We could also store the node indexes
	last int
}

func heapinit() minheap {
	var arr [10000]*node

	mh := minheap{
		heap: &arr,
		last: 0,
	}

	return mh
}

func heapinsert(mh *minheap, n *node) {
	mh.last = mh.last + 1
	mh.heap[mh.last] = n

	currentindex := mh.last
	parentindex := currentindex / 2 // Floored division

	for parentindex >= 1 && mh.heap[parentindex].totalrisk > mh.heap[currentindex].totalrisk {
		temp := mh.heap[parentindex]
		mh.heap[parentindex] = mh.heap[currentindex]
		mh.heap[currentindex] = temp
		currentindex = parentindex
		parentindex = currentindex / 2
	}
}

func heapremove(mh *minheap) *node {
	if mh.last == 0 {
		return nil
	}

	result := mh.heap[1]
	mh.heap[1] = mh.heap[mh.last]
	mh.heap[mh.last] = nil
	mh.last = mh.last - 1

	i := 1
	for i <= mh.last {
		swap := i
		if 2*i <= mh.last && (mh.heap[swap]).totalrisk > (mh.heap[2*i]).totalrisk {
			swap = (2 * i)
		}
		if 2*i+1 <= mh.last && (mh.heap[swap]).totalrisk > (mh.heap[2*i+1]).totalrisk {
			swap = (2*i + 1)
		}

		if i != swap { // We need to swap
			temp := mh.heap[i]
			mh.heap[i] = mh.heap[swap]
			mh.heap[swap] = temp
			i = swap
		} else {
			break
		}
	}
	return result
}

func find(n *node, mh *minheap) int {
	for i := 1; i <= mh.last; i++ {
		if mh.heap[i] == n {
			return i
		}
	}
	return 0
}

func heapdelete(mh *minheap, n *node) {
	i := find(n, mh)

	mh.heap[i] = mh.heap[mh.last]
	mh.heap[mh.last] = nil
	mh.last = mh.last - 1

	for i <= mh.last {
		swap := i
		if 2*i <= mh.last && (mh.heap[swap]).totalrisk > (mh.heap[2*i]).totalrisk {
			swap = (2 * i)
		}
		if 2*i+1 <= mh.last && (mh.heap[swap]).totalrisk > (mh.heap[2*i+1]).totalrisk {
			swap = (2*i + 1)
		}

		if i != swap { // We need to swap
			temp := mh.heap[i]
			mh.heap[i] = mh.heap[swap]
			mh.heap[swap] = temp
			i = swap
		} else {
			break
		}
	}
}
