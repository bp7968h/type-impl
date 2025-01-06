use type_impl::CustomVec;

#[test]
fn initial_len_capacity_check() {
    let vec: CustomVec<usize> = CustomVec::new();
    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);
}
    
#[test]
fn push_element() {
    let mut vec: CustomVec<usize> = CustomVec::new();
    vec.push(2);
    vec.push(1);
    vec.push(3);
        
    assert_eq!(vec.capacity(), 4);
    assert_eq!(vec.len(), 3);
        
    vec.push(4);
    vec.push(5);
        
    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}
    
#[test]
fn get_elem() {
    let mut vec: CustomVec<usize> = CustomVec::new();
    assert_eq!(None, vec.get(1));
        
    vec.push(1);
    vec.push(2);
        
    assert_eq!(Some(&1), vec.get(0));
    assert_eq!(Some(&2), vec.get(1));
}
