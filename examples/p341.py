# """
# This is the interface that allows for creating nested lists.
# You should not implement it, or speculate about its implementation
# """
#class NestedInteger(object):
#    def isInteger(self):
#        """
#        @return True if this NestedInteger holds a single integer, rather than a nested list.
#        :rtype bool
#        """
#
#    def getInteger(self):
#        """
#        @return the single integer that this NestedInteger holds, if it holds a single integer
#        Return None if this NestedInteger holds a nested list
#        :rtype int
#        """
#
#    def getList(self):
#        """
#        @return the nested list that this NestedInteger holds, if it holds a nested list
#        Return None if this NestedInteger holds a single integer
#        :rtype List[NestedInteger]
#        """

class NestedIterator(object):

    def __init__(self, nestedList):
        """
        Initialize your data structure here.
        :type nestedList: List[NestedInteger]
        """
        self.flattend = []
        self.idx = 0
        stack = [[nestedList, -1]]
        while stack:
            lst, idx = stack[-1]
            idx += 1

            while idx < len(lst):
                if lst[idx].isInteger():
                    self.flattend.append(lst[idx])
                else:
                    sub_lst = lst[idx].getList()
                    stack[-1][1] = idx
                    stack.append([sub_lst, -1])
                    break
                idx += 1

            if idx == len(lst):
                stack = stack[:-1]

    def next(self):
        """
        :rtype: int
        """
        if self.hasNext():
            val = self.flattend[self.idx]
            self.idx += 1
            return val.getInteger()

    def hasNext(self):
        """
        :rtype: bool
        """
        return self.idx < len(self.flattend)

# Your NestedIterator object will be instantiated and called as such:
# i, v = NestedIterator(nestedList), []
# while i.hasNext(): v.append(i.next())