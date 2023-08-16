use std::io;

fn main() {
    let n: usize = read();
    let a: Vec<String> = read_vec();

    let mut a_bubble = a.copy();
    let mut a_selection = a.copy();

    let sorted_bubble = sort_bubble(a_bubble, n);
    let sorted_selection = sort_selection(a_selection, n);

    print_vec(sorted_bubble);
    println!({}, is_stable(sorted_bubble));
    print_vec(sorted_selection);
    println!({}, is_stable(sorted_selection));
}

fn sort_bubble(a: Vec<()>, n: i32) -> Vev<()> {
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
    a
}

fn sort_selection(a: Vec<()>, n: i32) -> Vec<()> {
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
    a
}

fn is_stable() {}

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
fn print_vec(v: Vec) {
    for i in 0..v.len() {
        if i != v.len() - 1 {
            print!("{} ", v[i]);
        } else {
            println!("{}", v[i]);
        }
    }
}
