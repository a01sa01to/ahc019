extern crate rand;
use rand::Rng;
use std::{
    collections::{HashMap, HashSet},
    io, mem,
    time::{Duration, Instant},
};

const INF: usize = 1_000_000_000;
const DX: [i32; 6] = [1, 0, 0, -1, 0, 0];
const DY: [i32; 6] = [0, 1, 0, 0, -1, 0];
const DZ: [i32; 6] = [0, 0, 1, 0, 0, -1];

fn input() -> (
    usize,
    (Vec<Vec<bool>>, Vec<Vec<bool>>),
    (Vec<Vec<bool>>, Vec<Vec<bool>>),
) {
    let d = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap() as usize
    };

    // 同じ形式なのでこれで十分
    fn input_sil(d: usize) -> Vec<Vec<bool>> {
        let mut v = vec![vec![false; d]; d];
        for i in 0..d {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            for (j, c) in input.trim().chars().enumerate() {
                v[i][j] = c == '1';
            }
        }
        v
    }
    (
        d,
        (input_sil(d), input_sil(d)),
        (input_sil(d), input_sil(d)),
    )
}

fn output(ans1: Vec<Vec<Vec<usize>>>, ans2: Vec<Vec<Vec<usize>>>) {
    let mut set = ans1.iter().flatten().flatten().collect::<HashSet<_>>();
    set.extend(ans2.iter().flatten().flatten());
    println!("{}", set.len() - 1);
    for i in 0..ans1.len() {
        for j in 0..ans1.len() {
            for k in 0..ans1.len() {
                print!("{} ", ans1[i][j][k]);
            }
        }
    }
    println!("");
    for i in 0..ans2.len() {
        for j in 0..ans2.len() {
            for k in 0..ans2.len() {
                print!("{} ", ans2[i][j][k]);
            }
        }
    }
    println!("");
}

fn is_same(b1: &Vec<(usize, usize, usize)>, b2: &Vec<(usize, usize, usize)>) -> bool {
    fn normalize(b: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
        let mut min_x = !0;
        let mut min_y = !0;
        let mut min_z = !0;
        for &(x, y, z) in b {
            min_x = if min_x > x { x } else { min_x };
            min_y = if min_y > y { y } else { min_y };
            min_z = if min_z > z { z } else { min_z };
        }
        b.iter()
            .map(|&(x, y, z)| (x - min_x, y - min_y, z - min_z))
            .collect()
    }

    if b1.len() != b2.len() {
        return false;
    }
    let mut b1 = normalize(&b1);
    let mut b2 = normalize(&b2);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for &(x, y, z) in &b2 {
        max_x = if max_x < x { x } else { max_x };
        max_y = if max_y < y { y } else { max_y };
        max_z = if max_z < z { z } else { max_z };
    }
    b1.sort();
    for i in 0..6 {
        for _ in 0..4 {
            b2.sort();
            if b1 == b2 {
                return true;
            }
            for (x, y, _) in &mut b2 {
                let t = *x;
                *x = max_y - *y;
                *y = t;
            }
            mem::swap(&mut max_x, &mut max_y);
        }
        if i & 1 != 0 {
            for (_, y, z) in &mut b2 {
                let t = *y;
                *y = max_z - *z;
                *z = t;
            }
            mem::swap(&mut max_y, &mut max_z);
        } else {
            for (x, _, z) in &mut b2 {
                let t = *z;
                *z = max_x - *x;
                *x = t;
            }
            mem::swap(&mut max_x, &mut max_z);
        }
    }
    false
}

