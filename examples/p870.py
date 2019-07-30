# Input: A = [2,7,11,15], B = [1,10,4,11]
# Output: [2,11,7,15]
class Solution(object):
    def advantageCount(self, A, B):
        """
        :type A: List[int]
        :type B: List[int]
        :rtype: List[int]
        """
        min_bs = [(i, n) for i, n in enumerate(B)]
        max_bs = [(i, n) for i, n in enumerate(B)]
        marks = [False] * len(B)

        min_bs.sort(key=lambda x: x[1])
        max_bs.sort(key=lambda x: x[1], reverse=True)

        A.sort()
        min_idx, max_idx = 0, 0
        results = [0] * len(B)
        for a in A:
            while min_idx < len(B) and marks[min_bs[min_idx][0]]:
                min_idx += 1
            while max_idx < len(B) and marks[max_bs[max_idx][0]]:
                max_idx += 1

            # maxi = max_bs[max_idx][1]
            maxi_idx = max_bs[max_idx][0]
            mini = min_bs[min_idx][1]
            mini_idx = min_bs[min_idx][0]
            if mini < a:
                results[mini_idx] = a
                marks[mini_idx] = True
            else:
                results[maxi_idx] = a
                marks[maxi_idx] = True

        return results


if __name__ == '__main__':
    s = Solution()
    print(s.advantageCount([12,24,8,32], [13,25,32,11]))
