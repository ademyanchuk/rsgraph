// Graph related algorithms

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
}
