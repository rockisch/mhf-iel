#include "main.c"

int main(int argc, char **argv) {
    if (argc < 3) {
        printf("\nusage: main.exe <charID> <logintoken> [loginmessages...]");
        return 0;
    }
    uint32_t charID = atoi(argv[1]);
    char *logintoken = argv[2];

    char **marr = malloc(8 * sizeof(char**));
    int mlen = 0;
    if (argc > 3) {
        mlen = argc - 3;
        char **mptr = argv + 3;
        for (int i = 0; i < mlen && i < 8; i++) {
            marr[i] = mptr[i];
        }
    }

    start(charID, logintoken, marr, mlen);
    free(marr);
}
