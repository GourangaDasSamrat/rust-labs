/// Represents a shoe with size and style information
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

/// Filters a vector of shoes by the specified shoe size
///
/// Uses an iterator with the `filter` method and a closure to find all shoes
/// matching the given size, then collects them into a new vector.
///
/// # Arguments
/// * `shoes` - A vector of Shoe objects to filter
/// * `shoe_size` - The shoe size to filter by
///
/// # Returns
/// A new vector containing only shoes matching the specified size
#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() consumes the vector and creates an iterator
    // filter() with a closure |s| s.size == shoe_size keeps only matching shoes
    // collect() gathers the results into a new vector
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

/// Unit tests for the iterator and filter functionality
#[cfg(test)]
mod tests {
    use super::*;

    /// Test that `shoes_in_size` correctly filters shoes by the specified size
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
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
                },
            ]
        );
    }
}
