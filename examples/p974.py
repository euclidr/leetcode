class Solution(object):
    def subarraysDivByK(self, A, K):
        """
        :type A: List[int]
        :type K: int
        :rtype: int
        """
        records = [0] * K
        pre = 0
        count = 0
        for n in A:
            pre = (pre + n) % K
            count += records[pre]
            records[pre] += 1
            if pre == 0:
                count += 1

        return count
