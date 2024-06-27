use crate::utils::bufcopy;
use crate::{utils, CliFlags, Error, MhfConfig, MhfVersion, Result};

use windows::core::s;
use windows::Win32::Foundation::{FreeLibrary, FARPROC, HANDLE, HGLOBAL, HMODULE};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::Memory::{GlobalLock, GlobalUnlock};
use windows::Win32::System::WindowsProgramming::{GetPrivateProfileIntA, GetPrivateProfileStringA};
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
use windows::Win32::UI::TextServices::HKL;

extern "C" fn mock_proc(_v: u32) -> u32 {
    // TODO: investigate individual procs
    0
}

extern "C" fn gg_proc() -> u32 {
    // TODO: I'm pretty sure this isn't called anymore in the fixed version, check
    // let mut x: u32 = 0;
    // unsafe {
    //     std::arch::asm!(
    //         "mov ebx, eax",
    //         out("ebx") x,
    //     );
    // }
    // let data = unsafe { &mut *DATA.get() }.0;
    1
}

#[derive(Debug)]
#[repr(C)]
struct DataF5 {
    main_module: HMODULE,   // 447178
    _pad_44717c: [u8; 0x8], // 44717c
    cmd_flags_1: u32,       // 447184 // set based on CLI flags
    cmd_flags_2: u32,       // 447188 // set based on CLI flags

    path1: [u8; 0x400],         // 44718c
    path2: [u8; 0x400],         // 44758c
    user_name: [u8; 0x800],     // 44798c
    user_password: [u8; 0x800], // 44818c

    cmd_number: u32,               // 44898c // CLI argument stuff // 1 in ZZ, 2 in F5
    cmd_netfcup: u32,              // 448990 // CLI '-NETFCUP'
    cmd_dmm: u32,                  // 448994 // set whenever a /DMM_<X> flag is specified // 7 in ZZ
    _pad_448998: [u8; 0x4],        // 448998 // set when mutex_master is already set?
    mutex_master: HANDLE,          // 44899c
    mutex_master_ready: HANDLE,    // 4489a0
    mutex_master_name: [u8; 0x40], // 4489a4
    ini_file: [u8; 0x40],          // 4489e4
    proc_1: usize,                 // 448a24 // copied from 40187a
    proc_2: usize,                 // 448a28 // copied from 401868, probably gg_proc
    proc_3: usize,                 // 448a2c // copied from 40188b
    _pad_448a30: [u8; 0xc],        // 448a30

    // Server data
    selected_char_id_1: u32,    // 448a3c
    selected_char_id_2: u32,    // 448a40
    user_token_id: u32,         // 448a44
    user_token: [u8; 0x10],     // 448a48
    _pad_448a58: [u8; 0x8],     // 448a58
    server_current_ts: u32,     // 448a60
    fixed_448a64_0x0: u32,      // 448a64
    _pad_448a68: [u8; 0x200],   // 448a68
    remote_addr: [u8; 0x100],   // 448c68
    remote_host: [u8; 0x100],   // 448d68
    remote_patch_count: u32,    // 448e68
    server_entrance_count: u32, // 448e6c
    // 0 if existing, 2 if new. I can see from the disassembly it can also be 1, which the game seems to treat as 2,
    // but that's based on a byte set on the individual character data that I couldn't find where to set.
    selected_char_status: u32,      // 448e70
    user_rights: u32,               // 448e74 // Missing in F5
    selected_char_hr: u32,          // 448e78
    selected_char_name: [u8; 0x10], // 448e7c

