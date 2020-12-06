use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use core::iter::StepBy;
use std::ops::RangeInclusive;
use std::io::{self, Write};
use std::fs::File;

const TIMES: usize = 2_000;         //how many times do we test each size in order to minimize statistical error
const MIN:   usize = 20;            //min array size
const MAX:   usize = 2_000;         //max array size
const STEP:  usize = 20;            //step for incrementing array size

fn main() {
    let mut rng = rand::thread_rng();

    test(&mut rng);

    let result = worker_gen_sizes(&mut rng);

    save_slice(&gen_sizes().collect::<Vec<_>>(), "../r/sizes.txt");

    save_slice(&result.selection_totals, "../r/selection_totals.txt");
    save_slice(&result.insertion_totals, "../r/insertion_totals.txt");
    save_slice(&result.bubble_totals, "../r/bubble_totals.txt");
    save_slice(&result.merge_totals, "../r/merge_totals.txt");
    save_slice(&result.heap1_totals, "../r/heap1_totals.txt");
    save_slice(&result.heap2_totals, "../r/heap2_totals.txt");
}

fn test(rng: &mut ThreadRng) {
    let mut vec: Vec<i32> = vec![0; 20];
    let dist = Uniform::new_inclusive(-100, 100);
    for i in vec.iter_mut() {
        *i = dist.sample(rng);
    }

    let vec = vec;
    println!("Array:\n{:?}", vec);

    let mut to_be_heapified1: Vec<i32> = vec.clone();
    let mut to_be_heapified2: Vec<i32> = vec.clone();
    HeapifiedSlice::heapify_1(&mut to_be_heapified1);
    println!("Heapification 1:\n{:?}", to_be_heapified1);
    HeapifiedSlice::heapify_2(&mut to_be_heapified2);
    println!("Heapification 2:\n{:?}", to_be_heapified2);
    fn is_heapified<T: Ord>(slice: &[T], top: usize) -> bool {
        let left = top * 2 + 1;
        let right = top * 2 + 1;
        if left < slice.len() && (slice[left] > slice[top] || !is_heapified(slice, left)) {
            return false;
        }
        if right < slice.len() && (slice[right] > slice[top] || !is_heapified(slice, right)) {
            return false;
        }
        true
    }
    assert!(is_heapified(&to_be_heapified1, 0), "heapify_1 is incorrect!");
    assert!(is_heapified(&to_be_heapified2, 0), "heapify_2 is incorrect!");

    let mut vec0: Vec<i32> = vec.clone();
    vec0.sort();
    println!("Default sorted:\n{:?}", vec0);

    let mut vec1: Vec<i32> = vec.clone();
    selection_sort(&mut vec1);
    println!("Selection sorted:\n{:?}", vec1);

    let mut vec2: Vec<i32> = vec.clone();
    insertion_sort(&mut vec2);
    println!("Insertion sorted:\n{:?}", vec2);

    let mut vec3: Vec<i32> = vec.clone();
    bubble_sort(&mut vec3);
    println!("Bubble sorted:\n{:?}", vec3);

    let mut vec4: Vec<i32> = vec.clone();
    merge_sort(&mut vec4);
    println!("Merge sorted:\n{:?}", vec4);

    let mut vec5: Vec<i32> = vec.clone();
    heap1_sort(&mut vec5);
    println!("Heap v1 sorted:\n{:?}", vec5);

    let mut vec6: Vec<i32> = vec.clone();
    heap2_sort(&mut vec6);
    println!("Heap v2 sorted:\n{:?}", vec6);

    assert!(vec0 == vec1, "Selection sort is incorrect!");
    assert!(vec0 == vec2, "Insertion sort is incorrect!");
    assert!(vec0 == vec3, "Bubble sort is incorrect!");
    assert!(vec0 == vec4, "Merge sort is incorrect!");
    assert!(vec0 == vec5, "Heap v1 sort is incorrect!");
    assert!(vec0 == vec6, "Heap v2 sort is incorrect!");

    println!("Tests passed! Either all implementations are correct or all are flawed (including the one from stdlib :D).");
}

fn benchmark<T: Ord, F>(slice: &mut [T], f: F) -> u128
where F: Fn(&mut [T])
{
    let start = Instant::now();

    f(slice);

    let end = Instant::now();
    let time_taken = end.duration_since(start).as_nanos();
    time_taken
}

