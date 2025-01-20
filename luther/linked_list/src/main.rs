use std::collections::LinkedList;

fn main() {
    // In Rust LL are generally avoided as they hv high memory overhead and worse cache locality than vec, vecdec 
    
    // Rust provides Doubly Linked List
    let mut list = LinkedList::new();

    list.push_back(43);
    list.push_back(46);
    list.push_back(44);

    for node in list {
        println!("{}", node);
    }
}