    global_alloc: HGLOBAL,        // 448ecc
    fixed_448ed0_0x1: u32,        // 448ed0
    unk_448ed4: u32,              // 448ed4
    selected_char_gr: u32,        // 448ed8 // can be value before 'bool + name' if bool is false
    _pad_448edc: [u8; 0x8],       //448edc
    preset_level: u32,            // 448ee4
    custom: u32,                  // 448ee8
    fullscreen_mode: u32,         // 448eec
    window_resolution_w: u32,     // 448ef0
    window_resolution_h: u32,     // 448ef4
    fullscreen_resolution_w: u32, // 448ef8
    fullscreen_resolution_h: u32, // 448efc
    disp_max_char: u32,           // 448f00
    texture_dxt_use: u32,         // 448f04
    now_monitor_wh: u32,          // 448f08
    sound_notuse: u32,            // 448f10
    sound_volume: u32,            // 448f14
    sound_volume_inactivity: u32, // 448f18
    sound_volume_minimize: u32,   // 448f1c
    sound_frequency: u32,         // 448f20
    sound_buffernum: u32,         // 448f24
    language: u32,                // 448f28 -- 0x0
    font_quality: u32,            // 448f2c -- 0x4
    font_weight: u32,             // 448f30 -- 0x2bc
    font_name: [u8; 0x60],        // 448f34 -- [0x3f20534d, 0x3f3f3f, 0x3f3f3f] is the default
    unk_setting_448f94: u32,      // 448f94
    drawskip: u32,                // 448f9c -- 0x1
    clogdis: u32,                 // 448fa0 -- 0x0
    proxy_use: u32,               // 448fa4
    proxy_ie: u32,                // 448fa8
    proxy_set: u32,               // 448fac
    proxy_addr: [u8; 0x40],       // 448fb0
    proxy_port: u32,              // 448ff0
    server_sel: u32,              // 448ff4
    inner_ptr_1_4491a8: usize,    // 448ff8
    _pad_448ffc: [u8; 0x40],      // 448ffc

    _pad_4406cc: [u8; 0xc],

    data_ptr: usize,                      // 449190
    keyboard_layout: HKL,                 // 449194
    inner_3: (),                          // 449198
    _pad_449198: [u8; 0x10],              // 449198
    inner_1: (),                          // 4491a8
    _pad_4491a8: [u8; 0x4],               // 4491a8
    fixed_4491ac_0x10: u32,               // 4491ac
    inner_ptr_2_4491d4: usize,            // 4491b0
    _pad_4491b4: [u8; 4],                 // 4491b4
    fixed_4491b8_0x10: u32,               // 4491b8
    inner_ptr_3_449198: usize,            // 4491bc
    proc_4: usize,                        // 4491c0 // fixed 40605e
    _pad_4491c4: [u8; 0x4],               // 4491c4
    proc_5: usize,                        // 4491c8 // fixed 40609c
    _pad_4491cc: [u8; 0x8],               // 4491cc
    inner_2: (),                          // 4491d4
    _pad_4491d4: [u8; 0x14],              // 4491d4
    mhfo_module: HMODULE,                 // 4491e8
    _pad_4491ec: [u8; 0x4],               // 4491ec
    _pad_4491f0: [u8; 0x520],             // 4491f0
    mutex_master_ready_name: [u8; 0x100], // 449710
    _pad_449810: [u8; 0x414],             // 449810
    mhddl_main: FARPROC,                  // 449c24
}

#[derive(Debug)]
#[repr(C)]
struct DataZZ {
    main_module: HMODULE,   // 447178
    _pad_44717c: [u8; 0x8], // 44717c
    cmd_flags_1: u32,       // 447184 // set based on CLI flags
    cmd_flags_2: u32,       // 447188 // set based on CLI flags

    path1: [u8; 0x400],         // 44718c
    path2: [u8; 0x400],         // 44758c
    user_name: [u8; 0x800],     // 44798c
    user_password: [u8; 0x800], // 44818c

    cmd_number: u32,               // 44898c // CLI argument stuff
    cmd_netfcup: u32,              // 448990 // CLI '-NETFCUP'
    cmd_dmm: u32,                  // 448994 // set whenever a /DMM_<X> flag is specified
    _pad_448998: [u8; 0x4],        // 448998 // set when mutex_master is already set?
    mutex_master: HANDLE,          // 44899c
    mutex_master_ready: HANDLE,    // 4489a0
    mutex_master_name: [u8; 0x40], // 4489a4
    ini_file: [u8; 0x40],          // 4489e4
    proc_1: usize,                 // 448a24 // copied from 40187a
    proc_2: usize,                 // 448a28 // copied from 401868, probably gg_proc
    proc_3: usize,                 // 448a2c // copied from 40188b
    _pad_448a30: [u8; 0xc],        // 448a30

