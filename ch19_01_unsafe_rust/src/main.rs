use std::slice;

fn deference_raw_pointer() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let _r = address as *const i32;
    // segmentation fault:
    // unsafe {
    //     println!("r is: {}", *_r);
    // }
}

unsafe fn unsafe_function() {
    println!("unsafe function called!");
}

// unsafe abstraction
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn unsafe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// creation of Foreign Function Interface (FFI)
// C defined which Application Binary Interface (ABI) to use (how to call at assembly level)
fn extern_code_execution() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    let res = unsafe {
        abs(-1)
    };
    println!("extern C abs: {}", res);
}

// create ABI for C from Rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {
    deference_raw_pointer();

    unsafe {
        unsafe_function();
    }

    unsafe_abstraction();

    extern_code_execution();
}
