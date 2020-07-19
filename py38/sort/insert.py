from utils import timer

from typing import List


@timer
def insert_sort(l: List):
    for i in range(1, len(l)):
        insert_num = l[i]

        j = i-1
        while j >= 0:
            if insert_num < l[j]:
                l[j+1] = l[j]
            else:
                break
            j -= 1
        j += 1

        l[j] = insert_num


if __name__ == "__main__":
    l = [1, 2, 4, 3]
    insert_sort(l)
    assert l == list(range(1, 5))