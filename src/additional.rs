pub fn binary_sort(){
    let mut displayed:Vec<i8>=vec![25,21,22,24,23,27,26];
    for integer in 1..displayed.len(){
        let x:i8;
        let y:i8;

        {
          x= *&displayed[integer-1];
          y= *&displayed[integer];
        }

        if x>y{
            displayed[integer]=x;
            displayed[integer-1]=y;
        }
    }
    println!("Is finished with {:?}",displayed);
}
pub fn insertion_sort(){
    let mut list:Vec<u8>=vec!(25,26,22,24,27,23,21);
    let length=list.len();
    for i in 1..length{
        let mut j=i;
        let mut var={(list[i],list[i-1])};//y and x
        while var.1>var.0{
            list[j-1]=var.0;
            list[j]=var.1;
            {
                j = j - 1;
                if j!=0 {
                var=(list[j],list[j-1]);}
                else { break; } //сделано
            }
        }
    }
    println!("Is finished with {:?}",list);
}
pub fn merge_sort(mut list: Vec<u8>) -> Vec<u8> {
    if list.len() > 1 {
        let mid = list.len() / 2;
        let left = merge_sort(list[..mid].to_vec());
        let right = merge_sort(list[mid..].to_vec());

        let mut result: Vec<u8> = Vec::with_capacity(left.len() + right.len());
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }
        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);
        result
    } else {
        list
    }
}
pub fn shell_sort(mut result: Vec<u8>) -> Vec<u8> {
    let length = result.len();
    let mut distance = length / 2;
    while distance > 0 {
        for i in distance..length {
            let temp = result[i];
            let mut j = i;
            while j >= distance && result[j - distance] < temp {
                result.swap(j, j - distance);
                j -= distance;
            }
            result[j] = temp;
        }
        distance /= 2;
    }
    result
}
pub fn selection_sort(mut result: Vec<u8>) {
    for last_element in (0..=result.len()-1).rev() {
        let mut index = 0;
        for current in 1..last_element+1 {
            if result[current] > result[index] {
                index = current;
            }
        }
        result.swap(index, last_element);
    }
    println!("{:?}", result);
}



