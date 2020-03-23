use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Option<T>,
    tail: Option<Rc<List<T>>>,
}

impl<T> Iterator for List<T> {
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        match &self.head {
            Some(x) => Some(x),
            None => None
        }
    }
    type Item = T;
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push(self: List<T>, element: T) -> List<T> {
        List {
            head: Some(element),
            tail: Some(Rc::new(self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn it_works() {
        let list: List<i32> = List::new();
        assert_eq!(list.head, None);
    }

    #[test]
    fn can_add() {
        let list = List::new().push(2);
        assert_eq!(list.head, Some(2));
    }

    #[test]
    fn check_tail() {
        let list = List::new().push(2).push(3);
        let head = match list.tail {
            Some(x) => x.head,
            None => None,
        };
        assert_eq!(head, Some(2));
    }
}
