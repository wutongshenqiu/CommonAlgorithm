from utils import timer

from typing import List


@timer
def shell_sort(l: List):
    """希尔排序

        1. 选择一个增量序列
        2. 按照增量序列的元素个数 k，一共进行 k 趟排序
        3. 每一趟排序根据当前增量元素将序列进行划分
        4. 对划分的子表进行插入排序
    """
    if len(l) == 1:
        return

    gap = len(l) // 2
    while gap > 0:
        # 一共有 gap 组
        for j in range(gap):
            # 远距离插入排序
            t = j + gap
            while t < len(l):
                insert_num = l[t]
                i = t - gap
                while i >= j:
                    if insert_num < l[i]:
                        l[i+gap] = l[i]
                    else:
                        break

                    i -= gap
                i += gap
                l[i] = insert_num
                t += gap

        gap //= 2


if __name__ == '__main__':
    l = list(range(1000000, 0, -1))
    shell_sort(l)

    assert l == list(range(1, 1000001))