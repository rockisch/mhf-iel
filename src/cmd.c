#include "main.c"

int main(int argc, char **argv) {
    int FIXED_ARGS = 4;
    if (argc < FIXED_ARGS) {
        printf("usage: main.exe <charID> <isNewChar> <logintoken> [loginmessages...]");
        return 0;
    }
    uint32_t char_id = atoi(argv[1]);
    uint8_t is_new = atoi(argv[2]) > 0;
    char *logintoken = argv[3];
    if (strlen(logintoken) != 16) {
        fprintf(stderr, "error: logintoken must be 16 characters long");
        return 0;
    }

    char **marr = malloc(8 * sizeof(char**));
    int mlen = 0;
    if (argc > FIXED_ARGS) {
        mlen = argc - FIXED_ARGS;
        char **mptr = argv + FIXED_ARGS;
        for (int i = 0; i < mlen && i < 8; i++) {
            marr[i] = mptr[i];
        }
    }

    uint32_t result = start(char_id, is_new, logintoken, marr, mlen);
    printf("game closed with status %d", result);
    free(marr);
}
