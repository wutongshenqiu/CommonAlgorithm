pub fn bubble_sort<T: Ord>(l: &mut [T]) {
    for i in 0..l.len() {
        for j in 0..l.len()-i-1 {
            if l[j] > l[j+1] {
                l.swap(j, j+1);
            }
        }
    }
}
