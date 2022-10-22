#include <windows.h>
#include <stdio.h>
#include <inttypes.h>
#include <Shlwapi.h>
#include <WinBase.h>
#include <time.h>
#include <WinUser.h>

struct initdata {
    uint32_t DOS_header_ptr; // 447178
    uint32_t _pad1[4]; // 44717c

    char path[256 * sizeof(uint32_t)]; // 44718c
    char path2[256 * sizeof(uint32_t)]; // 44758c
    char username[512 * sizeof(uint32_t)]; // 44798c
    char password[512 * sizeof(uint32_t)]; // 44818c

    uint32_t _pad2[2]; // 44898c
    uint32_t fixed448994_0x7; // 448994
    uint32_t _pad3[1]; // 448998
    HANDLE ms_mutex; // 44899c
    uint32_t _pad4[1]; // 4489a0 -- check what this is
    char ms_name[16 * sizeof(uint32_t)]; // 4489a4
    char ini_name[16 * sizeof(uint32_t)]; // 4489e4
    uint32_t (*unk1_proc_ptr)(uint); // 448a24
    uint32_t (*gg_proc_ptr)(); // 448a28
    uint32_t (*unk2_proc_ptr)(uint); // 448a2c
    uint32_t _pad5[3]; // 448a30

    // Sign response data
    uint32_t server_sign_character_id_selected_1; // 448a3c
    uint32_t server_sign_character_id_selected_2; // 448a40
    uint32_t server_sign_0xffffffff; // 448a44
    char server_sign_token[4 * sizeof(uint32_t)]; // 448a48
    uint32_t _pad6[2]; // 448a58
    uint32_t server_sign_current_ts; // 448a60
    uint32_t fixed448a64_0x0; // 448a64
    uint32_t _pad8[128]; // 448a68
    char server_sign_addr[96 * sizeof(uint32_t)]; // 448c68
    char server_sign_host[32 * sizeof(uint32_t)]; // 448d68
    uint32_t server_sign_patch_count; // 448e68
    uint32_t server_sign_entrance_count; // 448e6c
    // 0 if existing, 2 if new. I can see from the disassembly it can also be 1, which the game seems to treat as 2,
    // but that's based on a byte set on the individual character data that I couldn't find where to set.
    uint32_t server_sign_character_status; // 448e70
    uint32_t unk448e74_0xe; // 448e74 -- something based on 'patch_count', if it differs from 0 this value gets zeroed
    uint32_t server_sign_exp_hr; // 448e78
    char server_sign_character_name_sel[4 * sizeof(uint32_t)]; // 448e7c
    uint32_t server_sign_character_id_list[16]; // 448e8c

    HGLOBAL global_alloc; // 448ecc
    uint32_t fixed448ed0_0x1;  // 448ed0
    uint32_t unk448ed4; // 448ed4
    uint32_t unk448ed8; // 448ed8
    uint32_t _pad448edc[2]; //448edc

    // Init config
    uint32_t PRESET_LEVEL; // 448ee4
    uint32_t CUSTOM;  // 448ee8
    uint32_t FULLSCREEN_MODE;  // 448eec
    uint32_t WINDOW_RESOLUTION_W; // 448ef0
    uint32_t WINDOW_RESOLUTION_H; // 448ef4
    uint32_t FULLSCREEN_RESOLUTION_W; // 448ef8
    uint32_t FULLSCREEN_RESOLUTION_H; // 448efc
    uint32_t DISP_MAX_CHAR; // 448f00
    uint32_t TEXTURE_DXT_USE; // 448f04
    uint32_t NOW_MONITOR_WH; // 448f08
    uint32_t GRAPHICS_VER; // 448f0c
    uint32_t SOUND_NOTUSE; // 448f10
    uint32_t SOUND_VOLUME; // 448f14
    uint32_t SOUND_VOLUME_INACTIVITY; // 448f18
    uint32_t SOUND_VOLUME_MINIMIZE; // 448f1c
    uint32_t SOUND_FREQUENCY; // 448f20
    uint32_t SOUND_BUFFERNUM; // 448f24
    uint32_t LANGUAGE; // 448f28 -- 0x0
    uint32_t FONT_QUALITY; // 448f2c -- 0x4
    uint32_t FONT_WEIGHT; // 448f30 -- 0x2bc
    char FONT_NAME[26 * sizeof(uint32_t)]; // 448f34 -- [0x3f20534d, 0x3f3f3f, 0x3f3f3f] is the default
    uint32_t DRAWSKIP; // 448f9c -- 0x1
    uint32_t CLOGDIS; // 448fa0 -- 0x0
    uint32_t PROXY_USE; // 448fa4
    uint32_t PROXY_IE; // 448fa8
    uint32_t PROXY_SET; // 448fac
    char PROXY_ADDR[16 * sizeof(uint32_t)]; // 448fb0
    uint32_t PROXY_PORT; // 448ff0
    uint32_t SERVER_SEL; // 448ff4

    void *inner1_ptr; // 448ff8
    uint32_t unk15[32]; // 448ffc
    char alt_ip_address[64 * sizeof(uint32_t)]; // 44907c
    uint32_t server_sign_expiry_ts; // 44917c
    uint32_t unk16; // 449180
    uint32_t unk17_0x1; // 449184
    uint32_t unk18[2]; // 449188
    void *initdata_ptr; // 449190 -- 0x447178
    HKL keyboard_layout; // 449194
    uint32_t inner3_addr_0x0[4]; // 449198
    uint32_t inner1_addr_0x0; // 4491a8
    uint32_t fixed4491ac_0x10; // 4491ac
    void *inner2_ptr; // 4491b0 -- 0x4491d4
    uint32_t unk24_0x0; // 4491b4
    uint32_t fixed4491b8_0x10; // 4491b8
    void *inner3_ptr; // 4491bc
    uint32_t proc4_ptr; // 4491c0
    uint32_t unk28_0x0; // 4491c4
    uint32_t proc5_ptr; // 4491c8
    uint32_t unk30[2]; // 4491cc
    uint32_t inner2_addr_0x0; // 4491d4
    uint32_t unk32[4]; // 4491d8
    uint32_t unk4491e8_0x10000000; // 4491e8
    uint32_t unk34_0x108c0000; // 4491ec -- can be diff
    uint32_t unk4491f0[338]; // 4491f0
    char msr_name[64 * sizeof(uint32_t)];
    uint32_t unk449810[261]; // 449810
    FARPROC mhDLL_Main; // 449c24
    uint32_t unk449c28; // 449c28
    uint32_t unk449c2c[4]; // 449c2c
    uint32_t unk449c3c; // 449c3c
};
