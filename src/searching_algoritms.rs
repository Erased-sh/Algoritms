pub fn linear_search(given_data:Vec<u8>,check:&u8){
    let mut found=false;
    let mut index=0;
for data in given_data.iter(){
    if data==check{
        found=true;
        break;
    }
    else {
        index+=1;
    }
}
    println!("{}",found.to_string());
}
pub fn binary_search(given_data: Vec<u8>, check: u8) {
    let mut start: usize = 0;
    let mut finish: usize = given_data.len() - 1;
    let mut found: bool = false;

    while start <= finish {
        let median = (start + finish) / 2;
        let item = given_data[median];
        if item == check {
            found = true;
            break;
        } else {
            if item < check {
                start = median + 1;
            } else {
                finish = median - 1;
            }
        }
    }
    println!("{}",found);
}
pub fn interpolation_sort(mut arr: Vec<u32>){
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            let pos = ((key - arr[0]) * (j - 1) as u32) / (arr[i] - arr[0]);
            arr.swap(j, pos as usize);
            j -= 1;
        }
        arr[j] = key;
    }
   println!("{:?}",arr);
}
