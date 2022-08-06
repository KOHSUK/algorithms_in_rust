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
        f.write_str(&format!(
            "{}",
            self.traverse()
                .iter()
                .map(|l| format!(
                    "{:?}",
                    l.iter()
                        .map(|v| v.map(|x| x.to_string()).unwrap_or(" ".to_string()))
                        .collect::<Vec<String>>()
                ))
                .collect::<Vec<String>>()
                .join("\n")
        ))
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

impl TryFrom<Vec<&str>> for TreeNode {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let mut index = 0;
        let mut temp = VecDeque::new();
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;

        loop {
            if root.is_none() {
                let val = value[index].parse()?;
                index += 1;
                let val = Rc::new(RefCell::new(Self::new(val)));
                root = Some(Rc::clone(&val));
                temp.push_front(Some(val));
                continue;
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
