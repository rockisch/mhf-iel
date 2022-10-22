#define EXPORTING_DLL
#include "dll.h"

#include "main.c"

int DllMain(struct IElessData data) {
    return start(data.char_id, data.is_new, data.logintoken, data.messages, data.message_size);
}
