from utils import timer

from typing import List, Optional, Any


class SmallHeap:

    def __init__(self, l: List):
        self._l = l
        self.make_heap()

    def sift_up(self, index: int):
        # 运算的下标比实际的下标大 1
        j = index + 1
        while j >= 2:
            i = j // 2
            if self._l[j-1] < self._l[i-1]:
                self._swap(j-1, i-1)
            else:
                break

            j //= 2

    def sift_down(self, index: int):
        # 运算的下标比实际的下标大1
        j = index + 1
        while j * 2 <= len(self._l):
            i = j * 2
            if i < len(self._l) and self._l[i] < self._l[i-1]:
                i += 1
            if self._l[i-1] < self._l[j-1]:
                self._swap(j-1, i-1)
                j = i
            else:
                break

    def make_heap(self):
        # 对所有的非叶子节点进行 sift_down
        for i in range(len(self._l) // 2, -1, -1):
            self.sift_down(i)

    def pop_top(self) -> Optional[Any]:
        if len(self._l) > 1:
            top_element = self._l[0]
            self._l[0] = self._l.pop()
            self.sift_down(0)
            return top_element

        elif len(self._l) > 0:
            return self._l.pop()

        return None

    def _swap(self, index1, index2):
        self._l[index1], self._l[index2] = self._l[index2], self._l[index1]


@timer
def heap_sort(l: List) -> List:
    small_heap = SmallHeap(l)

    sorted_list = []

    while (top_element := small_heap.pop_top()) is not None:
        sorted_list.append(top_element)

    return sorted_list



if __name__ == '__main__':
    import random
    l = list(range(500000, 0, -1))
    random.shuffle(l)
    assert heap_sort(l) == list(range(1, 500001))