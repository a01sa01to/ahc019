use std::io;

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
    let mut ans1 = vec![vec![vec![true; d]; d]; d];
    let mut ans2 = vec![vec![vec![true; d]; d]; d];
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if !sil1.0[k][i] {
                    ans1[i][j][k] = false;
                }
                if !sil1.1[k][j] {
                    ans1[i][j][k] = false;
                }
                if !sil2.0[k][i] {
                    ans2[i][j][k] = false;
                }
                if !sil2.1[k][j] {
                    ans2[i][j][k] = false;
                }
            }
        }
    }
    let mut cnt = 0;
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans1[i][j][k] {
                    cnt += 1;
                }
                if ans2[i][j][k] {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
    let mut idx = 1;
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans1[i][j][k] {
                    print!("{} ", idx);
                    idx += 1;
                } else {
                    print!("0 ");
                }
            }
        }
    }
    println!("");
    for i in 0..d {
        for j in 0..d {
            for k in 0..d {
                if ans2[i][j][k] {
                    print!("{} ", idx);
                    idx += 1;
                } else {
                    print!("0 ");
                }
            }
        }
    }
}
