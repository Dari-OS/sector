extern crate sector;

use sector::{states::Manual, Sector};
fn main() {
    let mut sec: Sector<Manual, _> = Sector::new();

    assert_eq!(sec.grow(10), 10);

    for i in 0..10 {
        let _ = sec.push(i + 1);
    }
}
