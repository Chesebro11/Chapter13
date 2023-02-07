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

// impl <T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//     // This means F must be able to be called once, take no arguments, and return a T
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }

// Using sort_by_key to order rectangles by width
// #derive(Debug)
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }

// Attempting to use an fnOnce closure with sort_by_key
// This is a convoluted way that doesnt work to try and count the number of times sory_by_key gets called when sorting list.
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12},
//     ];

//     let mut sort_operations = vec![];
//     let value = String::from("by key called");

//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{:#?}", list);
// }

// Using an FnMut closure with sort_by_key
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut num_sort_operations = 0;
//     list.sort_by_key(|r| {
//         num_sort_operations += 1;
//         r.width
//     });
//     println!("{:#?}, sorted in {num_sort_operations} operations", list);
// }

// Processing a Series of Items with iterators

// The iterator pattern allows you to perform some task on a sequence of items in turn
// An iterator is responsible for the logic of iterating over each item and determing 
// When the sequence has finished. You don't have to implement the logic yourself!

// Using an iterator in a for loop
// let v1 = vec![1, 2, 3];

// let v1_iter = v1.iter();

// for val in v1_iter {
//     println!("Got: {}", val)
// }

// The Iterator Trait and the next Method
// pub trait Iterator {
//     type Item;

//     fn next (&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }
// New syntax is introduced here. type Item, and Self::Item.
// These are defining an associated type with this trait.
// This code says implementing the Iterator trait requires that you also define an item type
// the Item type will be the type return from the iterator

// Calling the next method on an iterator
// v1_iter has to be mutable because each next call eats up an item from the iterator
// #[test]
// fn iterator_demonstration() {
//     let v1 = vec![1, 2, 3];

//     let mut v1_iter = v1.iter();

//     assert_eq!(v1_iter.next(), Some(&1));
//     assert_eq!(v1_iter.next(), Some(&2));
//     assert_eq!(v1_iter.next(), Some(&3));
//     assert_eq!(v1_iter.next(), None);

// }

// Methods that Consume the Iterator
// Calling the sum method to get the total of all items in the iterator
// #[test]
// fn iterator_sum() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);

// }
// WE cant use v1_iter after the call to sum because sum takes ownership of the iterator we call it on

// Methods that PRoduce Other Iterators

// let v1: Vec<i32> = vec![1, 2, 3];

// v1.iter().map(|x| x + 1);
// Calling the iterator adaptor map to create a new iterator
// #[test]
// fn iterator_sum() {
// let v1: Vec<i32> = vec![1, 2, 3];

// let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

// assert_eq!(v2, vec![2, 3, 4]);
// }
// Calling the map method to create a new iterator and then calling the collect method to
// consume the new iterator and create a vector 

// Using Closures that Capture Their Environment