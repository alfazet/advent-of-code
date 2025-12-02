#include "part1.h"

constexpr int64_t MAX_N = 1e7;

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

int part1(FILE *inFile)
{
    std::vector<Range> ranges;
    std::string cur;
    int64_t l, r;
    char c;
    while ((c = fgetc(inFile)) != EOF)
    {
        if (c == '-')
        {
            l = std::stol(cur);
            cur.clear();
        }
        else if (c == ',' || c == '\n')
        {
            r = std::stol(cur);
            ranges.emplace_back(l, r);
            cur.clear();
        }
        else
        {
            cur.push_back(c);
        }
    }

    uint64_t res = 0;
    for (int half = 1; half <= MAX_N; half++)
    {
        int64_t x = half;
        int p = intLen(half);
        while (p--)
        {
            x *= 10;
        }
        x += half;
        for (const auto &range : ranges)
        {
            if (range.contains(x))
            {
                res += x;
                break;
            }
        }
    }
    printf("%ld\n", res);

    return 0;
}
