use std::slice;

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    /*
    The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and
    must trust that this pointer is valid. The add method on raw pointers is also unsafe, 
    because it must trust that the offset location is also a valid pointer
     */
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}