class Solution(object):
    def partitionDisjoint(self, A):
        """
        :type A: List[int]
        :rtype: int
        """
        mins = [0] * len(A)
        m = A[-1]
        for i in xrange(len(A)-1, -1, -1):
            m = min(A[i], m)
            mins[i] = m

        mx = A[0]
        for i in xrange(1, len(A)):
            if mins[i] >= mx:
                return i
            mx = max(A[i], mx)

        raise 'unreachable'

if __name__ == '__main__':
    s = Solution()
    print(s.partitionDisjoint([5,0,3,8,6]))
