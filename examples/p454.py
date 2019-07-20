class Solution(object):
    def fourSumCount(self, A, B, C, D):
        """
        :type A: List[int]
        :type B: List[int]
        :type C: List[int]
        :type D: List[int]
        :rtype: int
        """
        from collections import defaultdict
        counter = defaultdict(lambda: 0)
        for i in C:
            for k in D:
                counter[i+k] += 1

        result = 0
        for i in A:
            for k in B:
                n = counter.get(-(i+k))
                if n:
                    result += n

        return result
