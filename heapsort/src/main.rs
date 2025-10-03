/// It moves the largest value to the root node
/// Args:
///     arr: the original list to be sorted
///     pos: the root node
///     ubound: max upper boundary to be looked up
///
/// # Example
///
/// Given the tree bellow:
///
/// ```
///      a
///     / \
///    b   c
/// ```
///
/// We have the following vector representation:
///
/// ```
/// ['a', 'b', 'c']
/// ```
///
/// In this case, `pos` is 'a' and ubound is 3.
///
fn heapify(arr: &mut [usize], pos: usize, ubound: usize) {
    let left = pos * 2 + 1;
    let right = pos * 2 + 2;

    let mut largest = pos;

    if left < ubound && arr[left] > arr[largest] {
        largest = left;
    }

    if right < ubound && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != pos {
        arr.swap(pos, largest);

        heapify(arr, largest, ubound);
    }
}

fn heapsort(arr: &mut [usize]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    for pos in (0..len/2).rev() {
        heapify(arr, pos, len);
    }

    println!("Intermediate: {:?}", arr);

    for ubound in (1..len).rev() {
        arr.swap(0, ubound);
        heapify(arr, 0, ubound);
    }
}

fn main() {
    let mut arr = [8, 3, 1, 7, 0, 10, 2, 12, 5, 9];

    println!("Original: {:?}", arr);

    heapsort(&mut arr);

    println!("HeapSort: {:?}", arr);
}
