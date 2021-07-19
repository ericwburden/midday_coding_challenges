// In Ancient Greece, it was common to write text with the first line going left to
// right, the second line going right to left, and continuing to go back and forth.
// This style was called "boustrophedon".
// 
// Given a binary tree, write an algorithm to print the nodes in boustrophedon order.
// 
// For example, given the following tree:
// 
//        1
//     /     \
//   2         3
//  / \       / \
// 4   5     6   7
// 
// You should return [1, 3, 2, 4, 5, 6, 7].

#[derive(Debug)]
pub struct Node {
    pub value: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn boustrophedon(head: &Node) -> Vec<char> {
    let mut ltr = true; // Start out going left to right
    
    let mut out = Vec::new();
    let mut to_out = vec![head];
    while !to_out.is_empty() {
        let mut next_up = Vec::new();
        if !ltr { to_out.reverse(); }
        for node in to_out.iter() {
            out.push(node.value);

            if ltr {
                if node.left.is_some() { next_up.push(node.left.as_ref().unwrap().as_ref()); }
                if node.right.is_some() { next_up.push(node.right.as_ref().unwrap().as_ref()); }
            } else {
                if node.right.is_some() { next_up.push(node.right.as_ref().unwrap().as_ref()); }
                if node.left.is_some() { next_up.push(node.left.as_ref().unwrap().as_ref()); }
            }
        }
        if !ltr { next_up.reverse(); }
        ltr = !ltr; // Switch each time
        to_out = next_up;
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // `tree` has the structure:
        //       a
        //      / \
        //     b   c
        //   / |   | \
        //  d  e   f  g
        let tree = Node {
            value: 'a',
            left: Some(Box::new(Node {
                value: 'b',
                left: Some(Box::new(Node { value: 'd', left: None, right: None })),
                right: Some(Box::new(Node { value: 'e', left: None, right: None }))
            })),
            right: Some(Box::new(Node {
                value: 'c',
                left: Some(Box::new(Node { value: 'f', left: None, right: None })),
                right: Some(Box::new(Node { value: 'g', left: None, right: None }))
            }))
        };
        assert_eq!(boustrophedon(&tree), vec!['a', 'c', 'b', 'd', 'e', 'f', 'g']);
    }

    #[test]
    fn bigger_one() {
        //         _____ p _____
        //        /             \
        //     _ o _           _ r _
        //    /     \         /     \
        //   g       n       o       s
        //  / \     / \     / \     / \
        // n   o   i   t   a   c   i   t
        let tree = Node {
            value: 'p',
            left: Some(Box::new(Node {
                value: 'o',
                left: Some(Box::new(Node {
                    value: 'g',
                    left:  Some(Box::new(Node { value: 'n', left: None, right: None })),
                    right: Some(Box::new(Node { value: 'o', left: None, right: None }))
                })),
                right: Some(Box::new(Node {
                    value: 'n',
                    left:  Some(Box::new(Node { value: 'i', left: None, right: None })),
                    right: Some(Box::new(Node { value: 't', left: None, right: None }))
                }))
            })),
            right: Some(Box::new(Node {
                value: 'r',
                left: Some(Box::new(Node {
                    value: 'o',
                    left:  Some(Box::new(Node { value: 'a', left: None, right: None })),
                    right: Some(Box::new(Node { value: 'c', left: None, right: None }))
                })),
                right: Some(Box::new(Node {
                    value: 's',
                    left:  Some(Box::new(Node { value: 'i', left: None, right: None })),
                    right: Some(Box::new(Node { value: 't', left: None, right: None }))
                }))
            })),
        };
        assert_eq!(
            boustrophedon(&tree), 
            vec!['p', 'r', 'o', 'g', 'n', 'o', 's', 't', 'i', 'c', 'a', 't', 'i', 'o', 'n']
        );
    }
}
