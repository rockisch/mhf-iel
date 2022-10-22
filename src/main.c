#include "main.h"

#define BUFSIZE 1024

const char *MHF_LABEL = "Monster Hunter Frontier Z";

static struct initdata INITDATA = {
    .DOS_header_ptr = 0x400000,
    .fixed448994_0x7 = 0x7,
    .fixed448a64_0x0 = 0x0,
    .fixed448ed0_0x1 = 0x1,
    .fixed4491ac_0x10 = 0x10,
    .fixed4491b8_0x10 = 0x10,
};

uint32_t gg_proc() {
    // GameGuard function, 1 means everything's ok.
    return 1;
}

uint32_t check_0x4466cc_proc(int a) {
    if (a != 0) {
        // Checks 0x004466cc, if it's different than 0, does something
        // From my tests it seems like it's always 0, so might be an edge case or something legacy
    }
    return 0;
}

void get_current_path(char *buf, size_t size) {
    uint32_t len = GetModuleFileNameA(NULL, buf, size);
    int i;
    for (i = len-1; i > 0; i--) {
        if (buf[i] == '\\')
            break;
    }
    if (i > 0) {
        while (i < len) {
            buf[i+1] = '\0';
            i++;
        }
    }
}

void __cdecl get_mhf_name(char *buf, size_t size, char *name) {
    sprintf_s(buf, size, "%s %s", MHF_LABEL, name);
}

HANDLE __cdecl get_mutex(char *name) {
    HANDLE hMutex = CreateMutexA((LPSECURITY_ATTRIBUTES)0x0, 0, name);
    DWORD DVar1 = GetLastError();
    if (DVar1 != 0x0) {
        ReleaseMutex(hMutex);
        CloseHandle(hMutex);
        return 0;
    }
    return hMutex;
}

void init_global_alloc(char **messages, size_t messages_size) {
    INITDATA.global_alloc = GlobalAlloc(0x42, 0x8ae0);
    uint32_t *global_ptr = GlobalLock(INITDATA.global_alloc);
    uint32_t *len_arr_ptr = global_ptr + 643;
    uint32_t *body_arr_ptr = len_arr_ptr + 8;
    for (int i = 0; i < messages_size; i++) {
        size_t len = strlen(messages[i]);
        len_arr_ptr[0] = len + 1;
        len_arr_ptr++;
        strncpy_s((char*)body_arr_ptr, 1024 * 4, messages[i], 1024 * 4);
        body_arr_ptr += 1024;
    }
    GlobalUnlock(INITDATA.global_alloc);
}

void init_init_config() {
    INITDATA.PRESET_LEVEL = GetPrivateProfileIntA("SET", "PRESET_LEVEL", 0, INITDATA.ini_name);
    INITDATA.CUSTOM = GetPrivateProfileIntA("SET", "CUSTOM", 1, INITDATA.ini_name);
    INITDATA.FULLSCREEN_MODE = GetPrivateProfileIntA("SCREEN", "FULLSCREEN_MODE", 1, INITDATA.ini_name);
    INITDATA.WINDOW_RESOLUTION_W = GetPrivateProfileIntA("SCREEN", "WINDOW_RESOLUTION_W", 1920, INITDATA.ini_name);
    INITDATA.WINDOW_RESOLUTION_H = GetPrivateProfileIntA("SCREEN", "WINDOW_RESOLUTION_H", 1080, INITDATA.ini_name);
    INITDATA.FULLSCREEN_RESOLUTION_W = GetPrivateProfileIntA("SCREEN", "FULLSCREEN_RESOLUTION_W", 1920, INITDATA.ini_name);
    INITDATA.FULLSCREEN_RESOLUTION_H = GetPrivateProfileIntA("SCREEN", "FULLSCREEN_RESOLUTION_H", 1080, INITDATA.ini_name);
    INITDATA.DISP_MAX_CHAR = GetPrivateProfileIntA("VIDEO", "DISP_MAX_CHAR", 100, INITDATA.ini_name);
    INITDATA.TEXTURE_DXT_USE = GetPrivateProfileIntA("VIDEO", "TEXTURE_DXT_USE", 0, INITDATA.ini_name);
    INITDATA.NOW_MONITOR_WH = GetPrivateProfileIntA("VIDEO", "NOW_MONITOR_WH", 0, INITDATA.ini_name);
    INITDATA.GRAPHICS_VER = GetPrivateProfileIntA("VIDEO", "GRAPHICS_VER", 1, INITDATA.ini_name);
    INITDATA.SOUND_NOTUSE = GetPrivateProfileIntA("SOUND", "SOUND_NOTUSE", 0, INITDATA.ini_name);
    INITDATA.SOUND_VOLUME = GetPrivateProfileIntA("SOUND", "SOUND_VOLUME", 0, INITDATA.ini_name);
    INITDATA.SOUND_VOLUME_INACTIVITY = GetPrivateProfileIntA("SOUND", "SOUND_VOLUME_INACTIVITY", 0, INITDATA.ini_name);
    INITDATA.SOUND_VOLUME_MINIMIZE = GetPrivateProfileIntA("SOUND", "SOUND_VOLUME_MINIMIZE", 0, INITDATA.ini_name);
    INITDATA.SOUND_FREQUENCY = GetPrivateProfileIntA("SOUND", "SOUND_FREQUENCY", 48000, INITDATA.ini_name);
    INITDATA.SOUND_BUFFERNUM = GetPrivateProfileIntA("SOUND", "SOUND_BUFFERNUM", 2048, INITDATA.ini_name);
    INITDATA.LANGUAGE = GetPrivateProfileIntA("LOCALIZATION", "LANGUAGE", 0, INITDATA.ini_name);
    INITDATA.FONT_QUALITY = GetPrivateProfileIntA("FONT", "QUALITY", 4, INITDATA.ini_name);
    INITDATA.FONT_WEIGHT = GetPrivateProfileIntA("FONT", "WEIGHT", 0x2bc, INITDATA.ini_name);
    GetPrivateProfileStringA("FONT", "NAME", (uint8_t[]){0x4d, 0x53, 0x20, 0x3f, 0x3f, 0x3f, 0x3f}, INITDATA.FONT_NAME, 32 * sizeof(uint32_t), INITDATA.ini_name);
    INITDATA.DRAWSKIP = GetPrivateProfileIntA("OPTION", "DRAWSKIP", 1, INITDATA.ini_name);
    INITDATA.CLOGDIS = GetPrivateProfileIntA("OPTION", "CLOGDIS", 0, INITDATA.ini_name);
    INITDATA.PROXY_USE = GetPrivateProfileIntA("LAUNCH", "PROXY_USE", 0, INITDATA.ini_name);
    INITDATA.PROXY_IE = GetPrivateProfileIntA("LAUNCH", "PROXY_IE", 0, INITDATA.ini_name);
    INITDATA.PROXY_SET = GetPrivateProfileIntA("LAUNCH", "PROXY_SET", 1, INITDATA.ini_name);
    GetPrivateProfileStringA("LAUNCH", "PROXY_ADDR", "127.0.0.1", INITDATA.PROXY_ADDR, 16 * sizeof(uint32_t), INITDATA.ini_name);
    INITDATA.PROXY_PORT = GetPrivateProfileIntA("LAUNCH", "PROXY_PORT", 8888, INITDATA.ini_name);
    INITDATA.SERVER_SEL = GetPrivateProfileIntA("LAUNCH", "SERVER_SEL", 1, INITDATA.ini_name);
}

