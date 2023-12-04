struct TestStruct {
    a: i32,
}

fn main() {
    let boxed = Box::new(TestStruct { a: 1 });
    let mut boxed = boxed;
    boxed.a = 3;
}
