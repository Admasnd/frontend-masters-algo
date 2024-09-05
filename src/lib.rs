use std::cmp::Ordering::{Less,Equal,Greater};

// linear_search takes a list and a value and sees if it is in the list.
// Each element in the list is checked one at a time.
// given: [1,2,3] and 3, expect: Some(2)
// given: [] and 3, expect: None
pub fn linear_search<T>(list: &[T], value: T) -> Option<usize> 
    where T: PartialEq<T>
{
    for (i, elem) in list.iter().enumerate() {
        if *elem == value {
            return Some(i)
        }
    }
    None
}

/* binary_search takes a list of sorted elements and a value and sees if it is in the list.
* You start with the full range of elements and continuously halve the search range while 
* checking the middle element until you either find the value or determine that it is not present
* within the list.
* given: [1,2,3] and 3, expect: Some(2)
* given: [] and 3, expect: None
*/
fn binary_search(list: &[i32], value: i32) -> Option<usize> {
    let length = list.len();

    // value not in an empty list
    if length == 0 {
        return None
    }

    let mut left : usize = 0;
    let mut right : usize = length - 1;
    let mut mid : usize;
   
    // TODO explain why this condition
    // TODO consider whether mid + 1 can ever go out of bounds
    // TODO fix bug where mid - 1 can sometimes go below 0
    while left <= right {
        // Below formula is equivalent to (right - left)/2 + left
        mid  = (left + right) / 2; 
        match list[mid].cmp(&value) {
            Equal => return Some(mid),
            Less => left = mid + 1, // look on right half of range
            Greater => right = mid - 1 // look at left half of range
        }
    }
    // could not find value in list
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ls_val_missing_list() {
        let list = [-1,3,5,7];
        let val = 4;
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

    #[test]
    fn ls_str_missing_list() {
        let list = ['a','5','t'];
        let val = 'q';
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

    #[test]
    fn ls_num_empty_list() {
        let list = [];
        let val = 4;
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

     #[test]
     fn ls_num_present_list() {
         let list = [-80,4,2];
         let val = 4;
         let result = linear_search(&list, val);
         assert_eq!(result, Some(1));
     }

     #[test]
     fn bs_num_empty_list() {
         let list = [];
         let val = 4;
         let result = binary_search(&list, val);
         assert_eq!(result, None);
     }

    #[test]
    fn bs_num_missing_odd_list() {
        let list = [10,20,30,40,50,60,70,80,90];
        let values = [5,15,25,35,45,55,65,75,85,95];
        
        for val in values.into_iter() {
            let oracle = linear_search(&list, val);
            let result = binary_search(&list, val);
            assert_eq!(result, oracle);
        }
    }

    #[test]
    fn bs_num_missing_even_list() {
        let list = [10,20,30,40,50,60,70,80,90,100];
        let values = [5,15,25,35,45,55,65,75,85,95,105];
        
        for val in values.into_iter() {
            let oracle = linear_search(&list, val);
            let result = binary_search(&list, val);
            assert_eq!(result, oracle);
        }
    }

    #[test]
    fn bs_num_present_odd_list() {
        let list = [10,20,30,40,50,60,70,80,90];
        for val in list.into_iter() {
            let oracle = linear_search(&list, val);
            let result = binary_search(&list, val);
            assert_eq!(result, oracle);
        }
    }

    #[test]
    fn bs_num_present_even_list() {
        let list = [10,20,30,40,50,60,70,80,90,100];
        for val in list.into_iter() {
            let oracle = linear_search(&list, val);
            let result = binary_search(&list, val);
            assert_eq!(result, oracle);
        }
    }

    #[test]
    fn bs_num_present_singleton_list() {
        let list = [50];
        let val = 50;
        let oracle = linear_search(&list, val);
        let result = binary_search(&list, val);
        assert_eq!(result, oracle);
    }
}