    // Server data
    selected_char_id_1: u32,    // 448a3c
    selected_char_id_2: u32,    // 448a40
    user_token_id: u32,         // 448a44
    user_token: [u8; 0x10],     // 448a48
    _pad_448a58: [u8; 0x8],     // 448a58
    server_current_ts: u32,     // 448a60
    fixed_448a64_0x0: u32,      // 448a64
    _pad_448a68: [u8; 0x200],   // 448a68
    remote_addr: [u8; 0x100],   // 448c68
    remote_host: [u8; 0x100],   // 448d68
    remote_patch_count: u32,    // 448e68
    server_entrance_count: u32, // 448e6c
    // 0 if existing, 2 if new. I can see from the disassembly it can also be 1, which the game seems to treat as 2,
    // but that's based on a byte set on the individual character data that I couldn't find where to set.
    selected_char_status: u32,      // 448e70
    user_rights: u32,               // 448e74
    selected_char_hr: u32,          // 448e78
    selected_char_name: [u8; 0x10], // 448e7c
    char_ids: [u32; 0x10],          // 448e8c

    global_alloc: HGLOBAL,  // 448ecc
    fixed_448ed0_0x1: u32,  // 448ed0
    unk_448ed4: u32,        // 448ed4
    selected_char_gr: u32,  // 448ed8 // can be value before 'bool + name' if bool is false
    _pad_448edc: [u8; 0x8], //448edc

    // Config
    preset_level: u32,            // 448ee4
    custom: u32,                  // 448ee8
    fullscreen_mode: u32,         // 448eec
    window_resolution_w: u32,     // 448ef0
    window_resolution_h: u32,     // 448ef4
    fullscreen_resolution_w: u32, // 448ef8
    fullscreen_resolution_h: u32, // 448efc
    disp_max_char: u32,           // 448f00
    texture_dxt_use: u32,         // 448f04
    now_monitor_wh: u32,          // 448f08
    graphics_ver: u32,            // 448f0c
    sound_notuse: u32,            // 448f10
    sound_volume: u32,            // 448f14
    sound_volume_inactivity: u32, // 448f18
    sound_volume_minimize: u32,   // 448f1c
    sound_frequency: u32,         // 448f20
    sound_buffernum: u32,         // 448f24
    language: u32,                // 448f28 -- 0x0
    font_quality: u32,            // 448f2c -- 0x4
    font_weight: u32,             // 448f30 -- 0x2bc
    font_name: [u8; 0x68],        // 448f34 -- [0x3f20534d, 0x3f3f3f, 0x3f3f3f] is the default
    drawskip: u32,                // 448f9c -- 0x1
    clogdis: u32,                 // 448fa0 -- 0x0
    proxy_use: u32,               // 448fa4
    proxy_ie: u32,                // 448fa8
    proxy_set: u32,               // 448fac
    proxy_addr: [u8; 0x40],       // 448fb0
    proxy_port: u32,              // 448ff0
    server_sel: u32,              // 448ff4

    inner_ptr_1_4491a8: usize,            // 448ff8
    _pad_448ffc: [u8; 0x40],              // 448ffc
    _pad_44903c: [u8; 0x40], // 44903c // the 'alt_ip_address' load happens here, with 0x100 width
    alt_ip_address: [u8; 0xC0], // 44907c
    _pad_44913c: [u8; 0x40], // 44913c
    server_expiry_ts: u32,   // 44917c
    remote_16e: u32,         // 449180
    fixed_449184_0x1: u32,   // 449184 // 2 if 100812B0 == 9
    _pad_449188: [u8; 0x8],  // 449188
    data_ptr: usize,         // 449190
    keyboard_layout: HKL,    // 449194
    inner_3: (),             // 449198
    _pad_449198: [u8; 0x10], // 449198
    inner_1: (),             // 4491a8
    _pad_4491a8: [u8; 0x4],  // 4491a8
    fixed_4491ac_0x10: u32,  // 4491ac
    inner_ptr_2_4491d4: usize, // 4491b0
    _pad_4491b4: [u8; 4],    // 4491b4
    fixed_4491b8_0x10: u32,  // 4491b8
    inner_ptr_3_449198: usize, // 4491bc
    proc_4: usize,           // 4491c0 // fixed 40605e
    _pad_4491c4: [u8; 0x4],  // 4491c4
    proc_5: usize,           // 4491c8 // fixed 40609c
    _pad_4491cc: [u8; 0x8],  // 4491cc
    inner_2: (),             // 4491d4
    _pad_4491d4: [u8; 0x14], // 4491d4
    mhfo_module: HMODULE,    // 4491e8
    _pad_4491ec: [u8; 0x4],  // 4491ec
    _pad_4491f0: [u8; 0x520], // 4491f0
    mutex_master_ready_name: [u8; 0x100], // 449710
    _pad_449810: [u8; 0x414], // 449810
    mhddl_main: FARPROC,     // 449c24
} // 449188

