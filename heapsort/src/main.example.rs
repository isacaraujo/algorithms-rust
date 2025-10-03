mod heapsort;

fn heapify(arr: &mut [usize], n: usize, i: usize) {
    let mut largest = i;

    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);

        heapify(arr, n, largest);
    }
}

fn heapsort(arr: &mut [usize]) {
    let n = arr.len();

    for i in (0..n/2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);

        heapify(arr, i, 0);
    }
}

fn main() {
    let mut arr = [8, 3, 1, 7, 0, 10, 2, 12, 5, 9];

    println!("Original: {:?}", arr);

    heapsort(&mut arr);

    println!("Heap Sort: {:?}", arr);
}
