pub fn shell_sort<T: Ord + Copy>(l: &mut [T]) {
    let mut gap = l.len() / 2;

    while gap > 0 {
        for j in 0..gap {
            let mut t = j + gap;
            
            while t < l.len() {
                let insert_num = l[t];
                let mut i = t;
                while i > j {
                    if insert_num < l[i-gap] {
                        l[i] = l[i-gap];
                    }
                    else {
                        break;
                    }
                    i -= gap;
                }
                l[i] = insert_num;
                t += gap
            }
        }
        gap /= 2;
    }
}