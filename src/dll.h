#ifndef IELESS_DLL_H
#define IELESS_DLL_H

#include <inttypes.h>

struct IElessData {
    uint32_t charID;
    char *logintoken;
    char **messages;
    size_t message_size;
};

#ifdef EXPORTING_DLL
extern __declspec(dllexport) int DllMain(struct IElessData);
#else
extern __declspec(dllimport) int DllMain(struct IElessData);
#endif

#endif
