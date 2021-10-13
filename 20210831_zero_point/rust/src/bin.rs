use mylib::LinkedList;

fn main() {
    let values = vec![1, 2, 3, -5, 10, -11, 7, 6, 5, 4, -9, -8, 8, 10, 3, 3, 3, 3, 3, 
                      -9, 10, 12, 14, 16, -30, -22, 8, 4, 3, 2, -9, 9, 5, 7, 2, -12,
                      3, 4, 2, 7, -12, -4, 8, 4, 6, 3, -1, -1, -1, -5, 3, 7, 1, 2, 5];
    println!("Removing zero sum sequences from {:?}", values);
    {
        let mut test_ll = LinkedList::from(values);
        test_ll.remove_zero_sequences();
        println!("{:?}", test_ll.iter().map(|x| x.value).collect::<Vec<_>>());
    }   
    println!("Done!");
}

