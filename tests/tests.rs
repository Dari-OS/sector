use sector::{states::Normal, Sector};
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
    let sec = Sector::<Normal, i32>::new();
}
