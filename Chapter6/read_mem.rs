fn main() {
    let mut non_zero = 0;
    for i in 1..10_000 {
        let ptr = i as *const u8;
        let byte_at_addr = unsafe { * ptr };
        if byte_at_addr != 0 {
            non_zero += 1;
        }
    }
    println!("non_zero count: {}", non_zero);
}
