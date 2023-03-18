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

fn main() {
    let (d, sil1, sil2) = input();
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

    // 出力
    let mut cnt = 0;
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans1[i][j][k] > 0 {
                    cnt += 1;
                    ans1[i][j][k] = cnt;
                }
                if ans2[i][j][k] > 0 {
                    cnt += 1;
                    ans2[i][j][k] = cnt;
                }
            }
        }
    }
    println!("{}", cnt);
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                print!("{} ", ans1[i][j][k]);
            }
        }
    }
    println!("");
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                print!("{} ", ans2[i][j][k]);
            }
        }
    }
    println!("");
}
