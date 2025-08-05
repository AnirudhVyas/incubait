
pub struct Node<T>{
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}


pub struct LinkedList<T>{
    pub head: Option<Box<Node<T>>>,
    pub tail: Option<Box<Node<T>>>,
}
