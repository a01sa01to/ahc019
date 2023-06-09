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
            if min_x > x {
                min_x = x;
            }
            if min_y > y {
                min_y = y;
            }
            if min_z > z {
                min_z = z;
            }
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
        if max_x < x {
            max_x = x;
        }
        if max_y < y {
            max_y = y;
        }
        if max_z < z {
            max_z = z;
        }
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

fn update_block_id(
    d: usize,
    block1: &mut Vec<Vec<(usize, usize, usize)>>,
    block2: &mut Vec<Vec<(usize, usize, usize)>>,
    ans1: &mut Vec<Vec<Vec<usize>>>,
    ans2: &mut Vec<Vec<Vec<usize>>>,
) {
    // len=0を消す
    block1.retain(|b| b.len() > 0);
    block2.retain(|b| b.len() > 0);
    // lenの大きい順にソート
    block1.sort_by_key(|b| -(b.len() as i32));
    block2.sort_by_key(|b| -(b.len() as i32));

    // 対応付けする
    let mut mp = HashMap::<usize, usize>::new();
    let mut used = HashSet::<usize>::new();
    for (i2, b2) in block2.iter().enumerate() {
        for (i1, b1) in block1.iter().enumerate() {
            if !used.contains(&i1) && is_same(b1, b2) {
                mp.insert(i2, i1);
                used.insert(i1);
                break;
            }
        }
    }

    // 答えを更新
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

    // 新しく構築し直し
    let l = block1.len() + nullcnt;
    block1.clear();
    block2.clear();
    block1.resize(l, vec![]);
    block2.resize(l, vec![]);
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans1[i][j][k] != 0 {
                    block1[ans1[i][j][k] - 1].push((i, j, k));
                }
                if ans2[i][j][k] != 0 {
                    block2[ans2[i][j][k] - 1].push((i, j, k));
                }
            }
        }
    }
}

