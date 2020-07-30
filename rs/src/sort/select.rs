pub fn select_sort<T: Ord>(l: &mut [T]) {
    for i in 0..l.len() {
        let mut min_index = i;
        let mut min_num = &l[i];

        for j in (i+1)..l.len() {
            if &l[j] < min_num {
                min_index = j;
                min_num = &l[j];
            }
        }

        l.swap(min_index, i);
    }
}