all: miv

kilo: miv.c
	$(CC) -o miv miv.c -Wall -W -pedantic -std=c99

clean:
	rm miv