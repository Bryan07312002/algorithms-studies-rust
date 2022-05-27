fn main() {
    println!("Algorithms codes \n");
    println!("Insertion sort: {:?} \n", insertion_sort([10,9,8,5,6,7,4,3,2,1]));
}

fn insertion_sort(array:[i32;10])->[i32;10]{
    let mut arr:[i32;10] = array;
    let mut key:i32;
    let mut _j:i32;

    for i in 0..arr.len() {
        key = arr[i].clone();
        _j = i as i32 - 1;

        while _j >= 0 && arr[_j as usize].clone() > key {
            arr[(_j + 1) as usize] = arr[_j as usize].clone();
            _j = _j - 1;
        }

        arr[(_j+1) as usize] = key;
    }

    arr
}

