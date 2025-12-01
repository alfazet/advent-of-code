main: main.o part1.o part2.o
	gcc main.o part1.o part2.o -o main

main.o: main.c part1.c part2.c
	gcc -c main.c -o main.o

part1.o: part1.c
	gcc -c part1.c -o part1.o

part2.o: part2.c
	gcc -c part2.c -o part2.o

clean:
	rm -f *.o main
