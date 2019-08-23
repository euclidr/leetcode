class Solution(object):
    def rangeBitwiseAnd2(self, m, n):
        while m < n:
            n = n & (n - 1)
        return n

    def rangeBitwiseAnd(self, m, n):
        """
        :type m: int
        :type n: int
        :rtype: int
        """
        def top1(i):
            rs = 0
            while i > 1:
                rs += 1
                i >>= 1
            return rs

        if not m:
            return 0

        m_top1 = top1(m)
        n_top1 = top1(n)
        if m_top1 != n_top1:
            return 0

        # print(m_top1, n_top1)

        rs, top = 0, m_top1
        while top >= 0:
            tmp = 1 << top
            if (tmp & m) == (tmp & n):
                if tmp & m:
                    rs += tmp
                top -= 1
            else:
                break

        return rs

if __name__ == '__main__':
    s = Solution()
    print(s.rangeBitwiseAnd(5, 7))
    print(s.rangeBitwiseAnd2(12, 13))
    print(s.rangeBitwiseAnd(111, 111))
