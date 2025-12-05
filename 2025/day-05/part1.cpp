#include "part1.h"

struct Range
{
    int64_t l, r;

    Range(int64_t l_, int64_t r_) : l(l_), r(r_)
    {
    }

    bool contains(int64_t x) const
    {
        return x >= l && x <= r;
    }
};

int part1(FILE *inFile)
{
    char *buf = NULL;
    int res = 0;
    std::vector<Range> ranges;
    for (;;)
    {
        size_t n, len;
        if ((n = getline(&buf, &len, inFile)) == -1)
            break;
        buf[n - 1] = '\0';
        n--;
        if (n == 0)
            break;

        char *mid = std::strchr(buf, '-');
        int64_t l = std::stol(buf), r = std::stol(mid + 1);
        ranges.emplace_back(l, r);
    }
    for (;;)
    {
        int64_t id;
        if (fscanf(inFile, "%ld\n", &id) == EOF)
            break;
        int ok = 0;
        for (const auto &range : ranges)
        {
            if (range.contains(id))
            {
                ok = 1;
                break;
            }
        }
        res += ok;
    }
    printf("%d\n", res);
    free(buf);

    return 0;
}
