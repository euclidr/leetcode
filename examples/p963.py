class Solution(object):
    def minAreaFreeRect(self, points):
        """
        :type points: List[List[int]]
        :rtype: float
        """
        if len(points) < 4:
            return 0

        result = 0
        for i in range(len(points)-3):
            tmp = self.minRectAreaFrom(i, points)
            if tmp:
                if not result or tmp < result:
                    result = tmp

        return result


    def minRectAreaFrom(self, start, points):
        from collections import defaultdict
        import math

        def gcd(a, b):
            if a == 0:
                return b
            if b == 0:
                return a
            if a < b:
                a, b = b, a
            return gcd(b, a%b)

        def perpetual(edge):
            return (edge[1], -edge[0]), (-edge[1], edge[0])

        def calArea(mid, left, right):
            edge1 = math.sqrt((left[0]-mid[0]) ** 2 + (left[1]-mid[1]) ** 2)
            edge2 = math.sqrt((right[0]-mid[0]) ** 2 + (right[1]-mid[1]) ** 2)
            return edge1 * edge2

        def minRectWith3Point(mid, lefts, rights):
            result = 0
            for l in lefts:
                for r in rights:
                    x = points[l][0] + (points[r][0] - points[mid][0])
                    y = points[l][1] + (points[r][1] - points[mid][1])
                    if [x, y] in points:
                        area = calArea(points[mid], points[l], points[r])
                        # print(area, points[mid], points[l], points[r])
                        if not result or area < result:
                            result = area
            return result

        edges = defaultdict(list)
        for i in range(start+1, len(points)):
            w = points[i][0] - points[start][0]
            h = points[i][1] - points[start][1]
            width = int(w / gcd(abs(w), abs(h)))
            height = int(h / gcd(abs(w), abs(h)))
            edges[(width, height)].append(i)

        # print(edges)
        result = 0

        for edge, pindexes in edges.items():
            perpetual1, perpetual2 = perpetual(edge)

            if edges.get(perpetual1) is not None:
                tmp = minRectWith3Point(start, pindexes, edges[perpetual1])
                if tmp:
                    if not result or tmp < result:
                        result = tmp

            if edges.get(perpetual2) is not None:
                tmp = minRectWith3Point(start, pindexes, edges[perpetual2])
                if tmp:
                    if not result or tmp < result:
                        result = tmp

        return result

if __name__ == '__main__':
    s = Solution()
    print(s.minAreaFreeRect([[7,3],[8,12],[13,5],[6,2],[8,0],[12,8],[14,2],[2,6]]))
    # print(s.minAreaFreeRect([[8,0],[7,3],[14,2],[13,5]]))
