pub mod List1 {
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
                head: ListLink::Nil
            }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {
            let prev_head = mem::replace(&mut self.head, ListLink::Nil);
            self.head = ListLink::Cons(elem, Box::new(prev_head))
        }

        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, ListLink::Nil) {
                ListLink::Nil => None,
                ListLink::Cons(e, ll) => {
                    self.head = *ll;
                    Some(e)
                }
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                ListLink::Nil => None,
                ListLink::Cons(e, ll) => {
                    Some(e)
                }
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter { node: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut list: List<T> = List::new();
            for _ in 0..n {
                match self.pop() {
                    None => break,
                    Some(e) => {
                        list.push(e);
                    }
                }
            }
            list
        }
    }

    struct ListIter<'a, T> {
        node: &'a ListLink<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.node {
                ListLink::Nil => None,
                ListLink::Cons(e, ll) => {
                    self.node = ll;
                    Some(e)
                }
            }
        }
    }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T
}

pub mod List2 {
    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementation, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, elem: T) {
            self.head = Some(Box::new(
                Node {
                    elem,
                    next: self.head.take(),
                }))
        }

        pub fn pop(&mut self) -> Option<T> {
            match self.head.take() {
                Some(t) => {
                    self.head = t.next;
                    Some(t.elem)
                }
                None => None
            }
        }

        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Some(t) => {
                    Some(&t.elem)
                }
                None => None
            }
        }

        pub fn iter(&self) -> ListIter<T> {
            ListIter {
                node: &self.head
            }
        }

        pub fn take(&mut self, n: usize) -> List<T> {
            let mut list = List::new();
            for _ in 0..n {
                match self.head.take() {
                    Some(t) => {
                        list.push(t.elem);
                        self.head = t.next;
                    }
                    None => break
                }
            }
            list
        }
    }

    pub struct ListIter<'a, T> {
        node: &'a NodeLink<T>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            match self.node {
                Some(t) => {
                    self.node = &t.next;
                    Some(&t.elem)
                }
                None => None
            }
        }
    }
}

pub mod List2doubleLinked {
    use std::cell::{Ref, RefCell};
    use std::rc::{Rc, Weak};

    pub struct Node<T> {
        elem: T,
        prev: NodeBackLink<T>,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Rc<RefCell<Node<T>>>>;
    type NodeBackLink<T> = Option<Weak<RefCell<Node<T>>>>;

    pub struct List<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>,
    }

    // *****
    // double linked list suggestion: use Rc, since we need more than one reference to the same node
    // for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

