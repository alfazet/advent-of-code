#include "part1.h"

template <typename T> std::vector<T> splitLine(std::string const &line)
{
    std::istringstream buf(line);
    return {std::istream_iterator<T>(buf), std::istream_iterator<T>()};
}

int part1(FILE *inFile)
{
    char *buf = NULL;
    int64_t res = 0;
    std::vector<std::vector<int64_t>> grid;
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
    for (int j = 0; j < operations.size(); j++)
    {
        int64_t subRes;
        switch (operations[j])
        {
        case '+':
            subRes = 0;
            for (int i = 0; i < grid.size(); i++)
                subRes += grid[i][j];
            break;
        case '*':
            subRes = 1;
            for (int i = 0; i < grid.size(); i++)
                subRes *= grid[i][j];
            break;
        }
        res += subRes;
    }
    printf("%ld\n", res);

    return 0;
}
