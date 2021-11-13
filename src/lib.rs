use std::cell::RefCell;
//use std::mem;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct trinode {
    children: [Option<Box<RefCell<trinode>>>; 256],
    terminal: bool,
}

impl trinode {
    pub fn new() -> Self {
        let children: [Option<Box<RefCell<trinode>>>; 256] = {
            let mut res: Vec<Option<Box<RefCell<trinode>>>> = vec![];
            for _i in 0..256 {
                res.push(None);
            }
            res.try_into().unwrap()
        };
        trinode {
            children,
            terminal: false,
        }
    }

    pub fn triinsert(&mut self, word: &str) {
        let mut tmp = self;

        for ch in word.chars() {
            if tmp.children[ch as usize].is_none() {
                tmp.children[ch as usize].replace(Box::new(RefCell::new(trinode::new())));
            }
            tmp = tmp.children[ch as usize].as_deref_mut().unwrap().get_mut();
        }

        if tmp.terminal {
        } else {
            tmp.terminal = true;
        }
    }

    fn print_words_rec(&self, prefix: &mut String) {

        if self.terminal {
            println!("Word : {}", prefix);
            //prefix.clear();
        }
        
        if self.children.iter().all(|x| x.is_none()) {
            prefix.clear();
        }

        for (i, c) in self.children.iter().enumerate() {
            if c.is_some() {
                prefix.push(char::from(i as u8));
                c.as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .print_words_rec(prefix);
            }
        }
    }

    pub fn print_words(&self) {
        self.print_words_rec(&mut "".to_string());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_new() {
        let t = trinode::new();
        assert!(t.children.iter().all(|x| x.is_none()));
    }
    #[test]
    fn add_child_test() {
        let mut t = trinode::new();
        //let mut new_t = t.add_child('c', false);
        t.triinsert("word");
        //assert!(t.children[2].is_some());
        //assert!(new_t.children.iter().all(|x| x.is_none()));
        t.print_words();
        dbg!(&t);
    }
}