    // how to access content of Rc<RefCell<T>>:
    // es let a = Rc::new(RefCell::new(5));
    // let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
    // *x = 6; // we can now change the content of the RefCell
    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None, tail: None }
        }

        pub fn push_head(&mut self, elem: T) {
            // Create a clone of the current head of the list.
            let old_head = self.head.clone();

            // Create a new node with the given element
            let new_head = Rc::new(RefCell::new(Node {
                elem,
                next: self.head.clone(),
                prev: None,
            }));

            // If the old head exists, update its previous pointer
            // to point to the new head using a Weak reference.
            old_head.map(|node|
                (*node).borrow_mut().prev = Some(Rc::downgrade(&new_head))
            );

            //Set head to the new head
            self.head = Some(new_head);

            // If the tail of the list is None (indicating an empty list),
            // set it to point to the new head.
            if self.tail.is_none() {
                self.tail = self.head.clone();
            }
        }

        pub fn push_tail(&mut self, elem: T) {
            // Create a clone of the current tail of the list.
            let old_tail = self.tail.clone();

            // Create a new node with the given element and set its next pointer to the current tail
            let new_tail = Rc::new(RefCell::new(Node {
                elem,
                next: self.tail.clone(),
                prev: None,
            }));

            // If the old tail exists, update its previous pointer to point
            // to the new tail using a Weak reference.
            old_tail.map(|node|
                (*node).borrow_mut().prev = Some(Rc::downgrade(&new_tail))
            );

            self.tail = Some(new_tail);

            // If the head of the list is None (indicating an empty list),
            // set it to point to the new tail.
            if self.head.is_none() {
                self.head = self.tail.clone()
            }
        }

        // to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
        // if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
        // it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
        pub fn pop_head(&mut self) -> Option<T> {
            match self.head.take() {
                Some(t) => {
                    // Unwrap the Rc and extract the inner Node.
                    let mut node = Rc::try_unwrap(t).unwrap_or_else(|_|
                        panic!("The last_elem was shared, failed to unwrap"))
                        .into_inner();
                    // Set the prev pointer of the next node to None.
                    node.prev = None;
                    // Update the head pointer to the next node.
                    self.head = node.next.clone();
                    // If the list contains only one element, set tail to None.
                    if self.head.is_none() {
                        self.tail = None;
                    }
                    // Return the value of the removed node.
                    Some(node.elem)
                }
                // Return the value of the removed node.
                None => None
            }
        }

        pub fn pop_tail(&mut self) -> Option<T> {
            match self.tail.take() {
                Some(t) => {
                    // Unwrap the Rc and extract the inner Node.
                    let mut node = Rc::try_unwrap(t).unwrap_or_else(|_|
                        panic!("The last_elem was shared, failed to unwrap"))
                        .into_inner();
                    // Set the prev pointer of the next node to None.
                    node.prev = None;
                    // Update the head pointer to the next node.
                    self.tail = node.next.clone();
                    // If the list contains only one element, set tail to None.
                    if self.tail.is_none() {
                        self.head = None;
                    }
                    // Return the value of the removed node.
                    Some(node.elem)
                }
                // Return the value of the removed node.
                None => None
            }
        }

        pub fn pop_n(&mut self, n: usize) -> Option<T> {
            let mut node = self.head.clone();
            let mut count = 0;
            while count < n {
                match node {
                    // If the list has a tail node, return a reference to its element.
                    Some(current_node) => {
                        node = current_node.borrow().next.clone();
                    }
                    None => break
                }
            }
            if count == n {
                match node.take() {
                    Some(t) => {
                        // Unwrap the Rc and extract the inner Node.
                        let mut el = Rc::try_unwrap(t).unwrap_or_else(|_|
                            panic!("The last_elem was shared, failed to unwrap"))
                            .into_inner();
                        // Set the prev pointer of the next node to None.
                        el.prev = None;
                        // Update the head pointer to the next node.
                        self.tail = el.next.clone();
                        // If the list contains only one element, set tail to None.
                        if self.tail.is_none() {
                            self.head = None;
                        }
                        // Return the value of the removed node.
                        Some(el.elem)
                    }
                    // Return the value of the removed node.
                    None => None
                }
            } else {
                None
            }
        }
        pub fn peek_head(&self) -> Option<Ref<T>> {
            match &self.head {
                // If the list has a head node, return a reference to its element.
                Some(node) => {
                    let r = Ref::map(node.borrow(), |mi| &mi.elem);
                    Some(r)
                }
                // If the list is empty, return None.
                None => None
            }
        }

        pub fn peek_tail(&self) -> Option<Ref<T>> {
            match &self.tail {
                // If the list has a head node, return a reference to its element.
                Some(node) => {
                    let r = Ref::map(node.borrow(), |mi| &mi.elem);
                    Some(r)
                }
                // If the list is empty, return None.
                None => None
            }
        }

        // Create an iterator starting from the head of the list and iterating forward.
        pub fn iter_head(&self) -> ListIter2<T> {
            ListIter2 {
                // Map the Ref of the current node to a reference to the next node's 'next' field.
                node: Ref::map(self.head.as_ref().unwrap().borrow(), |node| &node.next)
            }
        }

        // Create an iterator starting from the tail of the list and iterating backward.
        pub fn iter_tail(&self) -> ListIter2<T> {
            ListIter2 {
                // Map the Ref of the current node to a reference to the next node's 'prev' field.
                node: Ref::map(self.tail.as_ref().unwrap().borrow(), |node| &node.next)
            }
        }


        pub fn take(&mut self, n: usize) -> List<T> {
            let mut list = List::new();
            for _ in 0..n {
                match self.head.take() {
                    Some(t) => {
                        let node = Rc::try_unwrap(t).unwrap_or_else(|_|
                            panic!("The last_elem was shared, failed to unwrap"))
                            .into_inner();
                        self.head = node.next.clone();
                        list.push_head(node.elem);
                    }
                    None => break
                }
            }
            list
        }

    }


    pub struct ListIter2<'a, T> {
        node: Ref<'a, NodeLink<T>>,
    }

    impl<'a, T> Iterator for ListIter2<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            match self.node {
                Some(t) => {
                    self.node = Ref::map(t.borrow(), |n| &n.next);
                    Some(&t.borrow().elem)
                }
                None => None
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

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

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


