"""inspire from https://stackoverflow.com/questions/17009056/how-to-implement-ologn-decrease-key-operation-for-min-heap-based-priority-queu"""

# https://github.com/python/mypy/issues/3661
from __future__ import annotations

from dataclasses import dataclass
from typing import Hashable, Dict, Optional, List


@dataclass
class HeapElement:
    key: int
    # used as key in map
    track_id: Hashable

    def __eq__(self, other: HeapElement) -> bool:
        return self.key == other.key

    def __ne__(self, other: HeapElement) -> bool:
        return not self == other

    def __lt__(self, other: HeapElement) -> bool:
        return self.key < other.key

    def __gt__(self, other: HeapElement) -> bool:
        return not self < other

    def __le__(self, other: HeapElement) -> bool:
        return self.key <= other.key

    def __ge__(self, other: HeapElement) -> bool:
        return not self <= other


class MinBinaryHeap:
    """add hashmap(dict) to support O(log n) decrease_key"""

    def __init__(self):
        self._heap: List[HeapElement] = []
        # a dict that map track_id to element index in heap
        self._map: Dict[Hashable, int] = dict()

    def get_element(self, track_id: int) -> Optional[HeapElement]:
        if element_index := self._map.get(track_id):
            return self._heap[element_index]
        return None

    def push(self, element: HeapElement):
        # if element has already in heap, try decrease_key
        if element.track_id in self._map:
            self.decrease_key(element)
        else:
            index = len(self._heap)
            self._map[element.track_id] = index
            self._heap.append(element)
            self._sift_up(index)

    def decrease_key(self, element: HeapElement):
        if old_index := self._map.get(element.track_id):
            if element < self._heap[old_index]:
                self._heap[old_index] = element
                self._sift_up(old_index)

    def pop(self) -> Optional[HeapElement]:
        if len(self._heap) == 0:
            return None

        pop_element = self._heap[0]
        del self._map[pop_element.track_id]

        if len(self._heap) > 0:
            last_element = self._heap.pop()
            if len(self._heap) > 0:
                self._heap[0] = last_element
                self._map[last_element.track_id] = 0
                self._sift_down(0)

        return pop_element

    def _sift_down(self, index: int):
        while (left_child_index := index * 2 + 1) <= len(self._heap) - 1:
            element = self._heap[index]
            min_child_index = left_child_index
            min_child_element = self._heap[left_child_index]

            # if right child exists
            if (right_child_index := left_child_index + 1) <= len(self._heap) - 1:
                if self._heap[right_child_index] < min_child_element:
                    min_child_index = right_child_index
                    min_child_element = self._heap[right_child_index]

            if element <= min_child_element:
                break
            self._swap(index, min_child_index)
            index = min_child_index

    def _sift_up(self, index: int):
        while (parent_index := (index - 1) // 2) >= 0:
            if self._heap[parent_index] <= self._heap[index]:
                break

            self._swap(index, parent_index)
            index = parent_index

    def _swap(self, index1: int, index2: int):
        self._heap[index1], self._heap[index2] = self._heap[index2], self._heap[index1]
        self._map[self._heap[index1].track_id], self._map[self._heap[index2].track_id] = \
            self._map[self._heap[index2].track_id], self._map[self._heap[index1].track_id]

    def __str__(self) -> str:
        return f"{self._heap[:3]} ..."


if __name__ == '__main__':
    heap = MinBinaryHeap()
    heap.push(HeapElement(3, 1))
    heap.push(HeapElement(7, 2))
    heap.push(HeapElement(12, 3))
    heap.push(HeapElement(14, 4))
    heap.push(HeapElement(5, 5))
    heap.push(HeapElement(20, 6))
    heap.push(HeapElement(9, 6))
    heap.push(HeapElement(10, 6))
    heap.pop()
    heap.pop()
    heap.pop()
    heap.pop()
    heap.pop()
    heap.pop()
    heap.pop()
    heap.pop()
    pass