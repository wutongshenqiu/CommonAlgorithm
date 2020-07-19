from utils import timer

from typing import List


@timer
def select_sort(l: List):
    for i in range(len(l)):
        min_num = l[i]
        min_index = i
        for index, num in enumerate(l[i+1:]):
            if num < min_num:
                min_index = index+i+1
                min_num = num

        l[min_index], l[i] = l[i], l[min_index]


if __name__ == "__main__":
    l = [2, 1, 3, 4]
    select_sort(l)
    assert l == list(range(1, 5))