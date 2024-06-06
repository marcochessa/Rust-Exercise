use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::{Rc, Weak};

pub enum NodeFunction {
    Generator(bool),
    Switch(bool),
    Light,
}

type NodeLink = Rc<RefCell<Node>>;
type ChildLink = Option<NodeLink>;
type ParentLink = Option<Weak<RefCell<Node>>>;

#[derive(Debug)]
pub enum MyError {
    OperationNotSupported,
    NodeNotFound,
}

pub struct Node {
    name: String,
    function: NodeFunction,
    parent: ParentLink,
    outs: [ChildLink; 2],
}

impl Node {
    pub fn new(name: &str, function: NodeFunction) -> Self {
        Node {
            name: name.to_string(),
            function: function,
            parent: None,
            outs: [None, None],
        }
    }

    // turn on or off the switch or the generator, if it's a light return an error
    pub fn toggle(&mut self) -> Result<(), MyError> {
        match self.function {
            NodeFunction::Light => Err(MyError::OperationNotSupported),
            NodeFunction::Switch(b) => {
                self.function = NodeFunction::Switch(!b);
                Ok(())
            }
            NodeFunction::Generator(b) => {
                self.function = NodeFunction::Generator(!b);
                Ok(())
            }
        }
    }

    pub fn is_toggle_on(&self) -> bool {
        match self.function {
            NodeFunction::Light => true, // lights are always switched on
            NodeFunction::Switch(b) => b,
            NodeFunction::Generator(b) => b,
        }
    }
}

pub struct CircuitTree {
    // choose the right type for root and names
    root: Option<NodeLink>,
    names: HashMap<String, NodeLink>,
}

impl CircuitTree {
    pub fn new() -> Self {
        CircuitTree {
            root: None,
            names: HashMap::new(),
        }
    }

    pub fn from_file(fname: &str) -> Self {
        let mut ct = CircuitTree::new();
        let f = File::open(fname).unwrap();
        let reader = BufReader::new(f);
        for l in reader.lines() {
            let l = l.unwrap();
            let toks = l.split_whitespace().collect::<Vec<&str>>();
            println!("Line: {:?}", toks);
            match toks[0] {
                "G" => {
                    let node = Node::new(toks[1], NodeFunction::Generator(toks[3] == "on"));
                    ct.add(toks[2], node).unwrap();
                }
                "S" => {
                    let node = Node::new(toks[1], NodeFunction::Switch(toks[3] == "on"));
                    ct.add(toks[2], node).unwrap();
                }
                "L" => {
                    let node = Node::new(toks[1], NodeFunction::Light);
                    ct.add(toks[2], node).unwrap();
                }
                _ => {
                    panic!("Invalid input");
                }
            }
        }
        ct
    }

    // get a node by name
    pub fn get(&self, name: &str) -> Option<NodeLink> {
        self.names.get(name).cloned()
    }

    // add a new node
    pub fn add(&mut self, parent_name: &str, node: Node) -> Result<(), MyError> {
        let parent = self.get(parent_name);

        match parent {
            Some(p) => {
                let name = node.name.clone();
                let node = Rc::new(RefCell::new(node));
                self.names.insert(name, node.clone());
                let mut p_ref = p.borrow_mut();
                // it's granted that the parent has at most two children
                if p_ref.outs[0].is_none() {
                    p_ref.outs[0] = Some(node.clone());
                } else {
                    p_ref.outs[1] = Some(node.clone());
                }
                node.borrow_mut().parent = Some(Rc::downgrade(&p));
                Ok(())
            }
            None => {
                if parent_name == "-" {
                    let name = node.name.clone();
                    let node = Rc::new(RefCell::new(node));
                    self.names.insert(name, node.clone());
                    self.root = Some(node.clone());
                    Ok(())
                } else {
                    return Err(MyError::NodeNotFound);
                }
            }
        }
    }

    // is the light on? Error if it's not a light
    pub fn light_status(&self, name: &str) -> Result<bool, MyError> {
        let node = self.get(name);
        match node {
            Some(n) => {
                let n_ref = n.borrow();
                match n_ref.function {
                    NodeFunction::Light => {
                        // it is a light: navigate back the tree to check if it's on
                        // return as soon as a switch is off
                        let mut cnode = n_ref.parent.clone().map(|wn| wn.upgrade().unwrap());

                        loop {
                            match cnode {
                                Some(p) => {
                                    let p_ref = (*p).borrow();

                                    if !p_ref.is_toggle_on() {
                                        return Ok(false);
                                    }

                                    cnode = p_ref.parent.clone().map(|wn| wn.upgrade().unwrap());
                                }
                                None => {
                                    return Ok(true);
                                }
                            }
                        }
                    }
                    _ => Err(MyError::OperationNotSupported),
                }
            }
            None => Err(MyError::NodeNotFound),
        }
    }

    pub fn turn_light_on(&self, name: &str) -> Result<(), MyError> {
        let node = self.get(name);

        match node {
            Some(n) => {
                let n_ref = n.borrow();
                match n_ref.function {
                    NodeFunction::Light => {
                        // it is a light: navigate back the tree and turn anything on
                        let mut cnode = n_ref.parent.clone().map(|wn| wn.upgrade().unwrap());
                        loop {
                            match cnode {
                                Some(p) => {
                                    let mut p_ref = (*p).borrow_mut();

                                    if !p_ref.is_toggle_on() {
                                        match p_ref.function {
                                            NodeFunction::Switch(_) => {
                                                p_ref.toggle()?;
                                            }
                                            NodeFunction::Generator(_) => {
                                                p_ref.toggle()?;
                                            }
                                            _ => { }
                                        }
                                    }

                                    cnode = p_ref.parent.clone().map(|wn| wn.upgrade().unwrap());
                                }
                                None => {
                                    return Ok(());
                                }
                            }
                        }
                    }
                    _ => Err(MyError::OperationNotSupported),
                }
            }
            None => Err(MyError::NodeNotFound),
        }
    }
}
