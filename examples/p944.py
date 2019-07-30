class Solution(object):
    def minDeletionSize(self, A):
        """
        :type A: List[str]
        :rtype: int
        """
        if len(A) == 0:
            return 0

        marks = [0] * len(A[0])
        prev = A[0]
        for i in range(1, len(A)):
            for j, c in enumerate(A[i]):
                if c < prev[j]:
                    marks[j] = 1
            prev = A[i]

        return sum(marks)