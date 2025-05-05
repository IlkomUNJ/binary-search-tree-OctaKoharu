use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::structure::tree::{Node, NodeLink};


pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

pub struct BST {
    pub root: Option<NodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn tree_insert(&mut self, value: i32) {
        let new_node = Node::new_nodelink(value);
        let mut y: Option<NodeLink> = None;
        let mut x = self.root.clone();

        while let Some(node) = x.clone() {
            y = x.clone();
            if value < node.borrow().value {
                x = node.borrow().left.clone();
            } else {
                x = node.borrow().right.clone();
            }
        }

        if let Some(y_node) = y.clone() {
            new_node.borrow_mut().parent = Some(Rc::downgrade(&y_node));
            if value < y_node.borrow().value {
                y_node.borrow_mut().left = Some(new_node);
            } else {
                y_node.borrow_mut().right = Some(new_node);
            }
        } else {
            self.root = Some(new_node);
        }
    }

    fn transplant(&mut self, u: &NodeLink, v: Option<NodeLink>) {
        let u_parent = Node::upgrade_weak_to_strong(u.borrow().parent.clone());
        if u_parent.is_none() {
            self.root = v.clone();
        } else if u_parent.as_ref().unwrap().borrow().left.as_ref().map(|left| Rc::ptr_eq(&u, left)).unwrap_or(false) {
            u_parent.as_ref().unwrap().borrow_mut().left = v.clone();
        } else {
            u_parent.as_ref().unwrap().borrow_mut().right = v.clone();
        }

        if let Some(v_node) = v {
            v_node.borrow_mut().parent = u.borrow().parent.clone();
        }
    }

    pub fn tree_delete(&mut self, value: i32) {
        let node_to_delete = self
            .root
            .clone()
            .and_then(|n| n.borrow().get_node_by_value(value));

        if let Some(z) = node_to_delete {
            let z_left = z.borrow().left.clone();
            let z_right = z.borrow().right.clone();

            if z_left.is_none() {
                self.transplant(&z, z_right.clone());
            } else if z_right.is_none() {
                self.transplant(&z, z_left.clone());
            } else {
                let mut y = z_right.clone().unwrap();
                loop {
                    let left = {
                        let y_borrow = y.borrow();
                        y_borrow.left.clone()
                    };
                
                    match left {
                        Some(left_node) => y = left_node,
                        None => break,
                    }
                }                

                if !Node::upgrade_weak_to_strong(y.borrow().parent.clone())
    .as_ref()
    .map(|parent| Rc::ptr_eq(parent, &z))
    .unwrap_or(false)
 {
                    self.transplant(&y, y.borrow().right.clone());
                    y.borrow_mut().right = z.borrow().right.clone();
                    if let Some(r) = y.borrow().right.clone() {
                        r.borrow_mut().parent = Some(Rc::downgrade(&y));
                    }
                }

                self.transplant(&z, Some(y.clone()));
                y.borrow_mut().left = z.borrow().left.clone();
                let l = {
                    let y_borrow = y.borrow();
                    y_borrow.left.clone()
                };
                if let Some(left_node) = l {
                    left_node.borrow_mut().parent = Some(Rc::downgrade(&y));
                }                
            }
        }
    }
}
