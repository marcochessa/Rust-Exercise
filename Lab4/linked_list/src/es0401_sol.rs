pub mod list1 {

    use std::mem;

    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }

    pub struct List<T> {
        head: ListLink<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                head: ListLink::Nil,
            }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {
            let node = ListLink::Cons(elem, Box::new(mem::replace(&mut self.head, ListLink::Nil)));
            self.head = node;
        }

        pub fn pop(&mut self) -> Option<T> {
            // we takeout the value of head and check if it's a Cons or Nil
            match mem::replace(&mut self.head, ListLink::Nil) {
                ListLink::Cons(i, item) => {
                    // if it's a Cons we update the head with the next element
                    self.head = *item;
                    Some(i)
                }
                ListLink::Nil => None,
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                ListLink::Cons(i, _) => Some(i),
                ListLink::Nil => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        pub fn iter(&self) -> ListIter<T> {
            ListIter::new(self)
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            for _ in 0..n {
                if let Some(i) = self.pop() {
                    new_list.push(i);
                } else {
                    break;
                }
            }
            new_list
        }
    }

    pub struct ListIter<'a, T> {
        next: &'a ListLink<T>,
    }

    impl<'a, T> ListIter<'a, T> {
        pub fn new(list: &'a List<T>) -> Self {
            ListIter { next: &list.head }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                ListLink::Cons(i, item) => {
                    self.next = item;
                    Some(i)
                }
                ListLink::Nil => None,
            }
        }
    }
}

pub mod list2 {

    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, elem: T) {
            let node = Node {
                elem: elem,
                next: self.head.take(),
            };
            self.head = Some(Box::new(node));
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn iter(&self) -> ListIter<T> {
            ListIter::new(self)
        }

        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            for _ in 0..n {
                if let Some(i) = self.pop() {
                    new_list.push(i);
                } else {
                    break;
                }
            }
            new_list
        }
        // take is trivial once you have implemented the other methods
    }

    pub struct ListIter<'a, T> {
        next: &'a NodeLink<T>,
    }

    impl<'a, T> ListIter<'a, T> {
        pub fn new(list: &'a List<T>) -> Self {
            ListIter { next: &list.head }
        }
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                Some(node) => {
                    self.next = &node.next;
                    Some(&node.elem)
                }
                None => None,
            }
        }
    }
}

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

//  type NodeLink = Option<Rc<RefCell<DNode>>>; // we define a type alias for better readibility
// Example
//  type NodeBackLink = ...

// struct DNode {
// v: i32,
// prev: NodeBackLink // here we can't put NodeLink to avoid a cycle reference, what do we use?
// next: NodeLink
// }

// struct DList {
// head: NodeLink,
// tail: NodeLink
// }

pub mod dlist {

    use std::borrow::Borrow;
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;
    use std::rc::Weak;

    type RcNode<T> = Rc<RefCell<DNode<T>>>;

    type NodeLink<T> = Option<RcNode<T>>;
    type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;

    #[derive(Debug)]
    pub struct DNode<T> {
        pub v: T,
        prev: NodeBackLink<T>,
        next: NodeLink<T>,
    }

