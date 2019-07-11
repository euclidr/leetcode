class Solution
{
public:
    int rand10()
    {
        int result = 15;
        while (result > 9)
        {
            result = 0;
            for (int i = 0; i < 4; i++)
            {
                while (true)
                {
                    int r = rand7();
                    if (r == 4)
                    {
                        continue;
                    }
                    int b = r > 4 ? 1 : 0;
                    result = (result << 1) + b;
                    break;
                }
            }
        }
        return result + 1;
    }
};

class Solution2
{
public:
    int rand10()
    {
        int result = 15;
        while (result > 9)
        {
            while(true) {
                int r = rand7();
                if ((r & 0x1) != 1) {
                    continue;
                }
                // cout << "r1" << r << endl;
                result = (r >> 1);
                break;
            }

            while(true) {
                int r = rand7();
                if ((r & 0x1) != 1) {
                    continue;
                }
                // cout << result << endl;
                result = ((result << 2) | (r >> 1));
                break;
            }
        }
        return result + 1;
    }
};