#[repr(C)]
struct GlobalData {
    _pad_0x0000: [u8; 0xa00],     // 0000
    _pad_0x0a00: [u8; 0xc],       // 0a00
    notices_count: [u32; 0x4],    // 0a0c
    _pad_0x0a10: [u8; 0x8],       // 0a1c
    notices_flags: [u16; 0x4],    // 0a24
    notices: [[u8; 0x1000]; 0x4], // 0a2c
    _filter: [u8; 0x3000],        // 4a2c
    _pad_0x4a2c: [u8; 0x1080],    // 7a2c
    mez_event_id: u32,            // 8aac
    mez_start: u32,               // 8ab0
    mez_end: u32,                 // 8ab4
    mez_solo_tickets: u32,        // 8ab8
    mez_group_tickets: u32,       // 8abc
    mez_stalls: [u32; 0x8],       // 8ac0
}

// TODO: this might be needed in the future
// struct DataStatic(*const Data);
// unsafe impl Sync for DataStatic {}
// static DATA: SyncUnsafeCell<DataStatic> = SyncUnsafeCell::new(DataStatic(0 as *const Data));

fn init_global_alloc(global_alloc: HGLOBAL, mhf_config: &MhfConfig) {
    let global_ptr = unsafe { GlobalLock(global_alloc) };
    unsafe { global_ptr.write_bytes(0, 0x8ae0) };
    {
        let global_data = unsafe { &mut *(global_ptr as *mut GlobalData) };
        for (i, notice) in mhf_config.notices.iter().enumerate() {
            global_data.notices_count[i] = notice.data.len() as u32;
            global_data.notices_flags[i] = notice.flags;
            bufcopy(&mut global_data.notices[i], notice.data.as_bytes());
        }
        global_data.mez_event_id = mhf_config.mez_event_id;
        global_data.mez_start = mhf_config.mez_start;
        global_data.mez_end = mhf_config.mez_end;
        global_data.mez_solo_tickets = mhf_config.mez_solo_tickets;
        global_data.mez_group_tickets = mhf_config.mez_group_tickets;
        for (i, stall) in mhf_config.mez_stalls.iter().enumerate() {
            global_data.mez_stalls[i] = *stall as u32;
        }
    }
    unsafe { GlobalUnlock(global_alloc) }
        .or_else(|e| match e.code().0 {
            0 => Ok(()),
            _ => Err(e),
        })
        .unwrap();
}

#[derive(Default)]
struct CmdData {
    cmd_flags_1: u32,
    cmd_flags_2: u32,
    cmd_dmm: u32,
}

fn init_cli(mhf_flags: &[CliFlags]) -> CmdData {
    let mut cmd_data = CmdData::default();
    for flag in mhf_flags {
        match flag {
            CliFlags::Selfup => cmd_data.cmd_flags_1 = 1,
            CliFlags::Restat => cmd_data.cmd_flags_1 = 2,
            CliFlags::Autolc => cmd_data.cmd_flags_1 = 3,
            CliFlags::Hanres => cmd_data.cmd_flags_1 = 4,
            CliFlags::DmmBoot => {
                cmd_data.cmd_flags_1 = 5;
                cmd_data.cmd_dmm = 1;
            }
            CliFlags::DmmSelfup => {
                cmd_data.cmd_flags_1 = 6;
                cmd_data.cmd_dmm = 1;
            }
            CliFlags::DmmAutolc => {
                cmd_data.cmd_flags_1 = 7;
                cmd_data.cmd_dmm = 1;
            }
            CliFlags::DmmReboot => {
                cmd_data.cmd_flags_1 = 8;
                cmd_data.cmd_dmm = 1;
            }
            CliFlags::Npge => {
                cmd_data.cmd_flags_1 = 9;
                cmd_data.cmd_flags_2 |= 6;
            }
            CliFlags::NpMhfoTest => cmd_data.cmd_flags_2 |= 4,
        }
    }
    cmd_data
}

