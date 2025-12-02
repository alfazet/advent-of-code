EXT ?= c
DAY ?= day-01
MAIN = $(DAY)/main
PART1 = $(DAY)/part1
PART2 = $(DAY)/part2

ifeq ($(EXT),c)
	CC = gcc
	FLAGS = -O3 -fopenmp
else
	CC = g++
	FLAGS = --std=c++20 -O3 -fopenmp
endif

$(MAIN): $(MAIN).o $(PART1).o $(PART2).o
	$(CC) $(MAIN).o $(PART1).o $(PART2).o -o $(MAIN)

$(MAIN).o: $(MAIN).$(EXT) $(PART1).$(EXT) $(PART2).$(EXT)
	$(CC) -c $(MAIN).$(EXT) -o $(MAIN).o

$(PART1).o: $(PART1).$(EXT)
	$(CC) -c $(PART1).$(EXT) -o $(PART1).o

$(PART2).o: $(PART2).$(EXT)
	$(CC) -c $(PART2).$(EXT) -o $(PART2).o

clean:
	rm -f **/*.o **/main
