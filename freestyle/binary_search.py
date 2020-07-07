# -*- coding: utf-8 -*-
"""二分查找

二分查找主要需要注意的有两方面
1. 查找上界和下界，应该为 [low, high) 的形式，这样可以分成 [low, mid) 和 [mid+1, high)
2. 没有查找到时如何返回
    在 python 中可以通过元组的方式，返回 exists 和插入位置
    在 golang 中其实也可以返回元组，但 sort 库里返回插入位置然后交给使用者判断
    在 java 中通过隐晦的负数插入位置提示需要查找的值不存在，详细请搜 binarySearch
"""


def bsearch(arr: list, val) -> (bool, int):
    """二分查找

    Args:
        arr: 排序好的数组
        val: 需要查找的值

    Returns:
        exists: 是否找到
        idx:    如果找到，值所在的位置，可能是任意一个匹配的位置
                如果没找到，比 val 大的最小元素位置，如果没有则是 len(arr)
    """

    idx = _bsearch(arr, val, 0, len(arr))
    exists = True

    if idx == len(arr) or arr[idx] != val:
        exists = False

    return exists, idx


def _bsearch(arr: list, val, low: int, high: int) -> int:
    if low == high:
        return low

    mid = (low + high) // 2

    if val <= arr[mid]:
        return _bsearch(arr, val, low, mid)
    else:
        return _bsearch(arr, val, mid+1, high)
