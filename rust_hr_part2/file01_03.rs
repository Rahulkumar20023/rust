enum MyEnum {
    Variant1(i32),
    Variant2(String), // Non-Copy field makes the whole enum non-Copy
}

fn takes_enum(e: MyEnum) {
    // e's ownership is moved here
}

fn main() {
    let e = MyEnum::Variant2(String::from("hello"));
    takes_enum(e);
    // e is no longer accessible here
}
