use std::collections::{BinaryHeap, HashSet};

/* Dijkstra Single Source Shortest Path Algorithm

Requirement: Weighted graph, no cycles with negative edges.

Concrete (non-generic) implementation assumes adjacency list (with index i of the list = vertex) form
and adj[i] is a list of pairs of integers to represent "vertices to" and corresponding weights (u -> v).

The implementation returns 2 lists the same size as the adjacency list "distances" and "parents",
where every position "i" in the list represents the information about i-th vertex.
"Distances"[i] = sum of the weights of the shortest weighted path from 'start' to vertex[i].
"Parents"[i] = parent of the vertex[i] on this path from "start" to vertex[i].
Parent of "start" is -1. Parent information is required when we want to reconstruct the exact shortest path
from "start" to some vertex "x". For those vertices, which are not connected to "start", result will be [inf], [-1]

This implementation will overflow if weighted distance for some vertices will be > i32::MAX
 */
pub fn sssp(adj: &Vec<Vec<(i32, i32)>>, start: usize) -> Option<(Vec<i32>, Vec<i32>)> {
    if adj.is_empty() || start >= adj.len() || adj[start].is_empty() {
        return None;
    }

    let mut distance = vec![i32::MAX; adj.len()];
    let mut parents = vec![-1; adj.len()];
    distance[start] = 0;

    let mut pq = BinaryHeap::new();
    pq.push((distance[start], start));
    let mut visited = HashSet::new();

    while let Some((mut d, u)) = pq.pop() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);
        // max heap
        d = -d;
        for (v, w) in &adj[u] {
            let v = *v as usize;
            if distance[v] > d + *w {
                distance[v] = d + *w;
                parents[v] = u as i32;
                pq.push((-distance[v], v));
            }
        }
    }
    Some((distance, parents))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basics() {
        // empty adjacency list, start is not there, start has no edges
        assert!(sssp(&vec![], 0).is_none());
        assert!(sssp(&vec![vec![(0, 0)]], 1).is_none());
        assert!(sssp(&vec![vec![(1, 1)], vec![]], 1).is_none());
        // Simple valid case
        assert_eq!(sssp(&vec![vec![(0, 0)]], 0), Some((vec![0], vec![-1])));
        // Harder, with cycles
        let adj = vec![
            vec![(1, 3), (2, 1)],
            vec![(3, 10), (4, 2)],
            vec![(1, 1)],
            vec![(4, 2), (0, 3)],
            vec![(3, 2)],
            vec![(6, 1)],
            vec![(5, 1)],
        ];
        let want = (
            vec![0, 2, 1, 6, 4, i32::MAX, i32::MAX],
            vec![-1, 2, 0, 4, 1, -1, -1],
        );
        assert_eq!(sssp(&adj, 0), Some(want))
    }
}
