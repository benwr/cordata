// UUIDs are way bigger than we need, and we do care about the size of
// this struct. If there were 10^9 nodes collaborating on the same
// document, 10 bytes give a ~10^-8  chance of a collision. With
// "only" 10^6 nodes collaborating, you get ~10^-13. I'll take those odds.
pub struct NodeId([u8; 10]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
