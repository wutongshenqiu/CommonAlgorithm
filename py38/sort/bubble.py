from typing import List

from utils import timer


@timer
def bubble_sort(l: List) -> None:
    for i in range(len(l)):
        for j in range(0, len(l)-i-1):
            if l[j] > l[j+1]:
                l[j], l[j+1] = l[j+1], l[j]


if __name__ == "__main__":
    l = list(range(5000, 0, -1))
    bubble_sort(l)
    
    assert l == list(range(1, 5001))