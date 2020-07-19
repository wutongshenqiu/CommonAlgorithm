from utils import timer

from typing import List
from queue import LifoQueue
from dataclasses import dataclass


def merge_sort(l: List, start_index: int, end_index: int):
    """分而治之"""
    if start_index >= end_index:
        return

    # 分
    mid_index = (start_index + end_index) // 2
    merge_sort(l, start_index, mid_index)
    merge_sort(l, mid_index+1, end_index)

    # 治
    temp_list = []
    i = start_index
    j = mid_index + 1

    while i <= mid_index and j <= end_index:
        if l[i] < l[j]:
            temp_list.append(l[i])
            i += 1
        else:
            temp_list.append(l[j])
            j += 1
    if i <= mid_index:
        temp_list.extend(l[i:mid_index+1])
    elif j <= end_index:
        temp_list.extend(l[j:end_index+1])
    l[start_index:end_index+1] = temp_list


@timer
def merge_sort_by_stack(l: List):
    """归并排序非递归

    递归方法的思想是自顶向下，非递归方法的思想是自底向上
        1. 初始要归并的程度是1
        2. 下一次要归并的小数组的长度是 i*2，i 为上一次归并的小数组长度
        3. 每次总的归并数量都是整个数组的元素个数 n
    """
    @dataclass
    class Index:
        left: int
        right: int

    def merge(index1: Index, index2: Index):
        temp_list = []
        i = index1.left
        j = index2.left

        while i <= index1.right and j <= index2.right:
            if l[i] < l[j]:
                temp_list.append(l[i])
                i += 1
            else:
                temp_list.append(l[j])
                j += 1
        if i <= index1.right:
            temp_list.extend(l[i:index1.right + 1])
        elif j <= index2.right:
            temp_list.extend(l[j:index2.right + 1])

        l[index1.left:index2.right+1] = temp_list

    merge_len = 1
    while merge_len * 2 <= len(l):
        j = 0
        while j <= len(l) - 2 * merge_len:
            index1 = Index(j, j+merge_len-1)
            index2 = Index(j+merge_len, j+2*merge_len-1)
            merge(index1, index2)
            j += 2 * merge_len

        # 注意这里，是一个长的(2个merge_len)和一个短的合并
        if j < len(l):
            merge(
                Index(j-2*merge_len, j-1),
                Index(j, len(l)-1)
            )

        merge_len *= 2


if __name__ == '__main__':
    l = list(range(50000, 0, -1))
    merge_sort_by_stack(l)

    # print(l)
    assert l == list(range(1, 50001))