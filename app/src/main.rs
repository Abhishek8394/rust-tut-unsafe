use std::ops::{Add};
use std::slice;
use std::fmt;
use unsafe_tut_core;
mod overload;
use crate::overload::{overload_demo};

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

        unsafe_tut_core::hello_rust();

        {
            println!("\nOperator overloading.");
            let x = Point{x: 1, y: 10};
            let y = Point{x: 9, y: 100};
            println!("x {:?} + y {:?}", x, y);
            let res = x + y;
            println!("= {:?}", res);
            assert_eq!(Point{x: 10, y: 110}, res);
            println!("Adding 2 diff types");
            let mm = Millimeters(900);
            let m = Meters(1);
            println!("{:?} + {:?}", mm, m);
            let res = mm + m;
            println!("{:?}", res);
            println!("Same method name overloading..");
            overload_demo();
            // demo for when overloading cannot be done.
        }
        {
            println!("\nSupertrait demo");
            // Since Outlinedemo requires Display trait to be implemented, 
            println!("Display trait is supertrait of outlinedemo.");
            
            trait OutlineDemo: fmt::Display{
                fn outline_print(&self){
                    let output = self.to_string();
                    let len = output.len();
                    println!("{}", "*".repeat(len + 4));
                    println!("*{}*", " ".repeat(len + 2));
                    println!("* {} *", output);
                    println!("*{}*", " ".repeat(len + 2));
                    println!("{}", "*".repeat(len + 4));
                }
            }

            struct DisplayPoint{x: i32, y:i32}
            impl fmt::Display for DisplayPoint{
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                    write!(f, "({}, {})", self.x, self.y)
                }
            }
            impl OutlineDemo for DisplayPoint{}
            let dp = DisplayPoint{x: -10, y: 5};
            dp.outline_print();
        }
        {
            println!("\nNewtype Pattern");
            // We are going to implement trait we dont own for types we dont own!
            struct Wrapper(Vec<String>);
            impl fmt::Display for Wrapper{
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                    write!(f, "[{}]", self.0.join(", "))
                }
            }
            // how do we get length of inner vector now? Deref to rescue!
            use std::ops::Deref;
            impl Deref for Wrapper{
                type Target = Vec<String>;
                fn deref (&self) -> &Self::Target{
                    &self.0
                }
            }

            let v1 = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("v1 = {}", v1);
            println!("size of v1 = {}", v1.len());
        }

        {
            println!("\nType aliases");
            type Kilometer = i32;
            let x = 5;
            let y: Kilometer = 5;
            assert_eq!(x, y);
            println!("i32 x: {:?} + Kilometer y: {:?} = {:?}", x, y, x + y);
        }
        {
            println!("\nNever return type - \"!\"");
            for guess in ["10", "hello", "4"].iter(){
                println!("For guess = {}", guess);
                // match requires everything to be of same type yet we still return continue.
                // This is because continue is of never return type "!"
                // Empty types (never return type) can be considered as any type for these purposes.
                let value: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    // panic! also returns "!"
                    Err(_) => continue,
                };
                println!("value = {}", value);
            }
        }
        {
            println!("\nDynamically sized types");
            // Below will error because rust needs to know how much memory a type needs
            // at compile time. However, `str` is a dynamically sized type so we wont know size
            // at compile time.
            // let s1: str = "hello";
            // let s2: str = "how are ya?";
            // Solution: Use &str. &str contains a mem address and a usize field denoting len. This
            // package will always be constant size.
            let s1: &str = "hello";
            let s2: &str = "how are ya?";
            println!("s1 = {:?}, s2 = {:?}", s1, s2);
            // All generic functions assume type has Size avail at compile time. To allow a type
            // that is of dynamic size, do following.
            // &T is a must because if it is a dyn sized type, we need refs at compile time!
            fn <T: ?Sized>foobar(s: &T) -> !{
                // println!("{:?}", T);
            }
        }
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

#[derive(Debug, PartialEq)]
struct Point{
    x: i32, y: i32
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

/// This only allows `mm + m` but not `m + mm`.
impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, m: Meters) -> Millimeters{
        Millimeters(self.0 + m.0 * 1000)
    }
}

/// To allow `let res = m + mm;` we need to implement for add in that direction.
impl Add<Millimeters> for Meters{
    type Output = Millimeters;
    fn add(self, mm: Millimeters) -> Millimeters{
        Millimeters(self.0 * 1000 + mm.0)
    }
}