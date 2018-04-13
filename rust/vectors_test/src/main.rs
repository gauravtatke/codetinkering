fn print_vec_iter(vecref: &Vec<u32>) {
    for val in vecref {
        println!("value is = {}", val);
    }
}

fn print_vec_by_index(vecref: &Vec<u32>) {
    let mut i = 0;
    while i < vecref.len() {
        println!("Value at index {} is {}", i, vecref[i]);
        i = i+1;
    }
}

fn main() {
    let v = vec![34, 41, 56];
    print_vec_iter(&v);
    print_vec_by_index(&v);
}