    pub struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>,
    }

    impl<T> DList<T> {
        pub fn new() -> Self {
            DList {
                head: None,
                tail: None,
            }
        }

        pub fn push_front(&mut self, elem: T) {
            let old_head = self.head.take(); // now sef.head is temporary None
            let new_node = Rc::new(RefCell::new(DNode {
                v: elem,
                prev: None,
                next: old_head.clone(),
            }));

            // now we must update the previous head with the back reference (in case it's not None)
            match old_head {
                Some(node) => {
                    node.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                }
                None => {
                    // if it was None also the tail was None and we must updated it
                    self.tail = Some(new_node.clone());
                }
            }

            self.head = Some(new_node);
        }

        pub fn pop_back(&mut self) -> Option<T> {
            let old_tail = self.tail.take();

            match old_tail {
                Some(node) => {
                    // get the previous node if any
                    let prev = (*node).borrow().prev.clone();
                    match prev {
                        Some(prev_node) => {
                            // update the previous node next field and set to None
                            // before upgrade the weak pointer
                            let prev_node = prev_node.upgrade().unwrap();
                            prev_node.borrow_mut().next = None;
                            self.tail = Some(prev_node);
                        }
                        None => {
                            // if there is no previous node, the list is not empty, update the head too
                            self.head = None;
                        }
                    }
                    let innner = Rc::try_unwrap(node).ok().unwrap().into_inner();
                    Some(innner.v)
                }
                None => None,
            }
        }

        // push back and pop front are trivial once you have implemented the other methods
        // you can implement them yourself

        // peek with Rc presents some issues, we need to change the signature to return a Ref<T> instead of a &T
        // the reason here https://www.reddit.com/r/learnrust/comments/m7jpz5/how_to_return_t_from_rcrefcellt_in_a_function/
        // By invoking borrow RefCell returns a wrapper Ref<T> which is not a reference to T, but a smart pointer for T which allows to get a reference to T
        // this means that when we return a &T from Ref<T> Ref<T> is dropped and the reference is not valid anymore, therefore we must return Ref<T>
        pub fn peek(&self) -> Option<Ref<T>> {
            self.head.as_ref().map(|node| {
                let x = (**node).borrow();
                // now we have a Ref<DNode<T>> which is not yet what we want to return
                // but since this problem is common Rc has a map method that allows to map the content of Ref<Something<T>> to Ref<T>
                // the logic is very similar to the map method of Option
                return Ref::map(x, |x| &x.v);
            })
        }

        // popn is a bit more complex, we must find the node we want to remove and adjust the pointers back and forth
        pub fn popn(&mut self, n: usize) -> Option<T> {
            // find the node we want to remove
            let mut cnode = self.head.clone();
            for _ in 0..n {
                if let Some(n) = cnode {
                    cnode = (*n).borrow().next.clone();
                } else {
                    return None;
                }
            }

            // now adjust pointers before removing
            cnode.and_then(|node| {
                // prev will point to next and viceversa
                let prev = (*node).borrow().prev.clone(); //.map(|n|n.upgrade().unwrap());
                let prev = prev.map(|n| n.upgrade().unwrap());
                let next = (*node).borrow().next.clone();

                // prev or next or both may be None, there are 4 cases to consider
                match (prev, next) {
                    (Some(prev), Some(next)) => {
                        prev.borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                    }
                    (Some(prev), None) => {
                        prev.borrow_mut().next = None;
                        self.tail = Some(prev.clone());
                    }
                    (None, Some(next)) => {
                        next.borrow_mut().prev = None;
                        self.head = Some(next.clone());
                    }
                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    }
                }

                // once removed from the list the rc should have exactly one reference,
                // HOWEVER PAY ATTENTION: this could not be true if there is an open iterator referencing the node
                // therefore we use try_unwrap and if teh node is recerenced we just ruturn None.

                Rc::try_unwrap(node)
                    .ok() // transform into an Option
                    .map(|x| x.into_inner().v) // transform into an Option<DNode<T>>
            })
        }

        // take is trivial once you have implemented the other methods

        pub fn iter(&self) -> DListIter<T> {
            DListIter::new(self)
        }
    }

    // For a non consuming iterator we must return a referece to the node.
    // It would be nice to return a reference to the value, or a Ref<T> but it's too difficult to manage their lifetimes
    // Therefore we implement a simpler solution that returns a clone of the RC node
    // usage: for it in list.iter() { let n = (*it).borrow(); println!("{}", n.v); }

    pub struct DListIter<T> {
        next: NodeLink<T>,
    }

    impl<T> DListIter<T> {
        pub fn new(list: &DList<T>) -> Self {
            DListIter {
                next: list.head.clone(),
            }
        }
    }

    impl<T> Iterator for DListIter<T> {
        // return a RcNode<T>, we can always have a clone of the node
        type Item = RcNode<T>;

        fn next(&mut self) -> Option<Self::Item> {
            let cnode = self.next.clone();
            cnode.as_ref().map(|node| {
                let n = (**node).borrow();
                match &n.next {
                    Some(n) => {
                        self.next = Some(n.clone());
                    }
                    None => {
                        self.next = None;
                    }
                };
            });
            cnode
        }
    }

}
