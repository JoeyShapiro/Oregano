use std::mem;
use std::slice;

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    slice::from_raw_parts(
        (p as *const T) as *const u8,
        mem::size_of::<T>(),
    )
}

fn main() {
    struct MyStruct {
        pos: u8,
        health: u8,
        id: u8,
        enemies: Vec<u8>,
        data: String,
        ch: char,
    }
    let mut enemies = Vec::new();
    enemies.push(100);
    let my_struct = MyStruct { pos: 0x1A, health: 0xFF, id: 42, enemies, data: "ABCD".to_owned(), ch: 'B' };
    let bytes: &[u8] = unsafe { any_as_u8_slice(&my_struct) };
    // tcp_stream.write(bytes);
    println!("{:?}", bytes);
}