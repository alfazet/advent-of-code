#include "part1.h"

#define BUF_SIZE 1024
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int part1(FILE *inFile)
{
    int res = 0, sufMax[BUF_SIZE];
    char *buf = NULL;
    for (;;)
    {
        size_t nRead, len;
        if ((nRead = getline(&buf, &len, inFile)) == -1)
            break;
        buf[nRead - 1] = '\0';
        nRead--;

        sufMax[nRead - 1] = buf[nRead - 1] - '0';
        for (int i = nRead - 2; i >= 0; i--)
        {
            sufMax[i] = MAX(sufMax[i + 1], buf[i] - '0');
        }
        int mx = (buf[0] - '0') * 10 + sufMax[1];
        for (int i = 1; i < nRead - 1; i++)
        {
            mx = MAX(mx, (buf[i] - '0') * 10 + sufMax[i + 1]);
        }
        res += mx;
    }
    printf("%d\n", res);
    free(buf);

    return 0;
}
