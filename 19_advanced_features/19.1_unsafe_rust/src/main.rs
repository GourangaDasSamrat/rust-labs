use std::slice;

// Learning note: `unsafe` unlocks a small set of extra operations.
// The code below keeps each example isolated so the risk is easy to see.

unsafe extern "C" {
    // This C function is safe to call for any `i32` input, so we mark it safe.
    safe fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

// Learning note: an unsafe trait means the compiler cannot verify some invariant.
// We promise that any type implementing this trait satisfies that invariant.
unsafe trait TrustedLayout {}

unsafe impl TrustedLayout for i32 {}

// Learning note: a union stores one field at a time.
// Reading a field is unsafe because Rust cannot know which variant is active.
#[repr(C)]
union IntOrBytes {
    number: u32,
    bytes: [u8; 4],
}

unsafe fn dangerous() {
    println!("called an unsafe function");
}

// SAFETY: `mid` is checked against the slice length, so the two returned
// slices are non-overlapping and point into the original slice.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// SAFETY: This example is single-threaded, so the mutable static is accessed
// from only one place at a time.
unsafe fn add_to_count(inc: u32) {
    COUNTER += inc;
}

fn main() {
    let mut num = 5;

    // Learning note: raw pointers can coexist even when references could not.
    let raw_const = &raw const num;
    let raw_mut = &raw mut num;

    unsafe {
        println!("raw const points to: {}", *raw_const);
        *raw_mut = 6;
        println!("raw mut updated value: {}", *raw_mut);
    }

    // Learning note: calling an unsafe function requires an unsafe block.
    unsafe {
        dangerous();
    }

    let mut values = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut values, 3);
    println!("split slices: {:?} | {:?}", left, right);

    // `abs` comes from C, but this specific function is safe to call.
    println!("abs(-3) from C: {}", abs(-3));

    unsafe {
        add_to_count(3);
        // Learning note: use a raw pointer to read a mutable static.
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    let value = IntOrBytes {
        number: 0x1234_5678,
    };

    unsafe {
        println!("union bytes: {:02x?}", value.bytes);
    }

    let _marker: &dyn TrustedLayout = &42;
}
