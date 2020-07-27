extern crate test;

use rand::thread_rng;
use rand::seq::SliceRandom;

mod bubble;
mod select;
mod insert;
mod shell;
mod quick;
mod merge;
mod heap;

pub use self::bubble::bubble_sort;
pub use self::select::select_sort;
pub use self::insert::insert_sort;
pub use self::shell::shell_sort;
pub use self::quick::quick_sort;
pub use self::merge::merge_sort;
pub use self::heap::heap_sort;


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use test::Bencher;

    const SMALL_BENCH_SIZE: usize = 5000;
    const MEDIUM_BENCH_SIZE: usize = 100000;
    const BIG_BENCH_SIZE: usize = 2000000;

    fn small_consecutive_integers() -> [i32; SMALL_BENCH_SIZE] {
        let mut l = [0; SMALL_BENCH_SIZE];
        for (mut i, v) in l.iter_mut().enumerate() {
            i += 1;
            *v = i as i32;
        }
        let mut rng = thread_rng();
        l.choose(&mut rng);
        l
    }

    fn medium_consecutive_integers() -> [i32; MEDIUM_BENCH_SIZE] {
        let mut l = [0; MEDIUM_BENCH_SIZE];
        for (mut i, v) in l.iter_mut().enumerate() {
            i += 1;
            *v = i as i32;
        }
        let mut rng = thread_rng();
        l.choose(&mut rng);
        l
    }

    #[test]
    fn test_bubble_sort() {
        let mut l = [4, 3, 2, 1];
        bubble_sort(&mut l);
        assert_eq!(l, [1, 2, 3, 4]);
    }
    
    #[bench]
    fn bench_bubble_sort(b: &mut Bencher) {
        let mut l = small_consecutive_integers();

        b.iter(|| bubble_sort(&mut l));
    }

    #[test]
    fn test_select_sort() {
        let mut l = [4, 3, 2, 1];
        select_sort(&mut l);
        assert_eq!(l, [1, 2, 3, 4]);
    }

    #[bench]
    fn bench_select_sort(b: &mut Bencher) {
        let mut l = small_consecutive_integers();

        b.iter(|| select_sort(&mut l));
    }

    #[test]
    fn test_insert_sort() {
        let mut l = [4, 3, 1, 2];
        insert_sort(&mut l);
        assert_eq!(l, [1, 2, 3, 4]);
    }

    #[bench]
    fn bench_insert_sort(b: &mut Bencher) {
        let mut l = small_consecutive_integers();

        b.iter(|| select_sort(&mut l));
    }

    #[test]
    fn test_shell_sort() {
        let mut l1 = [4, 3, 1, 2];
        let mut l2 = [4, 3, 2, 1];

        shell_sort(&mut l1);
        shell_sort(&mut l2);

        assert_eq!(l1, [1, 2, 3, 4]);
        assert_eq!(l2, [1, 2, 3, 4]);
    }

    #[bench]
    fn bench_shell_sort(b: &mut Bencher) {
        let mut l = medium_consecutive_integers();

        b.iter(|| shell_sort(&mut l));
    }

    #[test]
    fn test_quick_sort() {
        let mut l1 = [2, 4, 3, 1, 5];
        let mut l2 = [5, 4, 3, 2, 1];
        
        quick_sort(&mut l1);
        quick_sort(&mut l2);
        
        assert_eq!(l1, [1, 2, 3, 4, 5]);
        assert_eq!(l2, [1, 2, 3, 4, 5]);
    }

    #[bench]
    fn bench_quick_sort(b: &mut Bencher) {
        let mut l = medium_consecutive_integers();

        b.iter(|| shell_sort(&mut l));
    }

    #[test]
    fn test_merge_sort() {
        let mut l1 = [4, 2, 3, 5, 1];
        let mut l2 = [8, 7, 6, 5, 4, 3, 2, 1];

        merge_sort(&mut l1);
        merge_sort(&mut l2);

        assert_eq!(l1, [1, 2, 3, 4, 5]);
        assert_eq!(l2, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[bench]
    fn bench_merge_sort(b: &mut Bencher) {
        let mut l = medium_consecutive_integers();
        
        b.iter(|| merge_sort(&mut l));
    }

    #[test]
    fn test_heap_sort() {
        let l1 = vec![5, 8, 2, 4, 1];
        let l2 = vec![5, 8, 2, 4, 1, 10, 11, 4, 2];
        
        assert_eq!(heap_sort(l1), vec![1, 2, 4, 5, 8]);
        assert_eq!(heap_sort(l2), vec![1, 2, 2, 4, 4, 5, 8, 10, 11]);
    }

    #[bench]
    fn bench_heap_sort(b: &mut Bencher) {
        let mut l: Vec<i32> = (1 as i32..MEDIUM_BENCH_SIZE as i32).collect();
        let mut rng = thread_rng();
        l.shuffle(&mut rng);

        b.iter(move || heap_sort(l.clone()));
    }
}