macro_rules! init_data {
    (
        $data:ident,
        $main_module:ident,
        $mutex_master:ident,
        $mutex_master_name:ident,
        $mutex_master_ready:ident,
        $mutex_master_ready_name:ident,
        $global_alloc:ident,
        $keyboard_layout:ident,
        $cmd_data:ident,
        $ini_file:ident,
        $mhf_folder_name:ident,
        $dll_name:ident,
        $config:ident
    ) => {
        $data.main_module = $main_module;
        $data.mutex_master = $mutex_master;
        $data.mutex_master_ready = $mutex_master_ready;
        $data.global_alloc = $global_alloc;
        $data.keyboard_layout = $keyboard_layout;
        $data.fixed_448a64_0x0 = 0x0;
        $data.fixed_448ed0_0x1 = 0x1;
        $data.fixed_4491ac_0x10 = 0x10;
        $data.fixed_4491b8_0x10 = 0x10;
        $data.proc_1 = mock_proc as usize;
        $data.proc_2 = gg_proc as usize;
        $data.proc_3 = mock_proc as usize;
        $data.proc_4 = mock_proc as usize;
        $data.proc_5 = mock_proc as usize;
        unsafe {
            $data.preset_level = GetPrivateProfileIntA(s!("SET"), s!("PRESET_LEVEL"), 0, $ini_file);
            $data.custom = GetPrivateProfileIntA(s!("SET"), s!("CUSTOM"), 1, $ini_file);
            $data.fullscreen_mode =
                GetPrivateProfileIntA(s!("SCREEN"), s!("FULLSCREEN_MODE"), 1, $ini_file);
            $data.window_resolution_w =
                GetPrivateProfileIntA(s!("SCREEN"), s!("WINDOW_RESOLUTION_W"), 1920, $ini_file);
            $data.window_resolution_h =
                GetPrivateProfileIntA(s!("SCREEN"), s!("WINDOW_RESOLUTION_H"), 1080, $ini_file);
            $data.fullscreen_resolution_w =
                GetPrivateProfileIntA(s!("SCREEN"), s!("FULLSCREEN_RESOLUTION_W"), 1920, $ini_file);
            $data.fullscreen_resolution_h =
                GetPrivateProfileIntA(s!("SCREEN"), s!("FULLSCREEN_RESOLUTION_H"), 1080, $ini_file);
            $data.disp_max_char =
                GetPrivateProfileIntA(s!("VIDEO"), s!("DISP_MAX_CHAR"), 100, $ini_file);
            $data.texture_dxt_use =
                GetPrivateProfileIntA(s!("VIDEO"), s!("TEXTURE_DXT_USE"), 0, $ini_file);
            $data.now_monitor_wh =
                GetPrivateProfileIntA(s!("VIDEO"), s!("NOW_MONITOR_WH"), 0, $ini_file);
            $data.sound_notuse =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_NOTUSE"), 0, $ini_file);
            $data.sound_volume =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_VOLUME"), 0, $ini_file);
            $data.sound_volume_inactivity =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_VOLUME_INACTIVITY"), 0, $ini_file);
            $data.sound_volume_minimize =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_VOLUME_MINIMIZE"), 0, $ini_file);
            $data.sound_frequency =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_FREQUENCY"), 48000, $ini_file);
            $data.sound_buffernum =
                GetPrivateProfileIntA(s!("SOUND"), s!("SOUND_BUFFERNUM"), 2048, $ini_file);
            $data.language =
                GetPrivateProfileIntA(s!("LOCALIZATION"), s!("LANGUAGE"), 0, $ini_file);
            $data.font_quality = GetPrivateProfileIntA(s!("FONT"), s!("QUALITY"), 4, $ini_file);
            $data.font_weight = GetPrivateProfileIntA(s!("FONT"), s!("WEIGHT"), 0x2bc, $ini_file);
            GetPrivateProfileStringA(
                s!("FONT"),
                s!("NAME"),
                s!("MS ????"),
                Some(&mut $data.font_name),
                $ini_file,
            );
            $data.drawskip = GetPrivateProfileIntA(s!("OPTION"), s!("DRAWSKIP"), 1, $ini_file);
            $data.clogdis = GetPrivateProfileIntA(s!("OPTION"), s!("CLOGDIS"), 0, $ini_file);
            $data.proxy_use = GetPrivateProfileIntA(s!("LAUNCH"), s!("PROXY_USE"), 0, $ini_file);
            $data.proxy_ie = GetPrivateProfileIntA(s!("LAUNCH"), s!("PROXY_IE"), 0, $ini_file);
            $data.proxy_set = GetPrivateProfileIntA(s!("LAUNCH"), s!("PROXY_SET"), 1, $ini_file);
            GetPrivateProfileStringA(
                s!("LAUNCH"),
                s!("PROXY_ADDR"),
                s!("127.0.0.1"),
                Some(&mut $data.proxy_addr),
                $ini_file,
            );
            $data.proxy_port =
                GetPrivateProfileIntA(s!("LAUNCH"), s!("PROXY_PORT"), 8888, $ini_file);
            $data.server_sel = GetPrivateProfileIntA(s!("LAUNCH"), s!("SERVER_SEL"), 1, $ini_file);
        };
        init_global_alloc($data.global_alloc, &$config);

        // Char
        $data.selected_char_id_1 = $config.char_id;
        $data.selected_char_id_2 = $config.char_id;
        bufcopy(&mut $data.selected_char_name, $config.char_name.as_bytes());
        $data.selected_char_hr = $config.char_hr;
        $data.selected_char_gr = $config.char_gr;
        $data.selected_char_status = if $config.char_new { 2 } else { 0 };

        // User
        bufcopy(&mut $data.user_name, $config.user_name.as_bytes());
        bufcopy(&mut $data.user_password, $config.user_password.as_bytes());
        $data.user_token_id = $config.user_token_id;
        bufcopy(&mut $data.user_token, $config.user_token.as_bytes());
        $data.user_rights = $config.user_rights;

        // Server
        $data.server_entrance_count = $config.entrance_count;
        $data.server_current_ts = $config.current_ts;

        // Meta
        $data.cmd_flags_1 = $cmd_data.cmd_flags_1;
        $data.cmd_flags_2 = $cmd_data.cmd_flags_2;
        $data.cmd_dmm = $cmd_data.cmd_dmm;
        bufcopy(&mut $data.mutex_master_name, $mutex_master_name.as_bytes());
        bufcopy(
            &mut $data.mutex_master_ready_name,
            $mutex_master_ready_name.as_bytes(),
        );
        bufcopy(&mut $data.path1, $mhf_folder_name.as_bytes());
        bufcopy(&mut $data.path2, $mhf_folder_name.as_bytes());
        bufcopy(&mut $data.ini_file, b"mhf.ini");
        bufcopy(
            &mut $data.remote_addr,
            format!("{}:{}", $config.server_host, $config.server_port).as_bytes(),
        );
        bufcopy(&mut $data.remote_host, $config.server_host.as_bytes());

        $data.mhfo_module = unsafe { LoadLibraryA($dll_name) }.or(Err(Error::Dll))?;
        $data.mhddl_main = unsafe { GetProcAddress($data.mhfo_module, s!("mhDLL_Main")) };
    };
}

