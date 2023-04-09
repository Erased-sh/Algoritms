mod additional;
mod searching_algoritms;

fn main() {
    let mut tpe:Vec<u8>=vec![25,21,22,24,23,27,26];
    //additional::binary_search();
    //additional::insertion_sort();
    //println!("{:?}",additional::merge_sort(tpe);
    //println!("{:?}",additional::shell_sort(tpe));
    //additional::selection_sort(tpe);
    searching_algoritms::binary_search(tpe,24);
}



