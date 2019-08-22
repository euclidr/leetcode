class NumArray(object):

    def __init__(self, nums):
        """
        :type nums: List[int]
        """
        cnt = len(nums)
        bitree = [0] * (cnt+1)
        for i, n in enumerate(nums):
            i = i + 1
            bitree[i] += n
            sp = self.tail(i)
            if i+sp <= cnt:
                bitree[i+sp] += bitree[i]
                
        self.cnt = cnt
        self.bitree = bitree
        self.nums = nums[:]
        #print(self.bitree)
        #print(self.nums)
        
    def tail(self, n):
        return n & (-n)

    def update(self, i, val):
        """
        :type i: int
        :type val: int
        :rtype: None
        """
        dif = val - self.nums[i]
        self.nums[i] = val
        idx = i + 1
        while idx <= self.cnt:
            self.bitree[idx] += dif
            idx += self.tail(idx)
        
        #print('update', i, val)
        #print(self.bitree)
        #print(self.nums)

    def sumRange(self, i, j):
        """
        :type i: int
        :type j: int
        :rtype: int
        """
        j = min(j, self.cnt)
        #print('sumHead', j+1, self.sumHead(j+1))
        return self.sumHead(j+1) - self.sumHead(i)
        
    def sumHead(self, i):
        idx, span, total = 0, 1, 0
        while (1 << span) < i:
            span += 1
            
        #print("sh", i, span)
            
        while span >= 0:
            if idx + (1 << span) <= i:
                idx += (1 << span)
                total += self.bitree[idx]
                #print("idx", idx)
            span -= 1
        return total
        


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(i,val)
# param_2 = obj.sumRange(i,j)