from typing import List
from queue import LifoQueue
from dataclasses import dataclass

from utils import timer


# 用栈来改写
# @timer
def quick_sort(l: List, start_index: int, end_index: int):
    if end_index <= start_index:
        return

    # 设置基准(可以有不同的设置方法)
    basis = l[start_index]

    i = start_index
    j = end_index
    while j > i:
        while j > start_index and j > i:
            if l[j] >= basis:
                j -= 1
            else:
                l[i] = l[j]
                break

        while i < end_index and i < j:
            if l[i] <= basis:
                i += 1
            else:
                l[j] = l[i]
                break

    l[i] = basis
    quick_sort(l, start_index, i-1)
    quick_sort(l, i+1, end_index)


@timer
def quick_sort_by_stack(l: List):

    @dataclass
    class StackItem:
        start_index: int
        end_index: int

    stack = LifoQueue()

    stack.put(StackItem(0, len(l)-1))

    while not stack.empty():
        stack_item: StackItem = stack.get()
        if stack_item.end_index > stack_item.start_index:
            basis = l[stack_item.start_index]

            i = stack_item.start_index
            j = stack_item.end_index
            while j > i:
                while j > stack_item.start_index and j > i:
                    if l[j] >= basis:
                        j -= 1
                    else:
                        l[i] = l[j]
                        break

                while i < stack_item.end_index and i < j:
                    if l[i] <= basis:
                        i += 1
                    else:
                        l[j] = l[i]
                        break
            l[i] = basis

            stack.put(StackItem(stack_item.start_index, i-1))
            stack.put(StackItem(i+1, stack_item.end_index))


if __name__ == '__main__':
    import random
    l = list(range(500000, 0, -1))
    random.shuffle(l)
    quick_sort_by_stack(l)

    assert l == list(range(1, 500001))