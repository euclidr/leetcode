class Solution(object):
    def smallestSufficientTeam(self, req_skills, people):
        """
        :type req_skills: List[str]
        :type people: List[List[str]]
        :rtype: List[int]
        """
        people_encoded = []
        target = 0
        for _ in req_skills:
            target <<= 1
            target += 1

        for i, person in enumerate(people):
            encoded = 0
            for skill in req_skills:
                encoded <<= 1
                if skill in person:
                    encoded += 1
            people_encoded.append((i, encoded, person))
            # print("encoded ", i, " ", encoded)

        people_encoded.sort(key=lambda x: len(x[2]))
        # print("target", target)
        people_encoded2 = []
        for i in range(0, len(people_encoded)):
            contained = False
            for j in range(i+1, len(people_encoded)):
                if people_encoded[i][1] | people_encoded[j][1] == people_encoded[j][1]:
                    contained = True
                    break
            if not contained:
                people_encoded2.append(people_encoded[i])
        people_encoded = people_encoded2
        people_encoded.reverse()

        return self.search(people_encoded, 0, 0, len(req_skills), target)

    def search(self, people, cur_idx, mask, max_depth, target):
        # print("idx %s, mask %s, max_depth %s" % (cur_idx, mask, max_depth))
        if mask == target:
            return []
        if cur_idx == len(people):
            return None
        if not max_depth:
            return None

        # print("value: ",  people[cur_idx])

        if (people[cur_idx][1] | mask) == mask:
            return self.search(people, cur_idx+1, mask, max_depth, target)

        new_mask = people[cur_idx][1] | mask
        new_max_depth = max_depth - 1
        tmp_max_depth = self.onebits(target) - self.onebits(new_mask)
        if tmp_max_depth < new_max_depth:
            new_max_depth = tmp_max_depth

        result1 = self.search(people, cur_idx+1, new_mask,
                              new_max_depth, target)

        # print("result1 ", result1)

        if result1 is not None and len(result1) == 0:
            return [people[cur_idx][0]]

        if result1 is None:
            new_max_depth = max_depth
        else:
            new_max_depth = len(result1) + 1

        result2 = self.search(people, cur_idx+1, mask, new_max_depth, target)
        # print("result2, result1", result2, result1)
        if result2 is None and result1 is None:
            return None
        if result2 is None:
            return [people[cur_idx][0]] + result1
        return result2

    def onebits(self, num):
        res = 0
        while num > 0:
            res += (num % 2)
            num >>= 1
        return res


if __name__ == '__main__':
    s = Solution()
    # print(s.smallestSufficientTeam(
    #         ["java", "nodejs", "reactjs"],
    #         [["java"], ["nodejs"], ["nodejs", "reactjs"]]))

    print(s.smallestSufficientTeam(
            ["hfkbcrslcdjq","jmhobexvmmlyyzk","fjubadocdwaygs","peaqbonzgl","brgjopmm","x","mf","pcfpppaxsxtpixd","ccwfthnjt","xtadkauiqwravo","zezdb","a","rahimgtlopffbwdg","ulqocaijhezwfr","zshbwqdhx","hyxnrujrqykzhizm"],
            [["peaqbonzgl","xtadkauiqwravo"],["peaqbonzgl","pcfpppaxsxtpixd","zshbwqdhx"],["x","a"],["a"],["jmhobexvmmlyyzk","fjubadocdwaygs","xtadkauiqwravo","zshbwqdhx"],["fjubadocdwaygs","x","zshbwqdhx"],["x","xtadkauiqwravo"],["x","hyxnrujrqykzhizm"],["peaqbonzgl","x","pcfpppaxsxtpixd","a"],["peaqbonzgl","pcfpppaxsxtpixd"],["a"],["hyxnrujrqykzhizm"],["jmhobexvmmlyyzk"],["hfkbcrslcdjq","xtadkauiqwravo","a","zshbwqdhx"],["peaqbonzgl","mf","a","rahimgtlopffbwdg","zshbwqdhx"],["xtadkauiqwravo"],["fjubadocdwaygs"],["x","a","ulqocaijhezwfr","zshbwqdhx"],["peaqbonzgl"],["pcfpppaxsxtpixd","ulqocaijhezwfr","hyxnrujrqykzhizm"],["a","ulqocaijhezwfr","hyxnrujrqykzhizm"],["a","rahimgtlopffbwdg"],["zshbwqdhx"],["fjubadocdwaygs","peaqbonzgl","brgjopmm","x"],["hyxnrujrqykzhizm"],["jmhobexvmmlyyzk","a","ulqocaijhezwfr"],["peaqbonzgl","x","a","ulqocaijhezwfr","zshbwqdhx"],["mf","pcfpppaxsxtpixd"],["fjubadocdwaygs","ulqocaijhezwfr"],["fjubadocdwaygs","x","a"],["zezdb","hyxnrujrqykzhizm"],["ccwfthnjt","a"],["fjubadocdwaygs","zezdb","a"],[],["peaqbonzgl","ccwfthnjt","hyxnrujrqykzhizm"],["xtadkauiqwravo","hyxnrujrqykzhizm"],["peaqbonzgl","a"],["x","a","hyxnrujrqykzhizm"],["zshbwqdhx"],[],["fjubadocdwaygs","mf","pcfpppaxsxtpixd","zshbwqdhx"],["pcfpppaxsxtpixd","a","zshbwqdhx"],["peaqbonzgl"],["peaqbonzgl","x","ulqocaijhezwfr"],["ulqocaijhezwfr"],["x"],["fjubadocdwaygs","peaqbonzgl"],["fjubadocdwaygs","xtadkauiqwravo"],["pcfpppaxsxtpixd","zshbwqdhx"],["peaqbonzgl","brgjopmm","pcfpppaxsxtpixd","a"],["fjubadocdwaygs","x","mf","ulqocaijhezwfr"],["jmhobexvmmlyyzk","brgjopmm","rahimgtlopffbwdg","hyxnrujrqykzhizm"],["x","ccwfthnjt","hyxnrujrqykzhizm"],["hyxnrujrqykzhizm"],["peaqbonzgl","x","xtadkauiqwravo","ulqocaijhezwfr","hyxnrujrqykzhizm"],["brgjopmm","ulqocaijhezwfr","zshbwqdhx"],["peaqbonzgl","pcfpppaxsxtpixd"],["fjubadocdwaygs","x","a","zshbwqdhx"],["fjubadocdwaygs","peaqbonzgl","x"],["ccwfthnjt"]]))
