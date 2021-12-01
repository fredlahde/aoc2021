/* clang -ansi -pedantic -Wall -Wextra -Werror -ggdb -g -march=native -fsanitize=address  part2.c && ./a.out  */
#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

typedef struct
{
    int *array;
    size_t used;
    size_t size;
} Array;

void initArray(Array *a, size_t initialSize)
{
    a->array = malloc(initialSize * sizeof(int));
    a->used = 0;
    a->size = initialSize;
}

void insertArray(Array *a, int element)
{
    if (a->used == a->size)
    {
        a->size *= 2;
        a->array = realloc(a->array, a->size * sizeof(int));
    }
    a->array[a->used++] = element;
}

void freeArray(Array *a)
{
    free(a->array);
    a->array = NULL;
    a->used = a->size = 0;
}

int main()
{
    char *path = "./input";
    char *contents;
    int fd;
    struct stat statbuf;
    int err;
    int len;
    int last, count, sum;
    char *tok;
    char *contents_heap;
    Array numbers;
    size_t i;

    fd = open(path, O_RDONLY);
    if (fd < 0)
    {
        printf("failed to read file\n");
        goto err;
    }

    err = fstat(fd, &statbuf);
    if (err < 0)
    {
        printf("failed to fstat file\n");
        goto err;
    }

    contents = (char *)mmap(0, statbuf.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    if (contents == MAP_FAILED)
    {
        printf("failed to mmap file\n");
        goto err;
    }

    len = strlen(contents);
    contents_heap = malloc(len);
    if (NULL == contents_heap)
    {
        printf("failed to malloc string\n");
        goto err;
    }
    if (NULL == memcpy(contents_heap, contents, len))
    {
        printf("failed to memcpy string\n");
        goto err;
    }
    tok = strtok(contents_heap, "\n");

    initArray(&numbers, 10);
    while (tok != NULL)
    {
        int curr;
        curr = strtol(tok, NULL, 10);
        insertArray(&numbers, curr);
        tok = strtok(NULL, "\n");
    }
    free(contents_heap);

    i = 0;
    last = !0;
    count = 0;
    while (1)
    {
        int one, two, three;
        if (i == numbers.used)
            break;
        one = *(numbers.array + i);
        if (i + 1 > numbers.used)
        {
            two = 0;
        }
        else
        {
            two = *(numbers.array + i + 1);
        }
        if (i + 2 > numbers.used)
        {
            three = 0;
        }
        else
        {
            three = *(numbers.array + i + 2);
        }

        sum = one + two + three;
        if (sum > last && last != !0)
            count++;
        last = sum;
        i++;
    }

    freeArray(&numbers);
    printf("%d\n", count);

err:
    close(fd);
    return 0;
}