fn gen_sizes() -> StepBy<RangeInclusive<usize>> {
    let min = if MIN != 0 { MIN } else { MIN + STEP };

    (min..=MAX).step_by(STEP)
}

fn worker_gen_sizes(rng: &mut ThreadRng) -> Result {
    worker(gen_sizes(), rng)
}

fn worker(sizes_iter: StepBy<RangeInclusive<usize>>, rng: &mut ThreadRng) -> Result {
    let dist = Uniform::new_inclusive(i32::MIN, i32::MAX);

    let mut selection_totals: Vec<u128> = Vec::new();
    let mut insertion_totals: Vec<u128> = Vec::new();
    let mut bubble_totals: Vec<u128> = Vec::new();
    let mut merge_totals: Vec<u128> = Vec::new();
    let mut heap1_totals: Vec<u128> = Vec::new();
    let mut heap2_totals: Vec<u128> = Vec::new();

    for size in sizes_iter {
        let mut selection_total: u128 = 0;
        let mut insertion_total: u128 = 0;
        let mut bubble_total: u128 = 0;
        let mut merge_total: u128 = 0;
        let mut heap1_total: u128 = 0;
        let mut heap2_total: u128 = 0;

        let mut vec: Vec<i32> = vec![0; size];

        for _ in 0..TIMES {

            for i in vec.iter_mut() {
                *i = dist.sample(rng);
            }

            selection_total += benchmark(&mut vec.clone(), selection_sort);
            insertion_total += benchmark(&mut vec.clone(), insertion_sort);
            bubble_total += benchmark(&mut vec.clone(), bubble_sort);
            merge_total += benchmark(&mut vec.clone(), merge_sort);
            heap1_total += benchmark(&mut vec.clone(), heap1_sort);
            heap2_total += benchmark(&mut vec.clone(), heap2_sort);
        }

        selection_total /= TIMES as u128;
        insertion_total /= TIMES as u128;
        bubble_total /= TIMES as u128;
        merge_total /= TIMES as u128;
        heap1_total /= TIMES as u128;
        heap2_total /= TIMES as u128;

        println!("size = {}, selection = {}, insertion = {}, bubble = {}, merge = {}, heap v1 = {}, heap v2 = {}", size, selection_total, insertion_total, bubble_total, merge_total, heap1_total, heap2_total);
        
        selection_totals.push(selection_total);
        insertion_totals.push(insertion_total);
        bubble_totals.push(bubble_total);
        merge_totals.push(merge_total);
        heap1_totals.push(heap1_total);
        heap2_totals.push(heap2_total);
    }

    Result { selection_totals, insertion_totals, bubble_totals, merge_totals, heap1_totals, heap2_totals }
}

fn selection_sort<T: Ord>(slice: &mut [T]) {
    for last_idx in (1..=slice.len()).rev() {
        let mut max_idx: usize = 0;

        for i in 1..last_idx {
            if slice[max_idx] < slice[i] {
                max_idx = i;
            }
        }

        slice.swap(max_idx, last_idx - 1);
    }
}

// MR: I have added two versions:
//
// The first version is about three times faster, but requires the Copy trait.
//
// fn insertion_sort<T: Ord + Copy>(slice: &mut [T]) {
//     for i in 1..slice.len() {
//         let ins_elem_val = slice[i];
//         let mut ins_elem_idx = i;

//         while (ins_elem_idx > 0) && (ins_elem_val < slice[ins_elem_idx - 1]) {
//             slice[ins_elem_idx] = slice[ins_elem_idx - 1];
//             ins_elem_idx -= 1;
//         }

//         slice[ins_elem_idx] = ins_elem_val;
//     }
// }
//
// The second version is slower than the above, but is substantially shorter.
// It might be a bit clearer, but that is a matter of personal taste.

fn insertion_sort<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut ins_elem_idx = i;

        while (ins_elem_idx > 0) && (slice[i] < slice[ins_elem_idx - 1]) {
            ins_elem_idx -= 1;
        }

        slice[ins_elem_idx..=i].rotate_right(1);
    }
}


