class Solution(object):
    def primePalindrome(self, N):
        """
        :type N: int
        :rtype: int
        """
        import math

        def gen_primes():
            primes = [2]
            n = 3
            while n < 20000:
                gap = int(math.sqrt(n))
                i, is_prime = 0, True
                while primes[i] <= gap:
                    if n % primes[i] == 0:
                        is_prime = False
                        break
                    i+=1
                if is_prime:
                    primes.append(n)
                n+=1
            return primes

        def is_prime(n, primes):
            gap = int(math.sqrt(n))
            
            i = 0
            while primes[i] <= gap:
                if n % primes[i] == 0:
                    return False
                i+=1
            return True

        def next_palindrome(num_s):
            l = len(num_s)
            half_l = l/2
            if l % 2 == 1:
                half_l += 1
            half = num_s[:half_l]

            should_upgrade = True
            for c in half:
                if c != '9':
                    should_upgrade = False
                    break
            if should_upgrade:
                return '1'+ ('0'*(l-1)) + '1'

            half = str(int(half)+1)
            half_1th = int(half[0])
            if half_1th % 2 == 0:
                half = str(half_1th+1) + ("0" * (half_l - 1))

            if l % 2 == 1:
                return half + half[:half_l-1][::-1]
            else:
                return half + half[::-1]

        def next_palindrome2(num):
            is_palidrome = True
            num_s = str(num)
            i, j = 0, len(num_s)-1
            while i < j:
                if num_s[i] != num_s[j]:
                    is_palidrome = False
                    break
                i+=1
                j-=1
            if is_palidrome:
                return num_s

            l = len(num_s)
            half_l = l/2
            if l % 2 == 1:
                half_l += 1
                half = num_s[:half_l]
                next = half + half[:half_l-1][::-1]
            else:
                half = num_s[:half_l]
                next = half + half[::-1]

            if int(next) >= num:
                return next
            else:
                return next_palindrome(next)

        if N <= 2:
            return 2
        primes = gen_primes()
        # print(primes)
        next = next_palindrome2(N)
        # print('--', next)
        while True:
            n = int(next)
            # print(n)
            if is_prime(n, primes):
                return n
            next = next_palindrome(next)

if __name__ == '__main__':
    s = Solution()
    print(s.primePalindrome(10))
    print(s.primePalindrome(2))
    print(s.primePalindrome(2333222))

# all palindrome that has even count of digits can be divided by 11
# there is a better solution
