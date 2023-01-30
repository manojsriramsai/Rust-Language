fn main() {
    numbers(20);
}

fn numbers(num:i32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is even",n);
        }
        else {
            println!("{} is odd",n);
        }
    }
}

fn is_even(num:i32)-> bool {
    return num%2==0;
}