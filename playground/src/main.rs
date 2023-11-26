use std::ops::Deref;

fn main() {
    let x = Box::new(String::from("test"));
    take_str(&(*(*x)));
    take_str(&(*x)[..]);
    take_str(x.deref().deref());
    take_str(&x);

    // deref coercion calls Deref::deref as many times as necessary to get a reference
    // to match the parameter's type
    print_string_ptr(&x);
    take_str(&x);
    print_string(*x);
}

fn print_string(str: String) {
    println!("str is: {}", str);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn print_string_ptr(string: &String) {
    println!("str is: {}", string);
}

fn take_str(str: &str) {
    println!("str is: {}", str);
}

fn sum_elements(arr: [i32; 1000]) -> i32 {
    arr.iter().sum()
}

fn check() {
    fn main() {
        let x = 10;
        let large_array = [1; 1000]; // A large array initialized with 1s
        let array_ptr = &large_array; // Regular reference to the array

        // Use dereference operator to pass the array to the function
        let sum = sum_elements(*array_ptr);
        println!("Sum with regular pointer: {}", sum);
        let x = MyBox::new(10);
        println!("Sum with regular pointer: {}", *x);
    }
}
