use std::io;

#[warn(dead_code)]
fn main() {
    let n: usize = read();
    let mut maxv = -1_000_000_000;
    let mut minv: i32 = read();

    for _ in 1..n {
        let r: i32 = read();
        maxv = maxv.max(r - minv);
        minv = minv.min(r);
    }

    println!("{}", maxv);
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
