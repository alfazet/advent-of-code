#include "part1.h"

#define BUF_SIZE 256

static int DX[8] = {-1, -1, -1, 0, 0, 1, 1, 1};
static int DY[8] = {-1, 0, 1, -1, 1, -1, 0, 1};

int part1(FILE *inFile)
{
    int res = 0, line = 0, dim = -1;
    char grid[BUF_SIZE][BUF_SIZE];
    char *buf = NULL;
    for (;;)
    {
        size_t n, len;
        if ((n = getline(&buf, &len, inFile)) == -1)
            break;
        buf[n - 1] = '\0';
        n--;

        if (dim == -1)
            dim = n;
        for (int i = 0; i < n; i++)
        {
            grid[line][i] = buf[i];
        }
        line++;
    }

    for (int i = 0; i < dim; i++)
    {
        for (int j = 0; j < dim; j++)
        {
            if (grid[i][j] != '@')
                continue;
            int ni, nj, occupied = 0;
            for (int k = 0; k < 8; k++)
            {
                ni = i + DX[k];
                nj = j + DY[k];
                if (ni >= 0 && ni < dim && nj >= 0 && nj < dim && grid[ni][nj] == '@')
                    occupied++;
            }
            if (occupied < 4)
                res++;
        }
    }

    printf("%d\n", res);
    free(buf);

    return 0;
}
