fn main() {
    let mut arr = [6, 5, 4, 3, 2, 1];
    println!("Before: {:?}", arr);
    quicksort(&mut arr);
    println!("After: {:?}", arr);
}


fn quicksort(arr: &mut [i32]) {
    let l = arr.len();
    quickersort(&mut arr, 0, l);
}

fn quickersort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        quickersort(&mut arr, low, p);
        quickersort(&mut arr, p, high);
    }
}

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mut pivot = high;
    for i in low..high {
        if arr[i] > arr[pivot] {
            let (a, b, c) = (arr[i], arr[pivot], arr[pivot - 1]);
            arr[i] = c;
            arr[pivot] = a;
            arr[pivot - 1] = b;
            pivot -= 1;
        }
    }
    pivot
}
