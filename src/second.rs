use std::mem

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct  Node {
    elem: i32,
    next: Link,
}

