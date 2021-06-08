fn main() {
    alignment();
}

fn alignment() {
    #[repr(C)]
    struct Foo {
        tiny: bool,
        normal: u32,
        small: u8,
        long: u64,
        short: u16,
    }

    #[allow(dead_code)]
    struct Bar {
        tiny: bool,
        normal: u32,
        small: u8,
        long: u64,
        short: u16,
    }

    assert_eq!(std::mem::size_of::<Foo>(), 32);
    assert_eq!(std::mem::size_of::<Bar>(), 16);
}
