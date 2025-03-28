extern crate sector;

// Imports the Sector and its state
use sector::{states::Normal, Sector};

#[cfg(feature = "std")]
fn main() {
    // Creating a new sector with no initial capacity
    let mut sector: Sector<Normal, _> = Sector::new();
    sector.push("Welcome To Sector!");
    // It is also possible to initialize the sector like this
    let mut sector = Sector::<Normal, _>::new();

    // Our capacity is now 0, as long as we used a non Zero-Sized-Type (ZST)
    // To get to know ZSTs better I recommend reading: https://doc.rust-lang.org/nomicon/exotic-sizes.html
    assert_eq!(sector.capacity(), 0);

    // We are able to push and pop just like a normal Vector!
    sector.push(201);
    sector.push(202);
    sector.push(203);
    sector.push(204);
    sector.push(205);
    assert_eq!(sector.pop(), Some(205));

    // Iterating is the same as we know it from a convetional Vector
    for (index, ele) in sector.iter().enumerate() {
        assert_eq!(*ele, 200 + index + 1);
    }

    // Let's initialize a Vector with the same content of the Sector
    let mut vector = vec![201, 202, 203, 204];

    // Now we look if the growing and shrinking of the capacity behaves the same
    assert_eq!(vector.capacity(), sector.capacity());

    for _ in 0..5 {
        let _ = vector.pop();
        let _ = sector.pop();
    }

    assert_eq!(vector.capacity(), sector.capacity());
    // I hope it comes to no ones suprise. They behave the same!
}

#[cfg(not(feature = "std"))]
fn main() {
    // Creating a new sector with no initial capacity
    let mut sector: Sector<Normal, _> = Sector::new();
    sector.push("Welcome To Sector!");
    // It is also possible to initialize the sector like this
    let mut sector = Sector::<Normal, _>::new();

    // Our capacity is now 0, as long as we used a non Zero-Sized-Type (ZST)
    // To get to know ZSTs better I recommend reading: https://doc.rust-lang.org/nomicon/exotic-sizes.html
    assert_eq!(sector.capacity(), 0);

    // We are able to push and pop just like a normal Vector!
    sector.push(201);
    sector.push(202);
    sector.push(203);
    sector.push(204);
    sector.push(205);
    assert_eq!(sector.pop(), Some(205));

    // Iterating is the same as we know it from a convetional Vector
    for (index, ele) in sector.iter().enumerate() {
        assert_eq!(*ele, 200 + index + 1);
    }

    for _ in 0..5 {
        let _ = sector.pop();
    }

    assert_eq!(0, sector.capacity());
    // I hope it comes to no ones suprise. They behave the same!
}
