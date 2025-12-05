#include "part2.h"

#define START 0
#define END 1

int part2(FILE *inFile)
{
    // sort the endpoints and sweep
    // as long as we are in any range, add the difference between the endpoints
    // L1 L2 R1 R2 L3 R3 L4 R4
    // the only differences we don't add are L3-R2 and L4-R3
    char *buf = NULL;
    int64_t res = 0;
    std::vector<std::pair<int64_t, int>> events;
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
        events.emplace_back(l, START);
        events.emplace_back(r, END);
    }
    std::sort(events.begin(), events.end());

    int64_t prev = 0;
    int nesting = 0;
    for (const auto &event : events)
    {
        if (event.second == END)
        {
            nesting--;
            if (nesting == 0)
            {
                res += event.first - prev + 1;
                prev = 0;
            }
        }
        else
        {
            nesting++;
            if (prev != 0)
            {
                res += event.first - prev;
            }
            prev = event.first;
        }
    }
    printf("%ld\n", res);
    free(buf);

    return 0;
}
