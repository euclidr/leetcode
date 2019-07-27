class Solution(object):
    def countOfAtoms(self, formula):
        """
        :type formula: str
        :rtype: str
        """
        from collections import defaultdict

        def search_digit(idx):
            digit_end = idx
            while digit_end < len(formula) and formula[digit_end].isdigit():
                digit_end += 1
            mul = 1
            if digit_end > idx:
                mul = int(formula[idx: digit_end])
            return mul, digit_end

        def search_lower(idx):
            lower_end = idx
            while lower_end < len(formula) and formula[lower_end].islower():
                lower_end += 1
            return formula[idx:lower_end], lower_end

        stack = [defaultdict(int)]
        idx = 0
        while idx < len(formula):
            if formula[idx] == '(':
                stack.append(defaultdict(int))
                idx += 1
            elif formula[idx] == ')':
                mul, idx = search_digit(idx+1)
                item = stack.pop()
                for k in item.keys():
                    stack[-1][k] += item[k] * mul
            else:
                atom = formula[idx]
                lowers, idx = search_lower(idx+1)
                atom += lowers
                mul, idx = search_digit(idx)
                stack[-1][atom] += mul

        atoms = stack[0].items()
        atoms.sort(key=lambda x: x[0])
        result = ""
        for atom in atoms:
            if atom[1] == 1:
                result += atom[0]
            else:
                result += "{}{}".format(atom[0], atom[1])

        return result

if __name__ == '__main__':
    s = Solution()
    print(s.countOfAtoms("H2O"))
    print(s.countOfAtoms("Mg(OH)2"))
    print(s.countOfAtoms("K4(ON(SO3)2)2"))
