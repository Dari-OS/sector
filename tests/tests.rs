use sector::{
    states::{Manual, Normal},
    Sector,
};
#[test]
fn test_len() {
    let mut sec = Sector::<Normal, _>::new();
    sec.push("Test".to_string());
    sec.push("Hello".to_string());
    sec.push("World".to_string());
    assert_eq!(3, sec.len())
}

#[test]
fn test_len_zst() {
    let mut sec = Sector::<Normal, ()>::new();
    sec.push(());
    sec.push(());
    sec.push(());
    assert_eq!(3, sec.len())
}

#[test]
fn test_len_empty() {
    let sec = Sector::<Normal, i32>::new();

    assert_eq!(0, sec.len())
}

#[test]
fn test_len_empty_zst() {
    let sec = Sector::<Normal, ()>::new();

    assert_eq!(0, sec.len())
}

#[test]
fn test_cap() {
    let mut sec = Sector::<Manual, i32>::new();
    assert_eq!(sec.capacity(), 0);
    sec.grow(1000);
    assert_eq!(sec.capacity(), 1000);
    sec.shrink(1000);
    assert_eq!(sec.capacity(), 0);
}

#[test]
fn test_cap_zst() {
    let mut sec = Sector::<Manual, ()>::new();
    assert_eq!(sec.capacity(), usize::MAX);
    assert_eq!(sec.grow(1000), 0);
    assert_eq!(sec.capacity(), usize::MAX);
    assert_eq!(sec.shrink(1000), 0);
    assert_eq!(sec.capacity(), usize::MAX);
}

#[test]
fn test_creation() {
    let mut sec1 = Sector::<Normal, u32>::new();
    assert_eq!(sec1.capacity(), 0);
    assert_eq!(sec1.len(), 0);
    assert_eq!(sec1.pop(), None);

    let mut sec2 = Sector::<Normal, u32>::with_capacity(100);
    assert_eq!(sec2.capacity(), 100);
    assert_eq!(sec2.len(), 0);
    assert_eq!(sec2.pop(), None);

    let mut sec3 = Sector::<Normal, u32>::try_with_capacity(100).unwrap();
    assert_eq!(sec3.capacity(), 100);
    assert_eq!(sec3.len(), 0);
    assert_eq!(sec3.pop(), None);

    let sec4 = Sector::<Normal, u32>::try_with_capacity(usize::MAX);
    assert!(sec4.is_err())
}
