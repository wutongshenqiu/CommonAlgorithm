pub fn quick_sort<T: Ord + Copy>(l: &mut [T]) {
    let mut stack: Vec<(usize, usize)> = Vec::new();

    stack.push((0, l.len()-1));

    while !stack.is_empty() {
        let (start_index, end_index) = stack.pop().unwrap();

        if start_index < end_index {
            let basis = l[start_index];
            let mut i = start_index;
            let mut j = end_index;
    
            while j > i {
                while j > i && j > start_index {
                    if l[j] >= basis {
                        j -= 1;
                    }
                    else {
                        l[i] = l[j];
                        break;
                    }
                }
                while i < j && i < end_index {
                    if l[i] <= basis {
                        i += 1;
                    }
                    else {
                        l[j] = l[i];
                        break;
                    }
                }
            }
            l[i] = basis;
            
            if i > 0 {
                stack.push((start_index, i-1));
            }
            if i < end_index {
                stack.push((i+1, end_index));
            }
        }
    }
}