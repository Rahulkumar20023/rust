#[derive(Copy, Clone)]
enum MyEnum {
    Variant1(u32),
    Variant2(bool),
}

fn takes_enum(e: MyEnum) {
    // e is copied here
}

fn main() {
    let e = MyEnum::Variant1(10);
    takes_enum(e);
    // e is still accessible here
}
