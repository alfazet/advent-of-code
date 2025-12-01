#include "part2.h"

int part2(FILE *inFile)
{
    char dir;
    int where = 50, prev_where, res = 0, x;
    for (;;)
    {
        if (fscanf(inFile, "%c%d\n", &dir, &x) == EOF)
            break;
        prev_where = where;
        res += x / 100;
        switch (dir)
        {
        case 'L':
            where -= x % 100;
            break;
        case 'R':
            where += x % 100;
            break;
        }
        if (prev_where != 0 && (where < 1 || where > 99))
            res++;
        while (where < 0)
            where += 100;
        while (where >= 100)
            where -= 100;
    }
    printf("%d\n", res);

    return 0;
}
