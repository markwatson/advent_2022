use std::{cell::RefCell, fmt::Display, rc::Rc};

#[allow(dead_code)]
#[derive(Debug)]
struct TreeNode<T> {
    value: Option<T>,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
    parent: Option<Rc<RefCell<TreeNode<T>>>>,
}

// TODO: need copy?
// multiple traits: <T: Display + Eq>
#[allow(dead_code)]
impl<T: Display + Copy> TreeNode<T> {
    pub fn new() -> TreeNode<T> {
        TreeNode {
            value: None,
            children: vec![],
            parent: None,
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode<T>>>) {
        self.children.push(new_node);
    }

    pub fn print(&self) -> String {
        if let Some(value) = &self.value {
            value.to_string()
        } else {
            return String::from("[")
                + &self
                    .children
                    .iter()
                    .map(|tn| tn.borrow().print())
                    .collect::<Vec<String>>()
                    .join(",")
                + "]";
        }
    }
}

#[allow(dead_code)]
impl TreeNode<u32> {
    pub fn parse_repr(s: String) -> Rc<RefCell<TreeNode<u32>>> {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        let mut current = Rc::clone(&root);
        let chars = s.chars().collect::<Vec<char>>();
        for (_, c) in chars
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx > 0 && *idx + 1 < chars.len())
        {
            if *c == '[' || c.is_numeric() {
                let child = Rc::new(RefCell::new(TreeNode::new()));
                current.borrow_mut().children.push(Rc::clone(&child));
                {
                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                    if c.is_numeric() {
                        mut_child.value = c.to_digit(10);
                    }
                }
                current = child;
            } else if *c == ',' || *c == ']' {
                let current_clone = Rc::clone(&current);
                current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
            } else {
                panic!("Unknown character: {}", c);
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tree_1() {
        let tree = TreeNode::<u32>::parse_repr(String::from("[1,2]"));
        assert_eq!(tree.borrow().children[0].borrow().value.unwrap(), 1);
    }

    #[test]
    fn test_init_tree_2() {
        let tree = TreeNode::<u32>::parse_repr(String::from("[1,2]"));
        assert_eq!(tree.borrow().children[1].borrow().value.unwrap(), 2);
    }

    #[test]
    fn test_init_tree_3() {
        let tree = TreeNode::<u32>::parse_repr(String::from("[0,1,[3,4,5,[7,8]],2]"));
        assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8]],2]");
    }

    #[test]
    fn test_add_child() {
        let tree = TreeNode::<u32>::parse_repr(String::from("[0,1,[3,4,5,[7,8]],2]"));
        let new_node = Rc::new(RefCell::new(TreeNode::new()));
        new_node.borrow_mut().value = Some(9);
        let child = &tree.borrow().children[2];
        child.borrow_mut().add_child(new_node);
        assert_eq!(tree.borrow().print(), "[0,1,[3,4,5,[7,8],9],2]");
    }
}
