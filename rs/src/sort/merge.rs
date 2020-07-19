#[allow(dead_code)]
pub fn merge_sort<T: Ord + Copy>(l: &mut [T]) {
    let mut merge_len = 1;

    while merge_len * 2 <= l.len() {
        let mut j = 0;
        while j <= l.len() - 2 * merge_len {
            let index1 = Index{left: j, right: j+merge_len-1};
            let index2 = Index{left: j+merge_len, right: j+2*merge_len-1};

            merge(l, index1, index2);
            j += 2 * merge_len; 
        }

        if j < l.len() {
            merge(l, Index{left: j-2*merge_len, right: j-1}, Index{left: j, right: l.len()-1});
        }

        merge_len *= 2;
    }
}

struct Index {
    left: usize,
    right: usize
}

fn merge<T: Ord + Copy>(l: &mut [T], index1: Index, index2: Index) {
    let mut temp_list: Vec<T> = Vec::new();

    let mut i = index1.left;
    let mut j = index2.left;

    while i <= index1.right && j <= index2.right {
        if l[i] < l[j] {
            temp_list.push(l[i]);
            i += 1;
        }
        else {
            temp_list.push(l[j]);
            j += 1;
        }
    }
    
    if i <= index1.right {
        temp_list.extend(l[i..index1.right+1].iter());
    }
    else if j <= index2.right {
        temp_list.extend(l[j..index2.right+1].iter());
    }

    for (index, num) in temp_list.iter().enumerate() {
        l[index1.left+index] = *num;
    }
}