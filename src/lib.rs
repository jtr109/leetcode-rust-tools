/*!
 * Convert from Breadth First (or called Level Order) Traversal
 */

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl<'a> TreeNode {
    pub fn from_breadth(data: &'a Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut elements = data
            .iter()
            .map(|e| match e {
                None => None,
                Some(val) => Some(Rc::new(RefCell::new(TreeNode::new(*val)))),
            })
            .rev()
            .collect::<Vec<Option<Rc<RefCell<TreeNode>>>>>();
        let root = match elements.pop() {
            None => return None,
            Some(node) => node,
        };
        let mut nodes = VecDeque::new();
        nodes.push_back(root.clone());
        while let Some(e) = nodes.pop_front() {
            match e {
                None => continue,
                Some(parent) => {
                    match elements.pop() {
                        None => break,
                        Some(left) => {
                            parent.as_ref().borrow_mut().left = left;
                            nodes.push_back(parent.borrow().left.clone());
                        }
                    }
                    match elements.pop() {
                        None => break,
                        Some(right) => {
                            parent.as_ref().borrow_mut().right = right;
                            nodes.push_back(parent.borrow().right.clone());
                        }
                    }
                }
            }
        }
        root
    }

    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(TreeNode::from_breadth(&data), expected);
    }

    #[test]
    fn test2() {
        let data = vec![
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            Some(6),
        ];
        let expected = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        assert_eq!(TreeNode::from_breadth(&data), expected);
    }

    #[test]
    fn test_empty() {
        let data = vec![];
        assert_eq!(TreeNode::from_breadth(&data), None);
    }

    #[test]
    fn test_one_element() {
        let data = vec![Some(3)];
        assert_eq!(
            TreeNode::from_breadth(&data),
            Some(Rc::new(RefCell::new(TreeNode::new(3))))
        );
    }
}
