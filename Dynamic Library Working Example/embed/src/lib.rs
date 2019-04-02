#[no_mangle]
pub extern fn shrek(cycle_count: i32) -> () {
    let mut first_f = 1;
    let mut second_f = 1;
    let mut i = 2;
    for _ in 0..cycle_count {
        let third_f = first_f + second_f;
        println!("F1 = {}; F2 = {}; F3 = {}", first_f, second_f, third_f);
        first_f = second_f;
        second_f = third_f;
        i += 1;
    }
    println!("Fibonacci number count: {}", i);
}