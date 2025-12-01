#include "part1.h"

int part1(FILE *inFile)
{
    char dir;
    int where = 50, res = 0, x;
    for (;;)
    {
        if (fscanf(inFile, "%c%d\n", &dir, &x) == EOF)
            break;
        switch (dir)
        {
        case 'L':
            where -= x;
            while (where < 0)
                where += 100;
            break;
        case 'R':
            where += x;
            while (where >= 100)
                where -= 100;
            break;
        }
        if (where == 0)
            res++;
    }
    printf("%d\n", res);

    return 0;
}
