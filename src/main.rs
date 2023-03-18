use std::collections::HashSet;
use std::io;

const INF: u32 = 1_000_000_000;

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

fn output(ans1: Vec<Vec<Vec<u32>>>, ans2: Vec<Vec<Vec<u32>>>) {
    let mut set = ans1.iter().flatten().flatten().collect::<HashSet<_>>();
    set.extend(ans2.iter().flatten().flatten());
    eprintln!("{:?}", set);
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

fn main() {
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

    // すでに埋められているか管理
    let mut silh1f = vec![vec![false; d]; d];
    let mut silh1r = vec![vec![false; d]; d];
    let mut silh2f = vec![vec![false; d]; d];
    let mut silh2r = vec![vec![false; d]; d];
    {
        let mut cnt = 0;
        for i in 0..d {
            for j in 0..d {
                for k in 0..d {
                    if ans1[i][j][k] > 0 && (!silh1f[i][k] || !silh1r[j][k]) {
                        cnt += 1;
                        ans1[i][j][k] = cnt;
                        silh1f[i][k] = true;
                        silh1r[j][k] = true;
                    } else {
                        ans1[i][j][k] = 0;
                    }
                }
            }
        }
    }
    {
        let mut cnt = 0;
        for i in 0..d {
            for j in 0..d {
                for k in 0..d {
                    if ans2[i][j][k] > 0 && (!silh2f[i][k] || !silh2r[j][k]) {
                        cnt += 1;
                        ans2[i][j][k] = cnt;
                        silh2f[i][k] = true;
                        silh2r[j][k] = true;
                    } else {
                        ans2[i][j][k] = 0;
                    }
                }
            }
        }
    }

    // 出力
    output(ans1, ans2);
}
