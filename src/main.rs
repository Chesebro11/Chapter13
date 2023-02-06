#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts{
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main () {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

//  Adding optional type annotations of the parameter and return value types in the closure
// let expensive_closure = |num: u32| -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     num
// };

// First line here shows a function defintion
// fn add_one_v1 (x: u32) -> u32 { x + 1 }
// second line shows a fully annotated closure definition
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// Third line we remove the type annotations from the closure definition
// let add_one_v3 = |x|             {x + 1};
// Fourth line we remove all brackets, which are optional
// These are all valid definitions that will produce the same behavior when theyre called.
// let add_one_v4 = |x|              x + 1;

// Attempting to call a closure whose tyeps are inferred with two different types.
// Calling with a string the first time then calling with an integer produces an error.
// let example_closure = |x| x;
// Calling example_closure the first time with the String value allows the compiler to infer the type of X and return the closure type to be string.
// let s = example_closure(String::from("hello"));
// let n = example_closure(5);

// Capturing References or Moving Ownership

// Closures can capture values in three ways, borrowing immutably, borrowing mutably, and taking ownership

// Defining and calling a closure that captures an immutable reference
// fn main () {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let only_borrows = || println!("From closure: {:?}", list);

//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }

// Defining and calling a closure that captures a mutable reference
// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     let mut borrows_mutably = || list.push(7);

//     borrows_mutably();
//     println!("After calling closure: {:?}", list);
// }

// use std::thread;
// // Using move to force the closure for the thread to take ownership of list
// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     thread::spawn(move || println!("From thread: {:?}", list))
//         .join()
//         .unwrap();
// }

// Moving Captured Values Out of Closures and the Fn Traits

impl <T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}