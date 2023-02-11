use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    // we can convert references into raw pointers
    // this code is safe, but dereferences raw pointers is not
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // you can also create a pointer directly
    // this is also unsafe to deref
    let address = 0x012345usize;
    let r3 = address as *const i32;

    // we can dereference unsafely, but must be explicit about it
    // with an unsafe wrapper. The value could be outside of valid memory for this
    // program, and cause a segfault. Normally we could not have a immutable and
    // mutable reference to the same data, but raw pointers do not have this
    // restriction.
    unsafe {
        println!("r1 is: {}", *r1);
        *r2 = 10;
        println!("r2 is: {}", *r2);
        // println!("r3 is: {}", *r3); // this will likely segfault
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // all calls to non-rust code is unsafe, since rust cannot guarantee its safe
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing and modifying static mutable variables is unsafe. This is because
    // there is no guarantee to avoid race conditions from multiple threads working
    // against a single static variable
    println!("name is: {}", HELLO_WORLD);
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", COUNTER);
    }
}

// we can mark functions as unsafe, which can then only be called from within
// unsafe blocks. The entire body of an unsafe function is equivalent to being
// in an unsafe block.
unsafe fn dangerous() {
    println!("This is unsafe!");
}

// an example of a safe API for an unsafe action. Be sure to test thoroughly.
// Rust wont allow us to borrow diffrent parts of the same slice, even though
// we know its safe because we have ensured those slices dont overlap.
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

extern "C" {
    fn abs(input: i32) -> i32;
}

// any trait that has unsafe methods must be marked as unsafe
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
