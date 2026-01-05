impl Solution {
    pub fn max_target_nodes(e1: Vec<Vec<i32>>, e2: Vec<Vec<i32>>) -> Vec<i32> {
        let [(c1, m1), (c2, _)] = [e1, e2].map(|e| {
            let (mut g, mut m, mut c, mut q, mut q1, mut o) = (
                vec![vec![]; e.len() + 1],
                vec![0; e.len() + 1],
                [0; 3],
                vec![0],
                vec![],
                1,
            );
            for e in e {
                let (a, b) = (e[0] as usize, e[1] as usize);
                g[a].push(b);
                g[b].push(a)
            }
            while !q.is_empty() {
                for &x in &q {
                    m[x] = o;
                    c[o] += 1;
                    for &y in &g[x] {
                        if m[y] < 1 {
                            q1.push(y)
                        }
                    }
                }
                (q, q1) = (q1, q);
                o = 3 - o;
                q1.clear()
            }
            (c, m)
        });
        let cmax = c2[1].max(c2[2]);
        m1.into_iter().map(|m1| c1[m1] + cmax).collect()
    }
}

struct Solution;

fn main() {
    use utils::prelude::*;
    init_logger();
    let e1 = [[0, 1], [0, 2], [2, 3], [2, 4]]
        .into_iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    let e2 = [[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]
        .into_iter()
        .map(|x| x.to_vec())
        .collect::<Vec<_>>();
    tracing::info!("{:?}", Solution::max_target_nodes(e1, e2))
}
