extern crate rand;
use rand::Rng;

fn sort(mut unsorted: Vec<i32>) -> Vec<i32> {
    if unsorted.len() < 2 {
        return unsorted
    }
    let mut result = Vec::with_capacity(unsorted.len());
    while unsorted.len() > 0 {
        let mut sublist = vec![unsorted.remove(0)];
        let mut leftovers = Vec::new();
        let mut last = sublist.get(0).unwrap().clone();

        for item in unsorted {
            if item >= last {
                sublist.push(item);
                last = item;
            } else {
                leftovers.push(item);
            }
        }
        result = merge(&result, &sublist);
        unsorted = leftovers;
    }
    result
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    if left.len() == 0 {
        return right.clone()
    } else if right.len() == 0 {
        return left.clone()
    }

    let mut result: Vec<i32> = Vec::with_capacity(left.len() + right.len());
    let mut a_dx: usize = 0;
    let mut b_dx: usize = 0;
    while a_dx < left.len() && b_dx < right.len() {
        if left[a_dx] <= right[b_dx] {
            result.push(left[a_dx].clone());
            a_dx += 1;
        } else {
            result.push(right[b_dx].clone());
            b_dx += 1;
        }
    }

    if a_dx < left.len() {
        result.extend(left[a_dx..].iter());
    }
    if b_dx < right.len() {
        result.extend(right[b_dx..].iter());
    }
    result
}

fn main() {
    let mut vec = Vec::new();
    for _ in 1..11 {
        vec.push(rand::thread_rng().gen_range(0, 100));
    }
    println!("The original list is: {:?}", vec);

    println!("{:?}", sort(vec));
}
