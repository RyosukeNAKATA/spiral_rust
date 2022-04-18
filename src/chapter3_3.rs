use std::io;

#[warn(dead_code)]
fn main() {
    let n: usize = read();
    let mut a: Vec<i32> = read_vec();

    let mut flag = true;
    let mut counter = 0;
    while flag {
        flag = false;
        for j in (1..n).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
                flag = true;
                counter += 1;
            }
        }
    }

    for k in 0..n {
        if k != n - 1 {
            print!("{} ", a[k]);
        } else {
            println!("{}", a[k]);
        }
    }
    println!("{}", counter);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
