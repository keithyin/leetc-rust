use core::slice::Iter;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>)
}
struct NestedIterator<'a> {
    value: &'a Vec<NestedInteger>,

    iter_queue: Vec<Iter<'a, NestedInteger>>

}

impl<'a> NestedIterator<'a> {

    fn new(nestedList: &'a Vec<NestedInteger>) -> Self {
        let mut nested_iterator = NestedIterator{value: nestedList, iter_queue: vec![]};
        nested_iterator.iter_queue.push(nested_iterator.value.iter());
        nested_iterator
    }

    fn next(&mut self) -> Option<i32> {
        loop {
            if let Some(last_iter) = self.iter_queue.last_mut() {
                match last_iter.next() {
                    Some(item) => {
                        match &item {
                            NestedInteger::Int(v) => {return Some(*v);},
                            NestedInteger::List(v) => {
                                self.iter_queue.push(v.iter());
                            }
                        }
                    }
                    None => {self.iter_queue.pop();}
                }
            } else {
                break;
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::lc::algo_341::{NestedInteger, NestedIterator};

    #[test]
    fn test_iterator() {
        let value = vec![
            NestedInteger::Int(10),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(2)]),
            NestedInteger::Int(11)];

        let mut iter = NestedIterator::new(&value);
        while let Some(v) = iter.next() {
            println!("{}", v);
        }
    }
}