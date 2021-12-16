pub fn build_up(vec: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let height = vec.len();
    let width = vec.len();
    let mut dist = vec.to_vec();
    for v in dist.iter_mut() {
        for i in v.iter_mut() {
            *i = usize::MAX;
        }
    }
    dist[0][0] = 0;
    dist[0][1] = vec[0][1];
    dist[1][0] = vec[1][0];
    let mut cands: Vec<(usize, usize)> = vec![(0, 1), (1, 0)];
    while let Some(n) = cands.iter().min_by_key(|(j, i)| dist[*j][*i]) {
        let node = (n.0, n.1);
        cands.retain(|p| *p != node);
        for neighbor in neighbors4(node.0, node.1, height, width) {
            let new_dist = dist[node.0][node.1] + vec[neighbor.0][neighbor.1];
            if new_dist < dist[neighbor.0][neighbor.1] {
                dist[neighbor.0][neighbor.1] = new_dist;
                if !cands.contains(&neighbor) {
                    cands.push(neighbor);
                }
            }
        }
    }
    dist
}

fn neighbors4(j: usize, i: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    neighbors(j, height)
        .iter()
        .filter(|s| s.is_some())
        .flat_map(|jj| {
            neighbors(i, width)
                .iter()
                .filter(|t| t.is_some())
                .map(|ii| (jj.unwrap(), ii.unwrap()))
                .collect::<Vec<_>>()
        })
        .filter(|(jj, ii)| (*jj == j || *ii == i) && !(*jj == j && *ii == i))
        .collect::<Vec<_>>()
}

fn neighbors(here: usize, upto: usize) -> [Option<usize>; 3] {
    [
        here.checked_sub(1),
        Some(here),
        (here + 1 < upto).then(|| here + 1),
    ]
}
