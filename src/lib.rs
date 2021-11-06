use std::marker::PhantomData;

#[derive(Debug, Default)]
struct Trie {
    //root : Node,
    terminal: bool,
    children: Vec<Option<Box<Trie>>>,
}

impl Trie {
    fn new() -> Trie {

        let mut trie = Trie::default();
        for _ in 0..26 {
            trie.children.push(None);
        }
        trie
    }

    fn add_child(&mut self, refer: char, terminal: bool) -> Box<Trie> {
        // Adds a child and return it
        let mut new_child = Box::new(Trie::new());
        // seems like it is created in stack
        // and can't be moved as it will 
        new_child.terminal = terminal;
        let place = refer as usize - 'a' as usize;
        self.children[place] = Some(new_child);
        new_child
    }

    fn trieinsert(&mut self, word: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let t = Trie::new();
        assert!(t.children.iter().all(|x| x.is_none()));
    }
    #[test]
    fn add_child_test() {
        let mut t = Trie::new();
        let mut new_t = t.add_child('c', false);
        assert!(t.children[2].is_some());
        assert!(new_t.children.iter().all(|x| x.is_none()));
    }
}
