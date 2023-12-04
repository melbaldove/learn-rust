struct TestStruct {
    a: String,
    b: String,
}

fn main() {
    let testStruct = TestStruct {
        a: String::from("testa"),
        b: String::from("testb"),
    };

    let moved_a = testStruct.a;
    mutateTestStruct(&testStruct);
}

fn mutateTestStruct(mut testStruct: &TestStruct) {}
