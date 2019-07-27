#!/bin/pyhon
class Solution(object):
    def coinChange(self, coins, amount):
        """
        :type coins: List[int]
        :type amount: int
        :rtype: int
        """
        if amount == 0:
            return 0
        coins.sort()
        coins2 = []
        for c in coins:
            if c <= amount:
                coins2.append(c)
        coins = coins2
        if not coins:
            return -1

        length = coins[-1]
        marks = [-1] * (length * 2)
        marks[0] = 0

        if amount == length:
            return 1

        INF = amount + 1
        for i in xrange(1, length):
            mini = INF
            for c in coins:
                if marks[i-c] >= 0:
                    mini = (marks[i-c] + 1) if marks[i-c] < mini else mini
            if mini != INF:
                marks[i] = mini

            if i == amount:
                return marks[i]

        marks[length] = 1

        for num in xrange(length+1, amount+1):
            if num % length == 0:
                for i in xrange(0, length):
                    marks[i] = marks[i+length]
                    marks[i+length] = -1
            idx = num % length + length
            mini = INF
            for c in coins:
                if marks[idx-c] >= 0:
                    mini = (marks[idx-c] + 1) if marks[idx-c] < mini else mini
            if mini != INF:
                marks[idx] = mini

        return marks[amount%length+length]


if __name__ == '__main__':
    s = Solution()
    print(s.coinChange([1,2,5], 11))
    print(s.coinChange([2], 3))