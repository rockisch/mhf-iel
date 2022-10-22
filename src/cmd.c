#include "main.c"

int main(int argc, char **argv) {
    int FIXED_ARGS = 4;
    if (argc < FIXED_ARGS) {
        printf("\nusage: main.exe <charID> <isNewChar> <logintoken> [loginmessages...]");
        return 0;
    }
    uint32_t char_id = atoi(argv[1]);
    uint8_t is_new = atoi(argv[2]) > 0;
    char *logintoken = argv[3];

    char **marr = malloc(8 * sizeof(char**));
    int mlen = 0;
    if (argc > FIXED_ARGS) {
        mlen = argc - FIXED_ARGS;
        char **mptr = argv + FIXED_ARGS;
        for (int i = 0; i < mlen && i < 8; i++) {
            marr[i] = mptr[i];
        }
    }

    start(char_id, is_new, logintoken, marr, mlen);
    free(marr);
}
