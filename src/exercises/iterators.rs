#![allow(unused)]

pub fn start() {
    // Iterators are responsible for the logic of iterating over
    // each item and determining when the sequence has finished.

    // Iterators are lazy. This doesn't do anything on its own
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // Each element in iterator is used in one iteration of the
    // loop.
    // For loops implicitly create and consume iterators.
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // Implementing the Iterator trait requires that you define
    // an Item type, and this Item type is used in the return
    // value of the 'next' method.
    // Item = type returned from the iterator.
    // Must define the 'next' method, which returns one item of
    // the iterator at a time wrapped in Some, and, when iteration
    // is over, returns None.

    // .next() behaviour:
    // .iter(): immutable references to values in vector
    // .into_iter(): takes ownership, returns owned values
    // .iter_mut(): mutable references to values in vector
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn iterator_demo() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        // .sum() takes ownership of iterator. consuming adaptor
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        // iterator adaptors change the original iterator
        let v2: Vec<i32> = vec![1, 2, 3];
        // iterator adaptors are lazy. use collect method to consume
        // the iterator and collect the resulting values into a
        // collection data type.
        let v2_inc: Vec<_> = v2.iter().map(|x| x + 1).collect();

        assert_eq!(v2_inc, vec![2, 3, 4]);
    }

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    // filter is used with a closure that captures the shoe_size
    // variable from its environment to iterate over a collection
    // of Shoe struct instances.
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // into_iter() takes ownership of the vector
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        );
    }
}
