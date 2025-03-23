extern crate sector;

use sector::{states::Dynamic, Sector};

fn main() {
    // We create a sector with a Dynamic state and an initial capacity
    let mut sec: Sector<Dynamic, _> = Sector::with_capacity(5);

    // Now our capacity is 5
    assert_eq!(sec.capacity(), 5);

    sec.push("Hello");
    sec.push(" ");
    sec.push("Rusty");
    sec.push(" ");
    sec.push("World!");

    assert_eq!(sec.capacity(), 5);

    // Transitions to the Locked state
    let sec = sec.to_locked();

    // We can only add/modify/remove data if we transition again

    for word in sec {
        print!("{word}");
    }
}
