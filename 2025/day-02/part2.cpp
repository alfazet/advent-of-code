#include "part2.h"

constexpr int MAX_LEN = 14;

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

int part2(FILE *inFile)
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

    std::unordered_set<int64_t> found;
    uint64_t res = 0;
    for (int reps = 2; reps <= MAX_LEN; reps++)
    {
        int maxPart = 1;
        for (int i = 0; i < MAX_LEN / reps; i++)
        {
            maxPart *= 10;
        }
        for (int part = 1; part < maxPart; part++)
        {
            int64_t x = part;
            int partLen = intLen(part);
            for (int i = 0; i < reps - 1; i++)
            {
                for (int j = 0; j < partLen; j++)
                {
                    x *= 10;
                }
                x += part;
            }
            if (found.count(x) > 0)
            {
                continue;
            }
            for (const auto &range : ranges)
            {
                if (range.contains(x))
                {
                    found.insert(x);
                    res += x;
                    break;
                }
            }
        }
    }
    printf("%ld\n", res);

    return 0;
}
