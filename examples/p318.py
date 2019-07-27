class Solution(object):
    def maxProduct_slow(self, words):
        """
        :type words: List[str]
        :rtype: int
        """
        from collections import defaultdict
        records = defaultdict(set)
        for i, word in enumerate(words):
            for c in set(word):
                records[c].add(i)

        full = set(range(len(words)))
        result = 0
        for i, word in enumerate(words):
            contain_set = set()
            for c in set(word):
                contain_set = contain_set | records[c]
            remain = full - contain_set
            a = len(word)
            for b_idx in remain:
                if b_idx > i:
                    result = max(result, a*len(words[b_idx]))

        return result

    def maxProduct_slow2(self, words):
        """
        :type words: List[str]
        :rtype: int
        """
        words_set = [set(w) for w in words]
        result = 0
        for i in range(len(words)):
            for j in range(i+1, len(words)):
                inter = words_set[i].intersection(words_set[j])
                if not inter:
                    result = max(result, len(words[i])*len(words[j]))
        return result

    def maxProduct(self, words):
        words_set = []
        for word in words:
            num = 0
            for c in word:
                num |= (1 << (ord(c)-ord('a')))
            words_set.append(num)

        result = 0
        for i in range(len(words)):
            for j in range(i+1, len(words)):
                inter = words_set[i] & words_set[j]
                if not inter:
                    result = max(result, len(words[i])*len(words[j]))
        return result



if __name__ == '__main__':
    s = Solution()
    print(s.maxProduct(['a', 'bb', 'ccc', 'abbb']))
    print(s.maxProduct(["abcw","baz","foo","bar","xtfn","abcdef"]))