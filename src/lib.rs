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

#[derive(Debug,Clone,Copy)]
struct Trie {
    root : Node,
    children : [Option<Trie>;26],
}

impl Trie {
    fn new(root:Node) -> Trie{
        Trie {
            root,
            children : [None;26]
        }
    }
    fn is_present_helper(&self,trie : Trie,expeted : char,cur_sz: usize,actual_sz : usize) -> bool {

        if trie.root.member == expeted {
            if trie.root.is_terminal() && cur_sz == actual_sz {
                return true;
            } else {
                for child in trie.children.into_iter() {
                    if child.is_none() {
                        continue;
                    } else {
                        let nchild = child.unwrap();
                        return self.is_present_helper(nchild, expeted,cur_sz+1, actual_sz);
                    }
                } 
            } 
        } else {
            return false;
        }
        false
    }
    fn is_present(&self,word:&str) -> bool {
        false
    }

    fn create_child(&self,child_node : Node) -> Trie{
        let place = child_node,member as usize - 'a' as usize;
        let new_child = Trie::new(child_node);
        self.children[place] = Some(new_node);
        new_node
    }

    fn insert_word(self,word : &str) -> Trie {
        let sz = word.len();
        let mut c = word.chars().next();
        let mut check = 0usize;
        //let mut terminal = false;
        let mut trie;
        if c.is_some(){
            check += 1;
            if check == sz {
                //terminal = true;
                self = Trie::new(Node::new(c,true));
                return self;
            }
            self = Trie::new(Node::new(c,false));

            trie = self;

            loop {
                match word.chars().next() {
                    Some(c) => {
                        check += 1;
                        if(check == sz) {
                            trie = trie.create_child(Node::new(c,true));
                            return self;
                        }
                        trie = trie.create_child(Node::new(c,true));
                    }
                    None => break
                }
                
            }
        }
        //todo!();
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
        let n1 = Node::new('a',false)
        let trie = Trie::new(n1);
    }

    fn check_insert_word() {
        
    }
}
