use itertools::Itertools;

const MAX_N: usize = 100;

pub fn solve(input: String) {
    let mut lines = input.lines();
    let rules = lines
        .by_ref()
        .take_while(|&line| !(line).is_empty())
        .map(|line| {
            line.split('|')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<(usize, usize)>>();

    let updates = lines
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; MAX_N];
    for (before, after) in rules {
        adj[before].push(after);
    }

    let is_reachable = |u: usize, v: usize| -> bool {
        let mut stack = vec![];
        stack.push(u);
        while let Some(cur) = stack.pop() {
            // won't work unless we have a DAG
            for nei in adj[cur].iter() {
                if *nei == v {
                    return true;
                }
                stack.push(*nei);
            }
        }

        false
    };
    let mut is_before: Vec<Vec<bool>> = vec![vec![false; MAX_N]; MAX_N];
    for (v, neis) in adj.iter().enumerate() {
        for nei in neis {
            if is_reachable(v, *nei) {
                is_before[v][*nei] = true;
            }
        }
    }

    let mut ans = 0;
    for update in updates {
        let mut sorted = true;
        for pair in update.windows(2) {
            if !is_before[pair[0]][pair[1]] {
                sorted = false;
            }
        }
        if sorted {
            ans += update[update.len() / 2];
        }
    }
    println!("{}", ans);
}
