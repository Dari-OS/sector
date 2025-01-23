use sector::{sector::Normal, Sector};

#[test]
fn first_test() {
    let mut sec = Sector::<_, Normal>::new();
    sec.push("Test".to_string());
    sec.push("Hello".to_string());
    sec.push("World".to_string());
    assert!(sec.pop() == Option::Some("World".to_string()));
    assert!(sec.pop() == Option::Some("Hello".to_string()));
    assert!(sec.pop() == Option::Some("Test".to_string()));
    assert!(sec.pop().is_none());
    drop(sec);
}
