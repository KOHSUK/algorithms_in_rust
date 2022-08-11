use core::fmt;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::error::Error;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn traverse_in_level_order(
        arr: &mut Vec<Vec<Option<i32>>>,
        node: Option<&Rc<RefCell<TreeNode>>>,
        level: usize,
    ) {
        if let Some(node) = node {
            let node = Rc::clone(node);
            let node = node.borrow();

            if arr.len() < level + 1 {
                arr.push(Vec::new());
            }
            arr[level].push(Some(node.val));

            Self::traverse_in_level_order(arr, node.left.as_ref(), level + 1);
            Self::traverse_in_level_order(arr, node.right.as_ref(), level + 1);
        } else {
            if arr.len() < level + 1 {
                arr.push(Vec::new());
            }
            arr[level].push(None);
        }
    }

    fn traverse(&self) -> Vec<Vec<Option<i32>>> {
        let mut arr = Vec::new();
        let root = Some(Rc::new(RefCell::new(self.clone())));
        Self::traverse_in_level_order(&mut arr, root.as_ref(), 0);
        arr
    }
}

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .traverse()
                .iter()
                .map(|l| {
                    format!(
                        "{}",
                        l.iter()
                            .map(|v| v
                                .map(|x| x.to_string())
                                .unwrap_or_else(|| "null".to_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[derive(Debug, Clone)]
pub struct TreeNodeError;
impl TreeNodeError {
    pub fn new() -> Self {
        Self
    }
}

impl fmt::Display for TreeNodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.description())
    }
}

impl Error for TreeNodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "Could not construct a TreeNode"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl TryFrom<&str> for TreeNode {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.replace('[', "").replace(']', "");
        let value = value.split(',').map(|x| x.trim()).collect::<Vec<&str>>();
        let mut index = 0;
        let mut temp = VecDeque::new();
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;

        if value.is_empty() {
            return Err(Box::new(TreeNodeError::new()));
        }

        loop {
            if root.is_none() {
                let val = value[index].parse()?;
                index += 1;
                let val = Rc::new(RefCell::new(Self::new(val)));
                root = Some(Rc::clone(&val));
                temp.push_front(Some(val));
                continue;
            }

            if index < value.len() && temp.is_empty() {
                return Err(Box::new(TreeNodeError::new()));
            }

            if index + 1 >= value.len() {
                break;
            }

            if let Some(node) = temp.pop_back() {
                let target = node.ok_or_else(TreeNodeError::new)?;
                let mut target = target.borrow_mut();

                if let Ok(left) = value[index].parse() {
                    let left = Rc::new(RefCell::new(Self::new(left)));
                    target.left = Some(Rc::clone(&left));
                    temp.push_front(Some(left));
                }
                index += 1;

                if let Ok(right) = value[index].parse() {
                    let right = Rc::new(RefCell::new(Self::new(right)));
                    target.right = Some(Rc::clone(&right));
                    temp.push_front(Some(right));
                }
                index += 1;
            }
        }

        let root = root.ok_or_else(TreeNodeError::new)?;
        let root = root.borrow();
        let root = root.clone();
        Ok(root)
    }
}

#[cfg(test)]
mod test {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_valid_array() {
        let root = TreeNode::try_from("[5,4,8,11,null,13,4,7,2,null,null,null,1]").unwrap();
        let mut test = TreeNode::new(5);
        test.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        test.right = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        if let Some(node) = test.left.as_ref() {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(11))));
            if let Some(node) = node.borrow_mut().left.as_ref() {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
            }
        }
        if let Some(node) = test.right.as_ref() {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(13))));
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
            if let Some(node) = node.borrow_mut().right.as_ref() {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
            }
        }
        assert_eq!(root, test);
    }

    #[test]
    fn test_valid_array2() {
        let root = TreeNode::try_from("[3,9,20,null,null,15,7]").unwrap();
        let mut test = TreeNode::new(3);
        test.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        test.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        if let Some(node) = test.right.as_ref() {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        }
        assert_eq!(root, test);
    }

    #[test]
    fn test_valid_array3() {
        let root = TreeNode::try_from("[3, 9, 20, null, null, 15, 7]").unwrap();
        let mut test = TreeNode::new(3);
        test.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        test.right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        if let Some(node) = test.right.as_ref() {
            node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
            node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        }
        assert_eq!(root, test);
    }

    #[test]
    #[should_panic]
    fn test_invalid_array() {
        TreeNode::try_from("[1,2,null,null,null,1]").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_array2() {
        TreeNode::try_from("[null]").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_invalid_array3() {
        TreeNode::try_from("").unwrap();
    }
}