int start(uint32_t char_id, uint8_t is_new, char *logintoken, char **messages, size_t messages_size) {
    char buf[BUFSIZE];

    strncpy(INITDATA.server_sign_token, logintoken, 16);
    strcpy(INITDATA.ini_name, "mhf.ini");
    strcpy(INITDATA.server_sign_addr, "127.0.0.1:53310");
    strcpy(INITDATA.server_sign_host, "mhf-n.capcom.com.tw");
    strcpy(INITDATA.alt_ip_address, "203.191.249.36:8080");

    INITDATA.server_sign_character_id_selected_1 = char_id;
    INITDATA.server_sign_character_id_selected_2 = char_id;
    INITDATA.server_sign_0xffffffff = 0xffffffff;
    INITDATA.server_sign_current_ts = time(NULL);
    INITDATA.server_sign_patch_count = 0x0;
    INITDATA.server_sign_entrance_count = 0x1;
    INITDATA.server_sign_character_status = is_new ? 2 : 0;
    INITDATA.server_sign_exp_hr = 0x0;
    INITDATA.server_sign_character_id_list[0] = char_id;
    INITDATA.server_sign_expiry_ts = time(NULL) + (3600 * 24 * 7);

    get_current_path(buf, BUFSIZE);
    SetCurrentDirectoryA(buf);
    strncpy(INITDATA.path, buf, 1024);
    strncpy(INITDATA.path2, buf, 1024);
    get_mhf_name(INITDATA.ms_name, 64, "MHF_MASTER");
    get_mhf_name(INITDATA.msr_name, 64, "MHF_MASTER_READY");
    INITDATA.keyboard_layout = GetKeyboardLayout(0);
    init_init_config();

    INITDATA.initdata_ptr = &INITDATA;
    INITDATA.inner1_ptr = &INITDATA.inner1_addr_0x0;
    INITDATA.inner2_ptr = &INITDATA.inner2_addr_0x0;
    INITDATA.inner3_ptr = &INITDATA.inner3_addr_0x0;
    INITDATA.gg_proc_ptr = &gg_proc;
    INITDATA.unk1_proc_ptr = &check_0x4466cc_proc;
    INITDATA.unk2_proc_ptr = &check_0x4466cc_proc;

    INITDATA.unk448e74_0xe = 0xe;
    INITDATA.unk4491e8_0x10000000 = 0x10000000;

    INITDATA.ms_mutex = get_mutex(INITDATA.ms_name);
    if (!INITDATA.ms_mutex) {
        fprintf(stderr, "failed to get '%s' mutex\n", INITDATA.ms_name);
        return EXIT_FAILURE;
    }

    init_global_alloc(messages, messages_size);
    if (GetLastError()) {
        fprintf(stderr, "failed to initialize global allocation\n");
        return EXIT_FAILURE;
    }

    sprintf_s(buf, BUFSIZE, "%s%s", &INITDATA.path, "mhfo-hd.dll");
    HINSTANCE mhDLL = LoadLibraryA(buf);
    if (!mhDLL) {
        fprintf(stderr, "failed to load 'mhfo-hd.dll' dll: %d\n", GetLastError());
        return EXIT_FAILURE;
    }
    INITDATA.mhDLL_Main = GetProcAddress(mhDLL, "mhDLL_Main");
    if (!INITDATA.mhDLL_Main) {
        fprintf(stderr, "failed to load 'mhDLL_Main' func: %d\n", GetLastError());
        return EXIT_FAILURE;
    }
    uint32_t result = INITDATA.mhDLL_Main(&INITDATA);
    FreeLibrary(mhDLL);
    return result;
}
