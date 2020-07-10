# -*- coding: utf-8 -*-
"""
10*1000*1000 shuffled numbers:
builtin sort elapsed: 6.0986
custom sort elapsed: 61.6460
"""

def quick_sort(arr):
    _quick_sort(arr, 0, len(arr)-1)


def _quick_sort(arr: list, left: int, right: int):
    if left >= right:
        return

    mid = _partition(arr, left, right)
    _quick_sort(arr, left, mid-1)
    _quick_sort(arr, mid+1, right)


def _partition(arr: list, left: int, right: int):
    pivot = arr[right]

    l, r = left, right-1

    while True:
        while arr[l] < pivot:
            l += 1
        while r >= left and arr[r] >= pivot:
            r -= 1

        if l > r:
            break

        arr[l], arr[r] = arr[r], arr[l]

    arr[l], arr[right] = arr[right], arr[l]

    return l


if __name__ == '__main__':
    import random
    from time import time
    origin = [i for i in range(10*1000*1000)]

    shuffled = list(origin)
    random.shuffle(shuffled)

    if origin == shuffled:
        print('not shuffled')

    print('sort started')

    shuffled2 = list(shuffled)
    st = time()
    shuffled2.sort()
    elapse = time() - st
    print(f'builtin sort elapsed: {elapse:.04f}')

    st = time()
    quick_sort(shuffled)
    elapse = time() - st
    if origin == shuffled:
        print('shuffled array now is in order')
        print(f'custom sort elapsed: {elapse:.04f}')
    else:
        print(shuffled)
