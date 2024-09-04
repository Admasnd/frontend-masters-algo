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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn val_missing_list() {
        let list = [-1,3,5,7];
        let val = 4;
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

    #[test]
    fn str_missing_list() {
        let list = ['a','5','t'];
        let val = 'q';
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

    #[test]
    fn empty_list() {
        let list = [];
        let val = 4;
        let result = linear_search(&list, val);
        assert_eq!(result, None);
    }

     #[test]
     fn value_present_list() {
         let list = [-80,4,2];
         let val = 4;
         let result = linear_search(&list, val);
         assert_eq!(result, Some(1));
     }
}
