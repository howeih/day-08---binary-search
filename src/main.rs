fn binary_search (data:&mut [i32], item:i32) -> i32{
    let mut res = -1i32;
    let mut left: i32 = 0;
    let mut right: i32 = (data.len() - 1) as i32;
    while left <= right{
        let middle = (left + right) / 2 ;
        if item > data[middle as usize]{
            left = middle + 1; 
        }else if item < data[middle as usize]{
            right = middle - 1;
        }else{
            res = middle as i32;
            break
        }
    }
    res
}

fn main() {
    let mut data = [2, 3, 4, 8, 22, 23, 24, 25, 26, 28, 31, 39, 40, 43, 45, 49, 54, 58, 59, 60, 72, 73, 76, 87, 95, 97, 98];
    data.sort();
    let result = binary_search(&mut data, 3);
    println!("{}", result);
}
