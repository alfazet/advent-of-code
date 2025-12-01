#include "part1.h"
#include "part2.h"

int main(int argc, char **argv)
{
    printf("--- Day 1: Secret Entrance ---\n");
    if (argc < 2)
    {
        fprintf(stderr, "missing part number\n");
        return 1;
    }
    if (argc < 3)
    {
        fprintf(stderr, "missing input file\n");
        return 1;
    }

    const char *path = argv[2];
    FILE *inFile = fopen(path, "r");
    if (inFile == NULL)
    {
        fprintf(stderr, "couldn't open file `%s`\n", path);
        return 1;
    }
    int part = atoi(argv[1]);
    switch (part)
    {
    case 1:
        if (part1(inFile) != 0)
            fprintf(stderr, "fatal error in part 1\n");
        break;
    case 2:
        if (part2(inFile) != 0)
            fprintf(stderr, "fatal error in part 2\n");
        break;
    default:
        fprintf(stderr, "part must be 1 or 2\n");
    }
    fclose(inFile);

    return 0;
}
