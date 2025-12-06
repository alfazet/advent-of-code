#include "part2.h"

template <typename T> std::vector<T> splitLine(std::string const &line)
{
    std::istringstream buf(line);
    return {std::istream_iterator<T>(buf), std::istream_iterator<T>()};
}

static int intLen(int x)
{
    int res = 0;
    while (x > 0)
    {
        x /= 10;
        res++;
    }

    return res;
}

int part2(FILE *inFile)
{
    char *buf = NULL;
    int64_t res = 0;
    std::vector<std::vector<int64_t>> grid;
    std::vector<std::vector<int>> gridLengths; // lengths of numbers
    for (;;)
    {
        size_t n, len;
        if ((n = getline(&buf, &len, inFile)) == -1)
            break;
        // last line
        if (buf[0] == '+' || buf[0] == '*')
            break;
        grid.emplace_back(splitLine<int64_t>(std::string{buf}));
    }
    std::vector<char> operations = splitLine<char>(std::string{buf});
    for (int i = 0; i < grid.size(); i++)
    {
        gridLengths.emplace_back();
        for (int j = 0; j < grid[0].size(); j++)
        {
            gridLengths.back().push_back(intLen(grid[i][j]));
        }
    }

    for (int j = 0; j < operations.size(); j++)
    {
        int64_t subRes;
        switch (operations[j])
        {
        case '+':
            subRes = 0;
            for (;;)
            {
                int maxLen = 0;
                for (int i = 0; i < grid.size(); i++)
                    maxLen = std::max(maxLen, gridLengths[i][j]);
                if (maxLen == 0)
                    break;

                int64_t number = 0, p10 = 1;
                for (int i = grid.size() - 1; i >= 0; i--)
                {
                    if (gridLengths[i][j] == maxLen)
                    {
                        gridLengths[i][j]--;
                        number += p10 * (grid[i][j] % 10);
                        grid[i][j] /= 10;
                        p10 *= 10;
                    }
                }
                printf("got number %ld out of column %d\n", number, j);
                subRes += number;
            }
            break;
        case '*':
            subRes = 1;
            for (;;)
            {
                int maxLen = 0;
                for (int i = 0; i < grid.size(); i++)
                    maxLen = std::max(maxLen, gridLengths[i][j]);
                if (maxLen == 0)
                    break;

                int64_t number = 0, p10 = 1;
                for (int i = grid.size() - 1; i >= 0; i--)
                {
                    if (gridLengths[i][j] == maxLen)
                    {
                        gridLengths[i][j]--;
                        number += p10 * (grid[i][j] % 10);
                        grid[i][j] /= 10;
                        p10 *= 10;
                    }
                }
                printf("got number %ld out of column %d\n", number, j);
                subRes *= number;
            }
            break;
        }
        printf("subres %d = %ld\n", j, subRes);
        res += subRes;
    }
    printf("%ld\n", res);

    return 0;
}
