
pub fn bubble_sort(elements: &mut [i32]) {
    let n = elements.len();

    // an empty and singleton list do not require sorting
    if n < 2 {
        return;
    }

    // start at the second to last element and ends at the first element (0th index).
    // we start with second to last element because we compare the current element with the 
    // element to its right within the inner loop. 
    // n is used because range slices are exclusive rather than inclusive for the end value which
    // gives the first value of n-1 in the outer loop but the value of n-2 in the inner loop.
    // range syntax cannot not be done in reverse without either calling rev() or range_step.
    for j in (1..n).rev() {
        // check each position starting from the 0th and ending at the j-1th position to 
        // ensure that the consecutive position's value is in the correct place.
        // at the end of this loop, the jth largest element is placed in its correct position
        for i in 0..j {
            // if adjacent elements out of order, swap them
            if elements[i] > elements[i+1] {
                let swap = elements[i];
                elements[i] = elements[i+1];
                elements[i+1] = swap;
            }
        }

    }
}

#[cfg(test)]
mod tests {
    // import code defined in sorting module
    use super::*;

    #[test]
    fn sort_empty() {
        let mut elements = [];
        bubble_sort(&mut elements);
        assert_eq!(elements, []);
    }

    #[test]
    fn sort_even() {
        let mut elements = [6,2,4,8,1,3];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1,2,3,4,6,8]);
    }

    #[test]
    fn sort_odd() {
        let mut elements = [6,12,5,8,1,3,2];
        bubble_sort(&mut elements);
        assert_eq!(elements, [1,2,3,5,6,8,12]);
    }
}
