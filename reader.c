#include <errno.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>

FILE* open_packaged(void* package_ptr, char* identifier, const char* modes, uint64_t *out_size){
    uint64_t header_size = *(uint64_t*)package_ptr;
    uint64_t offset = sizeof(uint64_t);

    bool found = false;
    uint64_t target_start;
    uint64_t target_size;

    while(offset < header_size && !found){
        target_size = *(uint64_t*)(package_ptr + offset);
        offset += sizeof(uint64_t);

        target_start = *(uint64_t*)(package_ptr + offset);
        offset += sizeof(uint64_t);

        //See if matches identifier
        if(strcmp(identifier, (char*)(package_ptr + offset)) == 0){
            found = true;
        }

        offset += strlen(package_ptr + offset) + 1;
    }

    if(!found){ return NULL; } //No matching file
    *out_size = target_size;
    return fmemopen(package_ptr + target_start, target_size, modes);
}

int main(int argc, char** argv){
    if(argc != 2){
        fprintf(stderr, "Error, please provide a .pak to preview");
        return 1;
    }

    //Open file
    FILE* chunk = fopen(argv[1], "rb");
    if(chunk == -1){
        perror("File handle error.");
        exit(1);
    }

    int handle = fileno(chunk);

    //Measure length
    fseek(chunk, 0, SEEK_END);
    long flength = ftell(chunk);
    fseek(chunk, 0, SEEK_SET);

    //Memory-map file
    void *memory_mapped = mmap(NULL,flength, PROT_READ, MAP_SHARED, handle, 0);
    if(memory_mapped == NULL | (int)memory_mapped == -1){
        perror("Memory-mapping error. Exiting...\nCause");
        exit(1);
    }


    //Open packaged file "test"
    FILE* test = open_packaged(memory_mapped, "test", "rb");
    if(test == NULL){
        fprintf(stderr, "Error opening package");
        exit(1);
    }

    char byte;
    while((byte = fgetc(test)) != EOF){
        printf("%c", byte);
    }
}
