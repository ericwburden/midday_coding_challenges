use std::cell::RefCell;
use std::fmt;

type ChildNode<T> = Option<RefCell<Box<BTreeNode<T>>>>;

#[derive(Clone, Debug)]
pub struct BTreeNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    val: T,
}

#[derive(Debug)]
pub struct BTree<T> {
    root: ChildNode<T>,
}

impl BTreeNode<i32> {
    pub fn new(val: i32, left: ChildNode<i32>, right: ChildNode<i32>) -> Self {
        BTreeNode::<i32> { left, right, val }
    }

    pub fn new_child(val: i32) -> ChildNode<i32> {
        Some(RefCell::new(Box::new(BTreeNode::<i32>::new(val, None, None))))
    }

    pub fn add_child(&mut self, val: i32) {
        if val < self.val {
            match self.left.as_ref() {
                None => self.left = BTreeNode::<i32>::new_child(val),
                Some(child_node) => child_node.borrow_mut().add_child(val),
            }
        } else {
            match self.right.as_ref() {
                None => self.right = BTreeNode::<i32>::new_child(val),
                Some(child_node) => child_node.borrow_mut().add_child(val),
            }
        }
    }

    pub fn height(&self) -> usize {
        let lheight = match &self.left {
            None => 0,
            Some(node) => node.borrow().height(),
        };
        let rheight = match &self.right {
            None => 0,
            Some(node) => node.borrow().height(),
        };
        if lheight > rheight { lheight + 1 } else { rheight + 1 }
    }

    pub fn invert_children(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);

        if let Some(node) = &self.left {
            node.borrow_mut().invert_children();
        }
        if let Some(node) = &self.right {
            node.borrow_mut().invert_children();
        }
    }

    pub fn pretty_print(&self, prefix: String, left: bool) {
        let prefix_current = if left { "|-" } else { "`-" };
        let suffix = if left { "l" } else { "r" };

        println!("{}{}{} ({})", prefix, prefix_current, self.val, suffix);

        let prefix_child = if left { "| " } else { "  " };
        let prefix = prefix + prefix_child;

        if self.left.is_some() || self.right.is_some() {
            match &self.left {
                None => println!("{}|-* (l)", prefix),
                Some(node) => node.borrow().pretty_print(prefix.clone(), true)
            }
            match &self.right {
                None => println!("{}`-* (r)", prefix),
                Some(node) => node.borrow().pretty_print(prefix, false)
            }
        }
    }
}

impl BTree<i32> {
    pub fn new(root: BTreeNode<i32>) -> Self {
        BTree::<i32> {
            root: Some(RefCell::new(Box::new(root))),
        }
    }

    pub fn add_value(&mut self, val: i32) -> &mut Self {
        match &self.root {
            None => {
                self.root = BTreeNode::<i32>::new_child(val);
            }
            Some(node) => node.borrow_mut().add_child(val),
        };
        self
    }

    pub fn add_sorted_slice(&mut self, vals: &[i32]) -> &mut Self {
        let central_idx = vals.len() / 2;

        self.add_value(vals[central_idx]);
        if central_idx > 0 {
            self.add_sorted_slice(&vals[0..central_idx]);
        }
        if (central_idx + 1) < vals.len() {
            self.add_sorted_slice(&vals[(central_idx + 1)..]);
        }

        self
    }

    pub fn add_unsorted_slice(&mut self, vals: &[i32]) -> &mut Self {
        let mut vals = vals.to_owned();
        vals.sort_unstable();
        self.add_sorted_slice(&vals);
        self
    }

    pub fn from(vals: &[i32]) -> Self {
        let mut btree = BTree::<i32> { root: None };
        btree.add_unsorted_slice(vals);
        btree
    }

    pub fn height(&self) -> usize {
        match &self.root {
            None => 0,
            Some(node) => node.borrow().height(),
        }
    }

    pub fn invert(&mut self) -> &mut Self {
        if let Some(node) = &self.root {
            node.borrow_mut().invert_children();
        }
        self
    }

    pub fn pretty_print(&self) {
        if let Some(node) = &self.root {
            node.borrow().pretty_print(String::from(""), false);
        } else {
            println!("Tree is empty.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_my_work() {
        let mut btree = BTree::<i32>::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

        println!("{:#?}", btree);
        println!("Tree has a height of {}", btree.height());

        btree.pretty_print();
        btree.invert();
        btree.pretty_print();
    }
}
