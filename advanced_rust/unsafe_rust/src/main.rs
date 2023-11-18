use unsafe_rust::split_at_mut;

extern "C" {
    fn abs(input: i32) -> i32;
}

// Reading from or writing to a mutable static variable is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe { COUNTER += inc };
}

fn main() {
    let mut v = vec![1,2,3,4,5,6];

    let r = &mut v[..];

    let (a,b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1,2,3]);
    assert_eq!(b, &mut [4,5,6]);
    println!("A === {:?}", a);
    println!("B === {:?}", b);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {   // This usage of extern does not require unsafe.
        println!("Just called a Rust function from C!");
    }

    add_to_count(3);
    unsafe { println!("COUNTER: {}", COUNTER); }
}
