fn quicksort_lomuto(arr: &mut [i32]) {
    let len = arr.len();

    if len < 1 {
        return;
    }

    quick_sort_range(arr, 0, len - 1);
}

fn quick_sort_range(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = partition_lomuto(arr, low, high);

        if pivot > 0 {
            quick_sort_range(arr, low, pivot - 1);
        }
        quick_sort_range(arr, pivot + 1, high);
    }
}

fn partition_lomuto(arr: &mut [i32], low: usize, high: usize) -> usize {
    let mut i = low;

    for j in low..high {
        if arr[j] <= arr[high] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn quicksort_hoare(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    quick_sort_hoare_range(arr, 0, len - 1);
}

fn quick_sort_hoare_range(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot = partition_hoare(arr, low, high);

        if pivot > 0 {
            quick_sort_hoare_range(arr, low, pivot);
        }

        if pivot + 1 < high {
            quick_sort_hoare_range(arr, pivot + 1, high);
        }
    }
}

fn partition_hoare(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = low + (high - low) / 2;
    arr.swap(pivot, low);

    let mut i = low;
    let mut j = high + 1;

    loop {
        loop {
            i += 1;
            if i > high || arr[i] >= arr[low] {
                break;
            }
        }

        loop {
            j -= 1;
            if arr[j] <= arr[low] {
                break;
            }
        }

        if i >= j {
            arr.swap(low, j);
            return j;
        }

        arr.swap(i, j);
    }
}

fn quicksort_3way(arr: &mut [i32]) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    quicksort_3way_range(arr, 0, len - 1);
}

fn quicksort_3way_range(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let (lt, gt) = partition_3way(arr, low, high);

        if lt > 0 {
            quicksort_3way_range(arr, low, lt -1);
        }

        if gt + 1 <= high {
            quicksort_3way_range(arr, gt + 1, high);
        }
    }
}

fn partition_3way(arr: &mut [i32], low: usize, high: usize) -> (usize, usize) {
    let pivot = arr[low].clone();

    let mut lt = low;
    let mut i = low + 1;
    let mut gt = high;

    while i <= gt {
        if arr[i] < pivot {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot {
            arr.swap(i, gt);
            if gt == 0 {
                break;
            }
            gt -= 1;
        } else {
            i += 1;
        }
    }

    (lt, gt)
}

fn main() {
    let mut arr = [8, 3, 1, 7, 0, 10, 2, 12, 5, 9];
    quicksort_lomuto(&mut arr);

    println!("QuickSort (Lomuto): {:?}", arr);

    let mut arr_hoare = [8, 3, 1, 7, 0, 10, 2, 12, 5, 9];
    quicksort_hoare(&mut arr_hoare);

    println!("QuickSort (Hoare): {:?}", arr_hoare);

    let mut arr_3way = [8, 3, 1, 7, 0, 10, 2, 12, 5, 9];
    quicksort_3way(&mut arr_3way);

    println!("QuickSort (3-way): {:?}", arr_3way);
}
