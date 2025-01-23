use sector::{sector::Normal, Sector};

#[test]
fn first_test() {
    let mut sec = Sector::<String, Normal>::new();
    sec.push("Test".to_string());
    sec.push("Hello".to_string());
    sec.push("World".to_string());
    print!("{:?}", sec.pop());
}