fn main() {
    // 時間計測
    let start_time = Instant::now();
    // 乱数
    let mut rng = rand::thread_rng();

    let (d, sil1, sil2) = input();

    // 答え
    let mut ans1 = vec![vec![vec![INF; d]; d]; d];
    let mut ans2 = vec![vec![vec![INF; d]; d]; d];

    // 絶対に削らなければいけない場所
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if !sil1.0[k][i] {
                    ans1[i][j][k] = 0;
                }
                if !sil1.1[k][j] {
                    ans1[i][j][k] = 0;
                }
                if !sil2.0[k][i] {
                    ans2[i][j][k] = 0;
                }
                if !sil2.1[k][j] {
                    ans2[i][j][k] = 0;
                }
            }
        }
    }

    // 全部1x1x1としておく
    let mut block1 = Vec::<Vec<(usize, usize, usize)>>::new();
    let mut block2 = Vec::<Vec<(usize, usize, usize)>>::new();
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans1[i][j][k] > 0 {
                    block1.push(vec![(i, j, k)]);
                    ans1[i][j][k] = block1.len();
                }
                if ans2[i][j][k] > 0 {
                    block2.push(vec![(i, j, k)]);
                    ans2[i][j][k] = block2.len();
                }
            }
        }
    }

    let mut stats = (0, 0);

    // 5s ぶんまわす
    while start_time.elapsed() < Duration::from_millis(5000) {
        stats.0 += 1;
        let idx1 = rng.gen_range(0, block1.len());
        let idx2 = rng.gen_range(0, block2.len());
        if block1[idx1].len() == 0 {
            continue;
        }
        if block2[idx2].len() == 0 {
            continue;
        }
        if !is_same(&block1[idx1], &block2[idx2]) {
            continue;
        }
        let mut new_block1 = block1.clone();
        let mut new_ans1 = ans1.clone();

        let mut changed = false;
        for &(x, y, z) in &block1[idx1] {
            for d in 0..6usize {
                let nx = (x as i32 + DX[d]) as usize;
                let ny = (y as i32 + DY[d]) as usize;
                let nz = (z as i32 + DZ[d]) as usize;
                // 範囲外
                if nx >= d || ny >= d || nz >= d {
                    continue;
                }
                // すでに同じブロック
                if new_ans1[nx][ny][nz] == new_ans1[x][y][z] {
                    continue;
                }
                // 実装がめんどいのでとりあえず1つだけをmerge
                if new_ans1[nx][ny][nz] > 0 {
                    let oldidx = new_ans1[nx][ny][nz] - 1;
                    if new_block1[oldidx].len() != 1 {
                        continue;
                    }
                    new_block1[idx1].push((nx, ny, nz));
                    new_block1[oldidx].clear();
                    new_ans1[nx][ny][nz] = idx1 + 1;
                    changed = true;
                    break;
                }
            }
            if changed {
                break;
            }
        }

        let mut new_block2 = block2.clone();
        let mut new_ans2 = ans2.clone();
        let mut changed = false;
        for &(x, y, z) in &block2[idx2] {
            for d in 0..6usize {
                let nx = (x as i32 + DX[d]) as usize;
                let ny = (y as i32 + DY[d]) as usize;
                let nz = (z as i32 + DZ[d]) as usize;
                // 範囲外
                if nx >= d || ny >= d || nz >= d {
                    continue;
                }
                // すでに同じブロック
                if new_ans2[nx][ny][nz] == new_ans2[x][y][z] {
                    continue;
                }
                // 実装がめんどいのでとりあえず1つだけをmerge
                if new_ans2[nx][ny][nz] > 0 {
                    let oldidx = new_ans2[nx][ny][nz] - 1;
                    if new_block2[oldidx].len() != 1 {
                        continue;
                    }
                    new_block2[idx2].push((nx, ny, nz));
                    new_block2[oldidx].clear();
                    if is_same(&new_block1[idx1], &new_block2[idx2]) {
                        changed = true;
                        break;
                    } else {
                        // revert
                        new_block2[idx2].pop();
                        new_block2[oldidx].push((nx, ny, nz));
                    }
                }
            }
            if changed {
                break;
            }
        }

        if changed {
            mem::swap(&mut block1, &mut new_block1);
            mem::swap(&mut ans1, &mut new_ans1);
            mem::swap(&mut block2, &mut new_block2);
            mem::swap(&mut ans2, &mut new_ans2);
            stats.1 += 1;
        }
    }

    // ブロックIDの修正
    block1.retain(|b| b.len() > 0);
    block2.retain(|b| b.len() > 0);
    block1.sort_by_key(|b| b.len());
    block2.sort_by_key(|b| b.len());
    let mut mp = HashMap::<usize, usize>::new();
    let mut used = HashSet::<usize>::new();
    for (i2, b2) in block2.iter().enumerate() {
        for (i1, b1) in block1.iter().enumerate() {
            if used.contains(&i1) {
                continue;
            }
            if is_same(b1, b2) {
                mp.insert(i2, i1);
                used.insert(i1);
                break;
            }
        }
    }

    for (i, b) in block1.iter().enumerate() {
        for &(x, y, z) in b {
            ans1[x][y][z] = i + 1;
        }
    }
    let mut nullcnt = 0;
    for (i, b) in block2.iter().enumerate() {
        for &(x, y, z) in b {
            if mp.contains_key(&i) {
                ans2[x][y][z] = mp.get(&i).unwrap() + 1;
            } else {
                nullcnt += 1;
                ans2[x][y][z] = block1.len() + nullcnt;
            }
        }
    }

    // 出力
    eprintln!("Stats: attempt={}, success={}", stats.0, stats.1);
    output(ans1, ans2);
}
