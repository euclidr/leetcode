class Solution(object):

    def __init__(self, nums):
        """
        :type nums: List[int]
        """
        from collections import defaultdict
        map = defaultdict(list)
        for i, n in enumerate(nums):
            map[n].append(i)
        self.map = map

    def pick(self, target):
        """
        :type target: int
        :rtype: int
        """
        import random
        indexes = self.map[target]
        if not indexes:
            return None
        return indexes[random.randint(0, len(indexes)-1)]

if __name__ == '__main__':
    s = Solution([1,2,3,3,3])
    print(s.pick(1))
    print(s.pick(3))
    print(s.pick(3))
    print(s.pick(3))

# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.pick(target)