macro_rules! init_ptrs {
    ($data:ident) => {
        $data.data_ptr = Box::as_ref(&$data) as *const _ as usize;
        $data.inner_ptr_1_4491a8 = &$data.inner_1 as *const _ as usize;
        $data.inner_ptr_2_4491d4 = &$data.inner_2 as *const _ as usize;
        $data.inner_ptr_3_449198 = &$data.inner_3 as *const _ as usize;
    };
}

pub fn run_mhf(config: crate::MhfConfig) -> Result<isize> {
    let mhf_folder = match &config.mhf_folder {
        Some(mhf_folder) => {
            std::env::set_current_dir(mhf_folder).or(Err(Error::GamePath))?;
            mhf_folder.clone()
        }
        None => std::env::current_dir().or(Err(Error::GamePath))?,
    };
    let mut mhf_folder_name = mhf_folder.to_str().ok_or(Error::GamePath)?.to_owned();
    if !mhf_folder_name.ends_with(&['/', '\\']) {
        mhf_folder_name.push('/');
    }

    // Init
    let main_module = unsafe { GetModuleHandleA(None).unwrap() };
    let keyboard_layout = unsafe { GetKeyboardLayout(0) };
    let mutex_master_name = utils::get_mutex_name("MHF_MASTER");
    let mutex_master = utils::get_or_create_mutex(&mutex_master_name)?;
    let mutex_master_ready_name = utils::get_mutex_name("MHF_MASTER_READY");
    let mutex_master_ready = utils::get_or_create_mutex(&mutex_master_ready_name)?;
    let global_alloc = utils::create_global_alloc()?;
    let cmd_data = if let Some(flags) = config.mhf_flags.as_ref() {
        init_cli(flags)
    } else {
        CmdData::default()
    };
    let ini_file = s!("./mhf.ini");

    let (data_ptr, proc, mhfo_module) = match config.version {
        MhfVersion::F5 => {
            let mut data = unsafe { Box::<DataF5>::new_zeroed().assume_init() };
            let dll_name = s!("mhfo.dll");
            // Required in order to skip initial GG check
            std::env::set_var("JKR", "1");
            init_data!(
                data,
                main_module,
                mutex_master,
                mutex_master_name,
                mutex_master_ready,
                mutex_master_ready_name,
                global_alloc,
                keyboard_layout,
                cmd_data,
                ini_file,
                mhf_folder_name,
                dll_name,
                config
            );

            let mhdll_main = data.mhddl_main;
            let mhfo_module = data.mhfo_module;
            init_ptrs!(data);
            (
                Box::into_raw(data) as *mut usize,
                mhdll_main.ok_or(Error::ProcNotFound)?,
                mhfo_module,
            )
        }
        MhfVersion::ZZ => {
            let mut data = unsafe { Box::<DataZZ>::new_zeroed().assume_init() };
            let graphics_ver =
                unsafe { GetPrivateProfileIntA(s!("VIDEO"), s!("GRAPHICS_VER"), 1, ini_file) };
            let dll_name = if graphics_ver == 1 {
                s!("mhfo-hd.dll")
            } else {
                s!("mhfo.dll")
            };
            init_data!(
                data,
                main_module,
                mutex_master,
                mutex_master_name,
                mutex_master_ready,
                mutex_master_ready_name,
                global_alloc,
                keyboard_layout,
                cmd_data,
                ini_file,
                mhf_folder_name,
                dll_name,
                config
            );

            // ZZ specific
            data.graphics_ver = graphics_ver;
            bufcopy(&mut data.char_ids, &config.char_ids);
            bufcopy(
                &mut data.alt_ip_address,
                format!("{}:8080", config.server_host).as_bytes(),
            );
            data.server_expiry_ts = config.expiry_ts;
            data.fixed_449184_0x1 = 0x1;

            let mhdll_main = data.mhddl_main;
            let mhfo_module = data.mhfo_module;
            init_ptrs!(data);
            (
                Box::into_raw(data) as *mut usize,
                mhdll_main.ok_or(Error::ProcNotFound)?,
                mhfo_module,
            )
        }
    };

    let proc: unsafe extern "C" fn(*const usize) -> isize = unsafe { std::mem::transmute(proc) };

    let result = unsafe { proc(data_ptr) };

    unsafe { FreeLibrary(mhfo_module) }.or(Err(Error::Dll))?;
    utils::release_global_alloc(global_alloc)?;

    // Ensure pointer is freed
    match config.version {
        MhfVersion::F5 => drop(unsafe { Box::from_raw(data_ptr as *mut DataF5) }),
        MhfVersion::ZZ => drop(unsafe { Box::from_raw(data_ptr as *mut DataZZ) }),
    }

    Ok(result)
}
