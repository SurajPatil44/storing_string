
#[derive(Debug,Default)]
struct Trie {
    //root : Node,
    terminal : bool,
    children : Vec<Option<Trie>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default(){
        let t = Trie::default();
        dbg!(t);
    }
}