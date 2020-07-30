pub fn insert_sort<T: Ord + Copy>(l: &mut [T]) {
    for i in 1..l.len() {
        let insert_num = l[i];
        let mut j = i;

        while j > 0 {
            if insert_num < l[j-1] {
                l[j] = l[j-1];
            }
            else {
                break;
            }
            j -= 1;
        }

        l[j] = insert_num;
    }
}