// Graph related algorithms

pub mod dijkstra;

use std::collections::HashMap;

pub fn dfs_visit(i: i32, adj_list: &Vec<Vec<i32>>, visited: &mut Vec<i32>, result: &mut Vec<i32>) {
    if visited[i as usize] == 1 {
        return;
    }
    visited[i as usize] = 1;
    for node in &adj_list[i as usize] {
        if visited[*node as usize] == 0 {
            dfs_visit(*node, adj_list, visited, result);
        }
    }
    result.push(i);
}

pub fn dfs_list(adj_list: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut visited = vec![0; adj_list.len()];
    let mut res = Vec::new();
    for i in 0..adj_list.len() {
        if visited[i] == 0 {
            dfs_visit(i as i32, adj_list, &mut visited, &mut res);
        }
    }
    res
}

pub fn bfs_list(adj_list: &[Vec<i32>], s: i32) -> (HashMap<i32, i32>, HashMap<i32, i32>) {
    let mut parent = HashMap::from([(s, -1)]);
    let mut level = HashMap::from([(s, 0)]);
    let mut i = 1;
    let mut current = vec![s];
    while !current.is_empty() {
        let mut next = vec![];
        for u in &current {
            for v in &adj_list[*u as usize] {
                if !level.contains_key(v) {
                    level.insert(*v, i);
                    parent.insert(*v, *u);
                    next.push(*v);
                }
            }
        }
        current = next;
        i += 1;
    }
    (level, parent)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dfs_visit() {
        let adj_list = vec![vec![0, 2], vec![1], vec![2]];
        let mut visited = vec![0; adj_list.len()];
        let mut res = Vec::new();
        dfs_visit(0, &adj_list, &mut visited, &mut res);
        assert_eq!(res, vec![2, 0]);
    }
    #[test]
    fn test_dfs_list() {
        let adj_list = vec![vec![0, 2], vec![1], vec![2]];
        assert_eq!(dfs_list(&adj_list), vec![2, 0, 1])
    }
    #[test]
    fn test_bfs_list() {
        let adj_list = vec![vec![5, 3], vec![3], vec![2, 0], vec![0, 1], vec![], vec![2]];
        let s = 0;
        let want_level = HashMap::from([(0, 0), (5, 1), (3, 1), (2, 2), (1, 2)]);
        let want_parent = HashMap::from([(0, -1), (5, 0), (3, 0), (2, 5), (1, 3)]);
        assert_eq!(bfs_list(&adj_list, s), (want_level, want_parent))
    }
}
