#[derive(Debug,Copy,Clone,PartialEq, Eq)]
struct Node {
    member: char,
    terminal: bool,
}

impl Node {
    fn new(member: char, terminal: bool) -> Node {
        Node { member, terminal }
    }

    fn is_terminal(self) -> bool {
        self.terminal
    }
}

#[derive(Debug)]
struct Trie {
    root : Node,
    children : [Option<Node>;26],
}

impl Trie {
    fn new(root:Node) -> Trie{
        Trie {
            root,
            children : [None;26]
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let n = Node::new('c', false);
        assert_eq!(
            n,
            Node {
                member: 'c',
                terminal: false
            }
        );
    }
    #[test]
    fn node_term_test() {
        let n1 = Node::new('a', false);
        assert_eq!(n1.is_terminal(),false);
        let n2 = Node::new('s', true);
        assert_eq!(n2.is_terminal(),true);
    }
    #[test]
    fn trie_creation() {
        let n1 = Node::new('a',false);
        let trie = Trie::new(n1);
    }
}