fn remove_invisible_blocks(
    d: usize,
    block1: &mut Vec<Vec<(usize, usize, usize)>>,
    block2: &mut Vec<Vec<(usize, usize, usize)>>,
    ans1: &mut Vec<Vec<Vec<usize>>>,
    ans2: &mut Vec<Vec<Vec<usize>>>,
) {
    let mut filled1f = vec![vec![false; d]; d];
    let mut filled1r = vec![vec![false; d]; d];
    let mut filled2f = vec![vec![false; d]; d];
    let mut filled2r = vec![vec![false; d]; d];
    let mut todocommon = Vec::<usize>::new();
    let mut todolater11 = Vec::<usize>::new();
    let mut todolater12 = Vec::<usize>::new();
    let mut todolater21 = Vec::<usize>::new();
    let mut todolater22 = Vec::<usize>::new();
    for i in 0..block1.len().max(block2.len()) {
        let mut need_check = false;

        // 片方でしか使われてないやつはスキップ
        let mut skip = false;
        if i >= block1.len() || block1[i].len() == 0 {
            todolater21.push(i);
            skip = true;
        }
        if i >= block2.len() || block2[i].len() == 0 {
            todolater11.push(i);
            skip = true;
        }
        if skip {
            continue;
        }

        for &(x, y, z) in &block1[i] {
            if !filled1f[x][z] && !filled1r[y][z] {
                need_check = true;
                break;
            }
        }
        for &(x, y, z) in &block2[i] {
            if !filled2f[x][z] && !filled2r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            // 残す
            for &(x, y, z) in &block1[i] {
                filled1f[x][z] = true;
                filled1r[y][z] = true;
            }
            for &(x, y, z) in &block2[i] {
                filled2f[x][z] = true;
                filled2r[y][z] = true;
            }
        } else {
            todocommon.push(i);
        }
    }
    for i in todocommon {
        let mut need_check = false;

        for &(x, y, z) in &block1[i] {
            if !filled1f[x][z] || !filled1r[y][z] {
                need_check = true;
                break;
            }
        }
        for &(x, y, z) in &block2[i] {
            if !filled2f[x][z] || !filled2r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            // 残す
            for &(x, y, z) in &block1[i] {
                filled1f[x][z] = true;
                filled1r[y][z] = true;
            }
            for &(x, y, z) in &block2[i] {
                filled2f[x][z] = true;
                filled2r[y][z] = true;
            }
        } else {
            // 消す
            for &(x, y, z) in &block1[i] {
                ans1[x][y][z] = 0;
            }
            for &(x, y, z) in &block2[i] {
                ans2[x][y][z] = 0;
            }
            block1[i].clear();
            block2[i].clear();
        }
    }
    for i in todolater11 {
        let mut need_check = false;
        for &(x, y, z) in &block1[i] {
            if !filled1f[x][z] && !filled1r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            for &(x, y, z) in &block1[i] {
                filled1f[x][z] = true;
                filled1r[y][z] = true;
            }
        } else {
            todolater12.push(i);
        }
    }
    for i in todolater21 {
        let mut need_check = false;
        for &(x, y, z) in &block2[i] {
            if !filled2f[x][z] && !filled2r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            for &(x, y, z) in &block2[i] {
                filled2f[x][z] = true;
                filled2r[y][z] = true;
            }
        } else {
            todolater22.push(i);
        }
    }
    for i in todolater12 {
        let mut need_check = false;
        for &(x, y, z) in &block1[i] {
            if !filled1f[x][z] || !filled1r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            for &(x, y, z) in &block1[i] {
                filled1f[x][z] = true;
                filled1r[y][z] = true;
            }
        } else {
            for &(x, y, z) in &block1[i] {
                ans1[x][y][z] = 0;
            }
            block1[i].clear();
        }
    }
    for i in todolater22 {
        let mut need_check = false;
        for &(x, y, z) in &block2[i] {
            if !filled2f[x][z] || !filled2r[y][z] {
                need_check = true;
                break;
            }
        }
        if need_check {
            for &(x, y, z) in &block2[i] {
                filled2f[x][z] = true;
                filled2r[y][z] = true;
            }
        } else {
            for &(x, y, z) in &block2[i] {
                ans2[x][y][z] = 0;
            }
            block2[i].clear();
        }
    }
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

    let mut stats = (0, 0, 0, 0);

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
        stats.2 += 1;

        let mut new_block1 = block1[idx1].clone();
        let mut changed1 = 998244353;
        let mut new_block2 = block2[idx2].clone();
        let mut changed2 = 998244353;
        for &(x1, y1, z1) in &block1[idx1] {
            for dir1 in 0..6usize {
                let nx1 = (x1 as i32 + DX[dir1]) as usize;
                let ny1 = (y1 as i32 + DY[dir1]) as usize;
                let nz1 = (z1 as i32 + DZ[dir1]) as usize;
                // 範囲外
                if nx1 >= d || ny1 >= d || nz1 >= d {
                    continue;
                }
                // すでに同じブロック
                if ans1[nx1][ny1][nz1] == ans1[x1][y1][z1] {
                    continue;
                }
                if ans1[nx1][ny1][nz1] > 0 {
                    let oldidx = ans1[nx1][ny1][nz1] - 1;
                    // if block1[oldidx].len() != 1 {
                    //     continue;
                    // }
                    new_block1.extend(block1[oldidx].iter());
                    changed1 = oldidx;

                    for &(x2, y2, z2) in &block2[idx2] {
                        for dir2 in 0..6usize {
                            let nx2 = (x2 as i32 + DX[dir2]) as usize;
                            let ny2 = (y2 as i32 + DY[dir2]) as usize;
                            let nz2 = (z2 as i32 + DZ[dir2]) as usize;
                            // 範囲外
                            if nx2 >= d || ny2 >= d || nz2 >= d {
                                continue;
                            }
                            // すでに同じブロック
                            if ans2[nx2][ny2][nz2] == ans2[x2][y2][z2] {
                                continue;
                            }
                            if ans2[nx2][ny2][nz2] > 0 {
                                let oldidx = ans2[nx2][ny2][nz2] - 1;
                                // if block2[oldidx].len() != 1 {
                                //     continue;
                                // }
                                new_block2.extend(block2[oldidx].iter());
                                if is_same(&new_block1, &new_block2) {
                                    changed2 = oldidx;
                                    break;
                                } else {
                                    // revert
                                    for _ in 0..block2[oldidx].len() {
                                        new_block2.pop();
                                    }
                                }
                            }
                        }
                        if changed2 != 998244353 {
                            break;
                        }
                    }
                    if changed2 != 998244353 {
                        break;
                    } else {
                        // revert
                        for _ in 0..block1[oldidx].len() {
                            new_block1.pop();
                        }
                        changed1 = 998244353;
                    }
                }
            }
            if changed1 != 998244353 {
                break;
            }
        }
        if changed1 == 998244353 {
            stats.3 += 1;
            continue;
        }

        if changed2 != 998244353 {
            // apply
            for &(x, y, z) in &block1[changed1] {
                ans1[x][y][z] = idx1 + 1;
            }
            for &(x, y, z) in &block2[changed2] {
                ans2[x][y][z] = idx2 + 1;
            }
            mem::swap(&mut block1[idx1], &mut new_block1);
            mem::swap(&mut block2[idx2], &mut new_block2);
            block1[changed1].clear();
            block2[changed2].clear();
            stats.1 += 1;
        }
    }
    update_block_id(d, &mut block1, &mut block2, &mut ans1, &mut ans2);
    remove_invisible_blocks(d, &mut block1, &mut block2, &mut ans1, &mut ans2);
    update_block_id(d, &mut block1, &mut block2, &mut ans1, &mut ans2);

    // 出力
    eprintln!(
        "Stats: attempt={}, success={}, initialCheckPassed={}, NotApplied={}",
        stats.0, stats.1, stats.2, stats.3
    );
    output(ans1, ans2);
}
