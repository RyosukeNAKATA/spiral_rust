use std::io;

fn main() {
    let n: usize = read();
    let mut a: Vec<i32> = read_vec();

    let mut ans = 0;

    for i in 0..n - 1 {
        let mut minj = i;
        for j in i..n {
            if a[j] < a[minj] {
                minj = j;
            }
        }
        if minj != i {
            ans += 1;
        }
        a.swap(i, minj);
    }

    print_vec(a);
    println!("{}", ans);
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
fn print_vec(v: Vec<i32>) {
    for i in 0..v.len() {
        if i != v.len() - 1 {
            print!("{} ", v[i]);
        } else {
            println!("{}", v[i]);
        }
    }
}
