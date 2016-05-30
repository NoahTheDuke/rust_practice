fn sort(mut unsorted: Vec<i32>) -> Vec<i32> {
    if unsorted.len() < 2 {
        return unsorted
    }
    let mut result = Vec::new();
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
        println!("result: {:?}", result);
        println!("sublis: {:?}", sublist);
        result.extend(sublist);
        unsorted = leftovers;
    }
    result
}

fn main() {
    println!("{:?}", sort(vec![2, 1, 3]));
    // println!("{:?}", sort(vec![1]));
}
