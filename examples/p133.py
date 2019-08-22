"""
# Definition for a Node.
class Node(object):
    def __init__(self, val, neighbors):
        self.val = val
        self.neighbors = neighbors
"""
class Solution(object):
    def cloneGraph(self, node):
        """
        :type node: Node
        :rtype: Node
        """
        if node is None:
            return None

        stack = [node]
        mapper = {}
        while stack:
            item = stack.pop()
            mapper[item.val] = Node(item.val, item.neighbors)
            for link in item.neighbors:
                if link.val not in mapper:
                    stack.append(link)

        for _, item in mapper.items():
            new_neighbors = []
            for neighbor in item.neighbors:
                new_neighbors.append(mapper[neighbor.val])
            item.neighbors = new_neighbors

        return mapper[node.val]
