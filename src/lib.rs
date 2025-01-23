pub mod components;
//TODO: Make a cleaner API interface
pub mod sector;
pub use sector::Sector;

#[cfg(test)]
mod tests {
    use super::*;
}
