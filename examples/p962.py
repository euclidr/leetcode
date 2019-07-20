class Solution(object):
    def maxWidthRamp(self, A):
        """
        :type A: List[int]
        :rtype: int
        """
        if not A:
            return 0

        lmins = A[:1] * len(A)
        for i in range(1, len(A)):
            lmins[i] = lmins[i-1] if lmins[i-1] < A[i] else A[i]

        rmax = A[-1:] * len(A)
        for i in reversed(range(0, len(A)-1)):
            rmax[i] = rmax[i+1] if rmax[i+1] > A[i] else A[i]

        result = 0
        left, right = 0, 1
        while left < len(A) and right < len(A):
            if lmins[left] <= rmax[right]:
                result = result if result > (right-left) else (right-left)
                right += 1
            else:
                left += 1

        return result
