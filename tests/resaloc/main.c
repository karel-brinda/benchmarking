#include <unistd.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <inttypes.h>


int main(int argc, char *argv[]) {

	if (argc != 3 && argc !=2){
		printf("resalloc - allocate resources for the purpose of benchmarking\n");
		printf("usage: resalloc <bytes_of_mem> [<secs_to_wait>]\n");
		return 1;
	}

	float secs = 0.5;
	if (argc==3) {
		secs = atof(argv[2]);
	}
	char * badString = "notPossible";
	char * endPtr;
	int64_t bytes = strtoll( argv[1], &endPtr, 10 );

	printf("allocating %" PRId64 " bytes of memory\n", bytes);
	void *ptr = malloc(bytes);

	printf("sleeping for %f seconds\n", secs);
	sleep(secs);

	printf("freeing memory\n");
	free(ptr);

	return 0;
}
