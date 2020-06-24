use std::slice;
use unsafe_tut;
/// call code defined in C language!
/// This is a FFI - Foreign function interface
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    println!("# Unsafe superpowers");
    println!("- Dereference Raw pointers");
    println!("- Call an unsafe function / method");
    println!("- Access / Modify static variables");
    println!("- Implement unsafe trait");
    println!("- Access fields of `unions`");
    {
        println!("\n# Raw pointers");
        let mut num = 5;
        let r1 = &num as *const i32; // immutable raw ptr
        let r2 = &mut num as *mut i32; // mutable raw ptr
        unsafe{
            println!("r1={:?}, r2={:?}", *r1, *r2);
        }
        let address = 0x12345usize;
        let r = address as *const i32;
        unsafe{
            // below will cause segmentation fault.
            if false{
                println!("value at address {}: {}", address, *r);
            }
        }
    }

    {
        println!("\n# Calling unsafe functions/methods");
        unsafe{
            dangerous();
        }

        // demo for wrapping unsafe code in safe functions
        let mut v = vec![10, 20, 40, 50, 60, 70];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(2);
        assert_eq!(&mut [10, 20], a);
        assert_eq!(&mut [40, 50, 60, 70], b);

        let mut v1 = vec![10, 20, 40, 50, 60, 70];
        let (c, d) = my_split_at_mut(&mut v1[..], 2);
        assert_eq!(&mut [10, 20], c);
        assert_eq!(&mut [40, 50, 60, 70], d);

        unsafe{
            println!("abs(-3) as defined in C language is {}", abs(-3));
        }

        unsafe_tut::hello_rust();
    }
}

unsafe fn dangerous(){}

// below will not work because you're trying to return multiple mutable slices of same list.
// fn my_split_at_mut(v: &mut Vec<i32>, mid: usize) -> (&mut [i32], &mut [i32]){
//     let len = v.len();
//     assert!(mid <= len);
//     (&mut v[0..mid], &mut v[mid..])
// }
fn my_split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = v.len();
    let ptr = v.as_mut_ptr();
    assert!(mid <= len);
    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}
