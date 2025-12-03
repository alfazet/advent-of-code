#include "part2.h"

#define BUF_SIZE 1024
#define N_DIGITS 12
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int part2(FILE *inFile)
{
    // dp[i][k] = what's the largest k-digit number
    // we can make out of the last i digits
    int64_t res = 0;
    int64_t dp[BUF_SIZE][N_DIGITS + 1];
    char *buf = NULL;
    for (;;)
    {
        size_t nRead, len;
        if ((nRead = getline(&buf, &len, inFile)) == -1)
            break;
        buf[nRead - 1] = '\0';
        nRead--;

        memset(dp, 0, BUF_SIZE * (N_DIGITS + 1) * sizeof(int64_t));
        dp[nRead - 1][1] = buf[nRead - 1] - '0';
        for (int i = nRead - 2; i >= 0; i--)
        {
            int64_t p = buf[i] - '0';
            dp[i][1] = MAX(p, dp[i + 1][1]);
            for (int k = 2; k <= N_DIGITS; k++)
            {
                p *= 10;
                // we can make a k-digit number here
                // only if a (k-1)-digit number was possible
                // starting from the next spot...
                if (dp[i + 1][k - 1] != 0)
                    dp[i][k] = MAX(dp[i][k], p + dp[i + 1][k - 1]);
                // ...or just take whatever the previous max was
                dp[i][k] = MAX(dp[i][k], dp[i + 1][k]);
            }
        }
        res += dp[0][N_DIGITS];
    }
    printf("%ld\n", res);
    free(buf);

    return 0;
}