fn bubble_sort<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut swapped: bool = false;
        for j in 0..(slice.len() - i - 1) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn merge_sort<T: Ord + Copy>(slice: &mut [T]) {

    fn merge<T: Ord + Copy>(left: &[T], right: &[T], buff: &mut Vec<T>) {
        let mut l = 0;
        let mut r = 0;
        while l < left.len() && r < right.len() {
            if right[r] < left[l] {
                buff.push(right[r]);
                r += 1;
            } else {
                buff.push(left[l]);
                l += 1;
            }
        }
        buff.extend_from_slice(&left[l..]);
        buff.extend_from_slice(&right[r..]);
    }

    fn sort<T: Ord + Copy>(slice: &mut [T], buff: &mut Vec<T>) {
        if slice.len() < 2 {
            return;
        } else if slice.len() == 2 {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
        } else {
            let len = slice.len();
            let (left, right) = slice.split_at_mut(len / 2);
            sort(left, buff);
            sort(right, buff);
            merge(left, right, buff);
            slice.copy_from_slice(buff);
            buff.clear();
        }
    }

    let mut buff: Vec<T> = Vec::with_capacity(slice.len());
    sort(slice, &mut buff);
}

struct HeapifiedSlice<'a, T: Ord> {

    slice: &'a mut [T]
}

impl <'a, T: 'a + Ord> HeapifiedSlice<'a, T> {

    //top element has no parent
    fn get_heap_parent(k: usize) -> Option<usize> {
        (k.checked_sub(1)).map(|elem| elem / 2)
    }

    fn fix_heap_bottom_to_top(slice: &mut [T], mut idx: usize) {
        while idx > 0 {
            let parent_idx = Self::get_heap_parent(idx).unwrap(); //safe because idx is > 0
            if slice[parent_idx] < slice[idx] {
                slice.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                return;
            }
        }
    }

    fn fix_heap_top_to_bottom(slice: &mut [T], mut top: usize) {
        loop {
            let left = top * 2 + 1;
            let right = top * 2 + 2;
            if right < slice.len() { //we have two children, and they can have children too
                if slice[left] >= slice[right] && slice[left] > slice[top] { //left is the biggest
                    slice.swap(top, left);
                    top = left;
                } else if slice[right] >= slice[left] && slice[right] > slice[top] { //right is the biggest
                    slice.swap(top, right);
                    top = right;
                } else { //top is the biggest, we are done
                    return;
                }
            } else if left < slice.len() { //we only have left child who does not have children, we are done after fixing it
                if slice[left] > slice[top] {
                    slice.swap(top, left);
                }
                return;
            } else { //we have no children, we are done
                return;
            }
        }
    }

    pub fn heapify_1(slice: &'a mut [T]) -> HeapifiedSlice<'a, T> {
        for idx in 1..slice.len() {
            Self::fix_heap_bottom_to_top(&mut slice[0..=idx], idx);
        }
        HeapifiedSlice { slice }
    }
    
    pub fn heapify_2(slice: &'a mut [T]) -> HeapifiedSlice<'a, T> {
        let last = slice.len() - 1;
        if last > 0 {
            let parent_of_last = Self::get_heap_parent(last).unwrap(); //safe because last is > 0
            for idx in (0..=parent_of_last).rev() {
                Self::fix_heap_top_to_bottom(slice, idx);
            }
        }
        HeapifiedSlice { slice }
    }

    pub fn sort(&mut self) {
        let mut len = self.slice.len();
        while len > 1 {
            self.slice.swap(0, len - 1);
            len -= 1;
            Self::fix_heap_top_to_bottom(&mut self.slice[..len], 0);
        }
    }
}

fn heap1_sort<T: Ord>(slice: &mut [T]) {
    HeapifiedSlice::heapify_1(slice).sort();
}

fn heap2_sort<T: Ord>(slice: &mut [T]) {
    HeapifiedSlice::heapify_2(slice).sort();
}

struct Result {

    selection_totals: Vec<u128>,
    insertion_totals: Vec<u128>,
    bubble_totals: Vec<u128>,
    merge_totals: Vec<u128>,
    heap1_totals: Vec<u128>,
    heap2_totals: Vec<u128>
}

//Save slice to a file and print them to stdout
fn save_slice<T: ToString>(slice: &[T], file_name: &str) {
    let mut string = slice.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    string.push('\n');
    io::stdout().write_all(string.as_bytes()).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(string.as_bytes()).unwrap();
}
