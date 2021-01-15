
pub mod variance;
pub mod session_iterator;

use session_iterator::ToSessionIterator;

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_session_iteration() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut session_iter = v.iter().session();

        let list1 = (*session_iter.next().unwrap(),*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.new_session();
        let list2 = (*session_iter.next().unwrap(),*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.commit_session();
        session_iter.new_session();
        let list3 = (*session_iter.next().unwrap(),*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.revert_session();
        session_iter.revert_session();
        let list4 = (*session_iter.next().unwrap(),*session_iter.next().unwrap(),*session_iter.next().unwrap());

        if (1,2,3) != list1 || (4,5,6) != list2 || (7,8,9) != list3 || (7,8,9) != list4{
            panic!(format!("\n    something went wrong with iteration\n   values:\n    list1: {:?}\n    list2: {:?}\n    list3: {:?}\n    list4: {:?}\n",
            list1,list2,list3,list4));
        }
    }
    #[test]
    fn test_session_revert() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut session_iter = v.iter().session();

        session_iter.new_session();
        let list1 = (session_iter.next().unwrap(),session_iter.next().unwrap());
        session_iter.new_session();
        let list2 = (session_iter.next().unwrap(),session_iter.next().unwrap());
        session_iter.revert_session();
        let list3 = (session_iter.next().unwrap(),session_iter.next().unwrap());
        session_iter.revert_session();
        let list4 = (session_iter.next().unwrap(),session_iter.next().unwrap());

        if list1 != list4 || list2 != list3 {
            panic!(format!("\n    something went wrong with reverting\n   values:\n    list1: {:?}\n    list2: {:?}\n    list3: {:?}\n    list4: {:?}\n",
            list1,list2,list3,list4));
        }
    }
    #[test]
    fn test_session_commit() {
        let v = vec![1,2,3,4,5,6,7,8,9];
        let mut session_iter = v.iter().session();

        session_iter.new_session();
        let list1 = (*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.commit_session();
        session_iter.new_session();
        let list2 = (*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.revert_session();
        let list3 = (*session_iter.next().unwrap(),*session_iter.next().unwrap());
        session_iter.revert_session();
        let list4 = (*session_iter.next().unwrap(),*session_iter.next().unwrap());

        if list1 != (1,2) || list4 != (5,6) || list2 != list3 {
            panic!(format!("\n    something went wrong with reverting\n   values:\n    list1: {:?}\n    list2: {:?}\n    list3: {:?}\n    list4: {:?}\n",
            list1,list2,list3,list4));
        }
    }
} 