//! The machine, the session and the user, on Windows.
//!
//! The runtime does no I/O of its own: the names of the machine -- the
//! user, the computer, the shell, the folders, the version of the
//! operating system, the language, the rights of the script, the registry
//! and the idle times -- are asked of the desktop, and the Windows
//! machine is the desktop.
//!
//! The answers are read where Windows keeps them: the account of the
//! user, the name of the computer, the known folders of the shell, the
//! registry of `advapi32`, and the drives of the kernel.
//!
//! The failure of a call is the text of the system, with the number of
//! the failure in front of it -- "(2) The system cannot find the file
//! specified." is what the machine says when the key of the registry is
//! not there.

use ahk::desktop::{Drive, DriveSpace, Folders, Idle};

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_FILE_NOT_FOUND, ERROR_NO_MORE_ITEMS, FILETIME, GetLastError, HANDLE,
    SetLastError, SYSTEMTIME,
};
use windows_sys::Win32::Globalization::{GetLocaleInfoW, LOCALE_SNAME, LOCALE_USER_DEFAULT};
use windows_sys::Win32::Security::{
    GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
};
use windows_sys::Win32::Storage::FileSystem::{
    GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDrives, GetTempPathW, GetVolumeInformationW,
    SetVolumeLabelW,
};
use windows_sys::Win32::System::Diagnostics::Debug::{
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW, OutputDebugStringW,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows_sys::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    HKEY_USERS, KEY_READ, KEY_WRITE, REG_BINARY, REG_DWORD, REG_EXPAND_SZ, REG_MULTI_SZ, REG_QWORD,
    REG_SZ, RegCloseKey, RegCreateKeyExW, RegDeleteKeyW, RegDeleteValueW, RegEnumKeyExW,
    RegEnumValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
};
use windows_sys::Win32::System::SystemInformation::{
    GetTickCount, GetWindowsDirectoryW, OSVERSIONINFOW,
};
use windows_sys::Win32::System::Threading::{
    GetCurrentProcess, GetCurrentThreadId, OpenProcessToken,
};
use windows_sys::Win32::System::WindowsProgramming::{GetComputerNameW, GetUserNameW};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows_sys::Win32::UI::Shell::{
    FOLDERID_CommonPrograms, FOLDERID_CommonStartup, FOLDERID_Documents, FOLDERID_ProgramFiles,
    FOLDERID_Programs, FOLDERID_PublicDesktop, FOLDERID_RoamingAppData, FOLDERID_StartMenu,
    FOLDERID_Startup,
};

use super::sys::{Wide, the_last_error, the_message_of};

// The version of the machine, as the kernel of it says it.
//
// `RtlGetVersion` is the function of `ntdll` that the shim of the
// manifest does not touch: `GetVersionExW` answers the version of
// Windows 6.2 to a program that has no manifest, and this one answers
// the version of the machine.
#[link(name = "ntdll")]
unsafe extern "system" {
    // The status of the call is zero when it succeeded: `STATUS_SUCCESS`
    // is the zero of the status of the kernel.
    fn RtlGetVersion(version: *mut OSVERSIONINFOW) -> i32;
}

/// The name of the user that launched the session.
///
/// The account of the user is the one that the token of the process
/// carries: `GetUserNameW` is of Windows 2000.
pub fn user_name() -> Result<String, String> {
    let mut buffer = [0u16; 256];
    let mut length = buffer.len() as u32;
    let length = unsafe { GetUserNameW(buffer.as_mut_ptr(), &mut length) };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(String::from_utf16_lossy(&buffer[..end]))
}

/// The name of the computer, as the network sees it.
///
/// `GetComputerNameW` is of Windows 2000, and the name it answers is the
/// name of the NetBIOS of the machine.
pub fn computer_name() -> Result<String, String> {
    let mut buffer = [0u16; 256];
    let mut length = buffer.len() as u32;
    let length = unsafe { GetComputerNameW(buffer.as_mut_ptr(), &mut length) };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(String::from_utf16_lossy(&buffer[..end]))
}

/// The program that runs a command of the session.
///
/// v2 calls it `A_ComSpec`, and the machine carries it in the
/// environment of the session: `cmd.exe` on a machine that has it, and
/// the command processor of the session otherwise.
pub fn shell() -> Result<String, String> {
    match std::env::var("COMSPEC") {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Ok("cmd.exe".to_string()),
    }
}

/// The path of the executable that is running the script.
///
/// It is what `A_AhkPath` answers. The path of the binary of the runner
/// is the path of the process, and the process reads it out of itself.
pub fn executable() -> Result<String, String> {
    let mut buffer = [0u16; 32768];
    let length = unsafe {
        GetModuleFileNameW(
            std::ptr::null_mut(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
        )
    };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(String::from_utf16_lossy(&buffer[..end]))
}

/// The folder of the temporary files of the user.
///
/// `GetTempPathW` is of Windows 2000, and it answers the folder where the
/// temporary files of the user are made.
fn the_temp_folder() -> Result<Option<String>, String> {
    let mut buffer = [0u16; 32768];
    let length = unsafe { GetTempPathW(buffer.len() as u32, buffer.as_mut_ptr()) };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(Some(String::from_utf16_lossy(&buffer[..end])))
}

/// The version of the operating system.
///
/// `GetVersionExW` is of Windows 2000, and it is shimmed by the manifest
/// of the program: a program that has no manifest is told the version of
/// Windows 6.2 whatever the machine runs. The tree reads the version of
/// the machine and not the version of the manifest, and the reader of it
/// is `RtlGetVersion` -- the function of `ntdll` that the shim does not
/// touch.
pub fn os_version() -> Result<String, String> {
    let mut info: OSVERSIONINFOW = unsafe { std::mem::zeroed() };
    info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;
    // The status of the call is zero when it succeeded: `STATUS_SUCCESS`
    // is the zero of the status of the kernel.
    let status = unsafe { RtlGetVersion(&mut info) };
    if status != 0 {
        return Err(format!(
            "the machine did not say the version of the system: {status}"
        ));
    }
    Ok(format!(
        "{}.{}.{}",
        info.dwMajorVersion, info.dwMinorVersion, info.dwBuildNumber
    ))
}

/// The language of the session.
///
/// `GetUserDefaultLocaleName` is of Windows Vista, and it answers the
/// name of the locale of the user -- "pt-BR" on the machine of the tree.
pub fn language() -> Result<String, String> {
    let mut buffer = [0u16; 85];
    let length = buffer.len() as i32;
    // `GetUserDefaultLocaleName` e de Windows Vista: o XP da o nome
    // da lingua com o `GetLocaleInfoW`, que e de Windows 2000.
    let length = unsafe { GetLocaleInfoW(LOCALE_USER_DEFAULT, LOCALE_SNAME, buffer.as_mut_ptr(), length) };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(String::from_utf16_lossy(&buffer[..end]))
}

/// Whether the script has the rights of an administrator.
///
/// The token of the process is the one that says it: `TokenElevation` is
/// of Windows Vista, and it answers the height of the token.
pub fn is_admin() -> Result<bool, String> {
    let mut token: HANDLE = std::ptr::null_mut();
    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return Err(the_last_error());
        }
        let mut elevation: TOKEN_ELEVATION = std::mem::zeroed();
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let answer = GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut core::ffi::c_void,
            size,
            &mut size,
        );
        CloseHandle(token);
        if answer == 0 {
            return Err(the_last_error());
        }
        Ok(elevation.TokenIsElevated != 0)
    }
}

/// The folder of the machine, and the reason when it is not there.
///
/// The known folders of the shell are of Windows Vista: `SHGetKnownFolderPath`
/// is the reader of them, and the memory of the answer is the memory of
/// the shell, which is freed with `CoTaskMemFree`.
fn the_folder_of(guid: &windows_sys::core::GUID) -> Result<Option<String>, String> {
    // As pastas conhecidas do shell sao de Windows Vista, e a funcao que
    // as le nao pode ser importada: o XP nao a tem. O nome dela e
    // perguntado a maquina, e a pasta classica do XP e a resposta.
    let Some(symbol) = super::sys::the_symbol_of("shell32.dll", "SHGetKnownFolderPath") else {
        return Ok(the_folder_of_the_xp(guid));
    };
    type Reader = unsafe extern "system" fn(
        *const windows_sys::core::GUID,
        u32,
        *mut core::ffi::c_void,
        *mut *mut u16,
    ) -> windows_sys::core::HRESULT;
    let reader: Reader = unsafe { std::mem::transmute(symbol) };
    let mut path: *mut u16 = std::ptr::null_mut();
    let answer = unsafe { reader(guid, 0, std::ptr::null_mut(), &mut path) };
    if answer < 0 {
        return Err(format!(
            "the folder of the machine is not there: {}",
            the_message_of(answer as u32)
        ));
    }
    let text = unsafe {
        let end = (0..)
            .find(|index| *path.add(*index) == 0)
            .unwrap_or_default();
        let text = String::from_utf16_lossy(std::slice::from_raw_parts(path, end));
        windows_sys::Win32::System::Com::CoTaskMemFree(path.cast());
        text
    };
    Ok(Some(text))
}

/// The folder of a known folder, on a machine of Windows XP.
///
/// WHY: the known folders are of Windows Vista. The XP has the folders
/// of the classic age: the ones of the user, and the ones of everybody,
/// and the names of them are the names of the environment.
fn the_folder_of_the_xp(guid: &windows_sys::core::GUID) -> Option<String> {
    use windows_sys::Win32::UI::Shell::{
        FOLDERID_CommonPrograms, FOLDERID_CommonStartMenu, FOLDERID_CommonStartup,
        FOLDERID_Documents, FOLDERID_Desktop, FOLDERID_Programs, FOLDERID_PublicDesktop,
        FOLDERID_RoamingAppData, FOLDERID_Startup, FOLDERID_StartMenu,
    };
    let variable = |name: &str| std::env::var(name).ok();
    let user = variable("USERPROFILE");
    let all = variable("ALLUSERSPROFILE");
    let app = variable("APPDATA");
    let same = |other: &windows_sys::core::GUID| {
        guid.data1 == other.data1
            && guid.data2 == other.data2
            && guid.data3 == other.data3
            && guid.data4 == other.data4
    };
    let folder = if same(&FOLDERID_RoamingAppData) {
        app?
    } else if same(&FOLDERID_Desktop) {
        format!("{}\\Desktop", user?)
    } else if same(&FOLDERID_PublicDesktop) {
        format!("{}\\Desktop", all?)
    } else if same(&FOLDERID_StartMenu) {
        format!("{}\\Microsoft\\Windows\\Start Menu", app?)
    } else if same(&FOLDERID_CommonStartMenu) {
        format!("{}\\Microsoft\\Windows\\Start Menu", all?)
    } else if same(&FOLDERID_Programs) {
        format!("{}\\Microsoft\\Windows\\Start Menu\\Programs", app?)
    } else if same(&FOLDERID_CommonPrograms) {
        format!("{}\\Microsoft\\Windows\\Start Menu\\Programs", all?)
    } else if same(&FOLDERID_Startup) {
        format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", app?)
    } else if same(&FOLDERID_CommonStartup) {
        format!("{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup", all?)
    } else if same(&FOLDERID_Documents) {
        format!("{}\\My Documents", user?)
    } else {
        return None;
    };
    Some(folder)
}

/// The folders of the user and of everybody.
///
/// The names of the v2 are the names of the known folders of the shell:
/// `A_AppData`, `A_Desktop`, `A_StartMenu`, `A_Programs`, `A_Startup`,
/// `A_MyDocuments` and `A_Temp`.
pub fn folders() -> Result<Folders, String> {
    Ok(Folders {
        app_data: the_folder_of(&FOLDERID_RoamingAppData)?,
        app_data_common: None,
        desktop: the_folder_of(&windows_sys::Win32::UI::Shell::FOLDERID_Desktop)?,
        desktop_common: the_folder_of(&FOLDERID_PublicDesktop)?,
        start_menu: the_folder_of(&FOLDERID_StartMenu)?,
        start_menu_common: the_folder_of(&windows_sys::Win32::UI::Shell::FOLDERID_CommonStartMenu)?,
        programs: the_folder_of(&FOLDERID_Programs)?,
        programs_common: the_folder_of(&FOLDERID_CommonPrograms)?,
        startup: the_folder_of(&FOLDERID_Startup)?,
        startup_common: the_folder_of(&FOLDERID_CommonStartup)?,
        my_documents: the_folder_of(&FOLDERID_Documents)?,
        temp: the_temp_folder()?,
    })
}

/// The folder of the Windows itself.
///
/// `GetWindowsDirectoryW` is of Windows 2000, and it answers the folder
/// where the system is installed.
pub fn windows_directory() -> Result<String, String> {
    let mut buffer = [0u16; 32768];
    let length = unsafe { GetWindowsDirectoryW(buffer.as_mut_ptr(), buffer.len() as u32) };
    if length == 0 {
        return Err(the_last_error());
    }
    let end = buffer
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(buffer.len());
    Ok(String::from_utf16_lossy(&buffer[..end]))
}

/// The folder of the programs of the machine.
///
/// It is the known folder of the shell: `FOLDERID_ProgramFiles` is of
/// Windows Vista.
pub fn program_files() -> Result<String, String> {
    the_folder_of(&FOLDERID_ProgramFiles)?
        .ok_or_else(|| "the folder of the programs is not there.".to_string())
}

/// The kind of a drive, in the word of the v2.
///
/// `GetDriveTypeW` is of Windows 2000, and the words of the machine are
/// the words of the v2: "Fixed", "Removable", "Network", "CDROM" and
/// "Unknown".
pub fn drive_type(path: &str) -> Result<Option<String>, String> {
    let root = the_root_of(path);
    let kind = unsafe { GetDriveTypeW(Wide::of(&root).as_ptr()) };
    Ok(Some(
        match kind {
            2 => "Removable",
            3 => "Fixed",
            4 => "Network",
            5 => "CDROM",
            6 => "RAMDisk",
            _ => "Unknown",
        }
        .to_string(),
    ))
}

/// The root of a drive: the letter of it and the separator.
fn the_root_of(path: &str) -> String {
    let path = path.trim();
    if path.len() >= 2 && path.as_bytes()[1] == b':' {
        return format!("{}\\", &path[..2]);
    }
    path.to_string()
}

/// The free space and the capacity of a drive.
///
/// `GetDiskFreeSpaceExW` is of Windows 2000, and it answers the space of
/// the drive for the caller of it.
pub fn drive_space(path: &str) -> Result<Option<DriveSpace>, String> {
    let root = the_root_of(path);
    let mut free = 0u64;
    let mut total = 0u64;
    let mut total_free = 0u64;
    let answer = unsafe {
        GetDiskFreeSpaceExW(
            Wide::of(&root).as_ptr(),
            &mut free,
            &mut total,
            &mut total_free,
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    let (filesystem, kind) = the_volume_of(&root);
    Ok(Some(DriveSpace {
        free: (free / (1024 * 1024)) as i64,
        capacity: (total / (1024 * 1024)) as i64,
        filesystem,
        kind,
    }))
}

/// The name of the format of a volume, and the kind of the drive.
fn the_volume_of(root: &str) -> (String, String) {
    let mut name = [0u16; 261];
    let mut serial = 0u32;
    let mut maximum = 0u32;
    let mut flags = 0u32;
    let mut filesystem = [0u16; 261];
    let answer = unsafe {
        GetVolumeInformationW(
            Wide::of(root).as_ptr(),
            name.as_mut_ptr(),
            name.len() as u32,
            &mut serial,
            &mut maximum,
            &mut flags,
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    let kind = match unsafe { GetDriveTypeW(Wide::of(root).as_ptr()) } {
        2 => "Removable",
        3 => "Fixed",
        4 => "Network",
        5 => "CDROM",
        6 => "RAMDisk",
        _ => "Unknown",
    };
    if answer == 0 {
        return (String::new(), kind.to_string());
    }
    let end = filesystem
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(filesystem.len());
    (
        String::from_utf16_lossy(&filesystem[..end]),
        kind.to_string(),
    )
}

/// The label of a volume.
pub fn drive_label(path: &str) -> Result<Option<String>, String> {
    let root = the_root_of(path);
    let mut name = [0u16; 261];
    let mut serial = 0u32;
    let mut maximum = 0u32;
    let mut flags = 0u32;
    let mut filesystem = [0u16; 261];
    let answer = unsafe {
        GetVolumeInformationW(
            Wide::of(&root).as_ptr(),
            name.as_mut_ptr(),
            name.len() as u32,
            &mut serial,
            &mut maximum,
            &mut flags,
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    let end = name
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(name.len());
    Ok(Some(String::from_utf16_lossy(&name[..end])))
}

/// The label of a volume is written, or taken away when it is empty.
///
/// The machine answers with `SetVolumeLabelW` of the root of the drive.
/// The empty label is the null pointer of the text, the way the machine
/// takes a label away.
pub fn drive_set_label(path: &str, label: &str) -> Result<(), String> {
    let root = the_root_of(path);
    let wide = Wide::of(label);
    let answer = unsafe {
        SetVolumeLabelW(
            Wide::of(&root).as_ptr(),
            if label.is_empty() {
                std::ptr::null()
            } else {
                wide.as_ptr()
            },
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    Ok(())
}

/// The number that identifies a volume.
pub fn drive_serial(path: &str) -> Result<Option<String>, String> {
    let root = the_root_of(path);
    let mut name = [0u16; 261];
    let mut serial = 0u32;
    let mut maximum = 0u32;
    let mut flags = 0u32;
    let mut filesystem = [0u16; 261];
    let answer = unsafe {
        GetVolumeInformationW(
            Wide::of(&root).as_ptr(),
            name.as_mut_ptr(),
            name.len() as u32,
            &mut serial,
            &mut maximum,
            &mut flags,
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    Ok(Some(serial.to_string()))
}

/// The drives of the machine.
///
/// `GetLogicalDrives` is of Windows 2000, and it answers the letters of
/// the drives that the machine has.
pub fn drive_list() -> Result<Vec<String>, String> {
    let mask = unsafe { GetLogicalDrives() };
    let mut drives = Vec::new();
    for index in 0..26u32 {
        if mask & (1 << index) == 0 {
            continue;
        }
        let letter = (b'A' + index as u8) as char;
        drives.push(format!("{letter}:"));
    }
    Ok(drives)
}

/// The state of a drive: "Ready" when the machine says it is ready.
pub fn drive_status(path: &str) -> Result<Option<String>, String> {
    let root = the_root_of(path);
    let mut name = [0u16; 261];
    let mut serial = 0u32;
    let mut maximum = 0u32;
    let mut flags = 0u32;
    let mut filesystem = [0u16; 261];
    let answer = unsafe {
        GetVolumeInformationW(
            Wide::of(&root).as_ptr(),
            name.as_mut_ptr(),
            name.len() as u32,
            &mut serial,
            &mut maximum,
            &mut flags,
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    // The bit 1 of the flags is the one that says that the drive is not
    // ready: the machine writes it when the media of it is not there.
    Ok(Some(
        if flags & 0x0000_0002 == 0 {
            "Ready"
        } else {
            "NotReady"
        }
        .to_string(),
    ))
}

/// The state of a drive of a disc.
pub fn drive_status_cd(path: &str) -> Result<Option<String>, String> {
    drive_status(path)
}

/// The drive of the machine, with everything that is known of it.
pub fn drive(path: &str) -> Result<Option<Drive>, String> {
    let root = the_root_of(path);
    let mut name = [0u16; 261];
    let mut serial = 0u32;
    let mut maximum = 0u32;
    let mut flags = 0u32;
    let mut filesystem = [0u16; 261];
    let answer = unsafe {
        GetVolumeInformationW(
            Wide::of(&root).as_ptr(),
            name.as_mut_ptr(),
            name.len() as u32,
            &mut serial,
            &mut maximum,
            &mut flags,
            filesystem.as_mut_ptr(),
            filesystem.len() as u32,
        )
    };
    if answer == 0 {
        return Err(the_last_error());
    }
    let end_of_name = name
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(name.len());
    let end_of_the_format = filesystem
        .iter()
        .position(|letter| *letter == 0)
        .unwrap_or(filesystem.len());
    let mut free = 0u64;
    let mut total = 0u64;
    let mut total_free = 0u64;
    unsafe {
        GetDiskFreeSpaceExW(
            Wide::of(&root).as_ptr(),
            &mut free,
            &mut total,
            &mut total_free,
        )
    };
    let kind = match unsafe { GetDriveTypeW(Wide::of(&root).as_ptr()) } {
        2 => "Removable",
        3 => "Fixed",
        4 => "Network",
        5 => "CDROM",
        6 => "RAMDisk",
        _ => "Unknown",
    };
    Ok(Some(Drive {
        drive_type: kind.to_string(),
        filesystem: String::from_utf16_lossy(&filesystem[..end_of_the_format]),
        label: String::from_utf16_lossy(&name[..end_of_name]),
        serial: serial as i64,
        free_megabytes: (free / (1024 * 1024)) as i64,
        capacity_megabytes: (total / (1024 * 1024)) as i64,
        status: if flags & 0x0000_0002 == 0 {
            "Ready".to_string()
        } else {
            "NotReady".to_string()
        },
    }))
}

/// The idle times of the session, in milliseconds.
///
/// `GetLastInputInfo` is of Windows 2000, and it answers the moment of the
/// last input of the session.
pub fn idle_times() -> Result<Idle, String> {
    let mut info: LASTINPUTINFO = unsafe { std::mem::zeroed() };
    info.cbSize = std::mem::size_of::<LASTINPUTINFO>() as u32;
    if unsafe { GetLastInputInfo(&mut info) } == 0 {
        return Err(the_last_error());
    }
    let now = the_tick_count();
    let any = now.saturating_sub(info.dwTime as u64);
    Ok(Idle {
        any,
        physical: any,
        keyboard: any,
        mouse: any,
    })
}

/// The number of the milliseconds of the machine, since it started.
fn the_tick_count() -> u64 {
    unsafe { GetTickCount() as u64 }
}

/// The code of the last error of the machine.
///
/// It is the code that the `OSError` of the v2 carries when the script
/// does not name it.
pub fn last_error() -> Result<i64, String> {
    Ok(unsafe { GetLastError() } as i64)
}

/// The error of the machine is told.
///
/// It is the error that the script keeps, and the next native call of the
/// script sees it.
pub fn set_last_error(code: i64) -> Result<(), String> {
    unsafe { SetLastError(code as u32) };
    Ok(())
}

/// The text that the machine says about the code of an error.
///
/// The text of the system is what the `OSError` of the v2 carries with the
/// code: "(87) The parameter is incorrect.".
pub fn the_text_of_an_error(code: i64) -> Result<String, String> {
    let mut buffer = [0u16; 512];
    let read = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            std::ptr::null(),
            code as u32,
            0,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            std::ptr::null(),
        )
    };
    if read == 0 {
        return Ok(String::new());
    }
    let mut text = String::from_utf16_lossy(&buffer[..read as usize]);
    // The text of the system ends with the line break that the v2 takes
    // away.
    while text.ends_with('\r') || text.ends_with('\n') {
        text.pop();
    }
    Ok(text)
}

/// The text of the session, for the debugger of the machine.
///
/// `OutputDebugStringW` is of Windows 2000, and it writes the text where
/// the debugger of the machine reads it.
pub fn output_debug(text: &str) -> Result<(), String> {
    unsafe { OutputDebugStringW(Wide::of(text).as_ptr()) };
    Ok(())
}

/// The number of the thread that runs the script.
///
/// `GetCurrentThreadId` is of Windows 2000, and it answers the number of
/// the thread of the caller.
pub fn the_current_thread_id() -> Result<i64, String> {
    Ok(unsafe { GetCurrentThreadId() } as i64)
}

/// The hive of a key of the registry, and the path under it.
///
/// The hive is **required**: measured on the binary of v2.0.19 `RegCreateKey("Software\\X")` is refused with "Parameter
/// #1 of RegCreateKey is invalid.".
fn the_hive_of(key: &str) -> Option<(HKEY, &str)> {
    let (hive, path) = key.split_once('\\')?;
    let hive = match hive.trim().to_uppercase().as_str() {
        "HKCU" | "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKLM" | "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKU" | "HKEY_USERS" => HKEY_USERS,
        "HKCR" | "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKCC" | "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        _ => return None,
    };
    Some((hive, path))
}

/// The key of the registry, opened for the reading.
fn the_key_to_read(key: &str) -> Result<HKEY, String> {
    the_key_opened(key, KEY_READ)
}

/// The key of the registry, opened for the writing.
///
/// Removing a value of a key needs the right to write in it: the key
/// that is opened for the reading only is not enough, and the machine
/// answers "(5) Access is denied.".
fn the_key_to_write(key: &str) -> Result<HKEY, String> {
    the_key_opened(key, KEY_WRITE)
}

/// The key of the registry, opened with the rights that are asked for.
fn the_key_opened(key: &str, rights: u32) -> Result<HKEY, String> {
    let Some((hive, path)) = the_hive_of(key) else {
        return Err("the key of the registry has no hive: the hive is required.".to_string());
    };
    let mut handle: HKEY = std::ptr::null_mut();
    let answer = unsafe { RegOpenKeyExW(hive, Wide::of(path).as_ptr(), 0, rights, &mut handle) };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    Ok(handle)
}

/// The value of a key of the registry, and the kind of it.
///
/// A value that is not there is `None`, and a key that is not there is
/// the failure of the machine: "(2) The system cannot find the file
/// specified." is what Windows says for a key that is not there.
pub fn registry_value(key: &str, name: &str) -> Result<Option<(String, String)>, String> {
    let handle = the_key_to_read(key)?;
    let mut kind = 0u32;
    let mut size = 0u32;
    let answer = unsafe {
        RegQueryValueExW(
            handle,
            Wide::of(name).as_ptr(),
            std::ptr::null_mut(),
            &mut kind,
            std::ptr::null_mut(),
            &mut size,
        )
    };
    if answer == ERROR_FILE_NOT_FOUND {
        unsafe { RegCloseKey(handle) };
        return Ok(None);
    }
    if answer != 0 {
        unsafe { RegCloseKey(handle) };
        return Err(the_message_of(answer as u32));
    }
    let mut data = vec![0u8; size as usize];
    let answer = unsafe {
        RegQueryValueExW(
            handle,
            Wide::of(name).as_ptr(),
            std::ptr::null_mut(),
            &mut kind,
            data.as_mut_ptr(),
            &mut size,
        )
    };
    unsafe { RegCloseKey(handle) };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    Ok(Some(the_value_of(kind, &data)))
}

/// The items of a key of the registry, for the `Loop, Reg` of the v1.
///
/// Measured on the binary of v1.1.37.02: the values of the key come
/// first, then the subkeys, and each subkey is followed by its own items
/// when the mode asks for the recursion. The mode that is not there is
/// the "V": the values alone.
pub fn registry_enumerate(
    key: &str,
    mode: &str,
    quiet: bool,
) -> Result<Vec<ahk::desktop::TheRegEntry>, String> {
    let mode = mode.trim();
    let mode = if mode.is_empty() { "V" } else { mode };
    let upper = mode.to_ascii_uppercase();
    let with_keys = upper.contains('K');
    let with_values = upper.contains('V');
    let recurse = upper.contains('R');
    let root = the_root_name_of(key);
    let mut out = Vec::new();
    the_key_is_enumerated(key, &root, with_keys, with_values, recurse, quiet, &mut out)?;
    Ok(out)
}

/// The root of a key, spelled whole the way the v1 writes it.
fn the_root_name_of(key: &str) -> String {
    let hive = key.split('\\').next().unwrap_or("").trim().to_uppercase();
    match hive.as_str() {
        "HKCU" | "HKEY_CURRENT_USER" => "HKEY_CURRENT_USER",
        "HKLM" | "HKEY_LOCAL_MACHINE" => "HKEY_LOCAL_MACHINE",
        "HKU" | "HKEY_USERS" => "HKEY_USERS",
        "HKCR" | "HKEY_CLASSES_ROOT" => "HKEY_CLASSES_ROOT",
        "HKCC" | "HKEY_CURRENT_CONFIG" => "HKEY_CURRENT_CONFIG",
        _ => "",
    }
    .to_string()
}

/// The items of one key, and the items of its subkeys when the recursion
/// is asked for.
fn the_key_is_enumerated(
    key: &str,
    root: &str,
    with_keys: bool,
    with_values: bool,
    recurse: bool,
    quiet: bool,
    out: &mut Vec<ahk::desktop::TheRegEntry>,
) -> Result<(), String> {
    use windows_sys::Win32::Storage::FileSystem::FileTimeToLocalFileTime;
    use windows_sys::Win32::System::Time::FileTimeToSystemTime;
    // The v1: the key that cannot be opened gives no items, and the
    // enumeration goes on. Measured on the binary of v1.1.37.02
    // (`source/script.cpp`, the `PerformLoopReg`): the `RegOpenKeyEx`
    // that does not answer `ERROR_SUCCESS` makes the v1 return `OK`.
    let handle = match the_key_to_read(key) {
        Ok(handle) => handle,
        Err(reason) => {
            if quiet {
                return Ok(());
            }
            return Err(reason);
        }
    };
    let subkey = key
        .split_once('\\')
        .map(|(_, rest)| rest.to_string())
        .unwrap_or_default();
    if with_values {
        let mut index = 0u32;
        loop {
            let mut name = vec![0u16; 16384];
            let mut len = name.len() as u32;
            let mut kind = 0u32;
            let answer = unsafe {
                RegEnumValueW(
                    handle,
                    index,
                    name.as_mut_ptr(),
                    &mut len,
                    std::ptr::null_mut(),
                    &mut kind,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };
            if answer == ERROR_NO_MORE_ITEMS {
                break;
            }
            if answer != 0 {
                unsafe { RegCloseKey(handle) };
                return Err(the_message_of(answer as u32));
            }
            out.push(ahk::desktop::TheRegEntry {
                name: String::from_utf16_lossy(&name[..len as usize]),
                kind: the_kind_name(kind),
                root: root.to_string(),
                subkey: subkey.clone(),
                modified: String::new(),
            });
            index += 1;
        }
    }
    if with_keys || recurse {
        let mut index = 0u32;
        loop {
            let mut name = vec![0u16; 16384];
            let mut len = name.len() as u32;
            let mut written: FILETIME = unsafe { std::mem::zeroed() };
            let answer = unsafe {
                RegEnumKeyExW(
                    handle,
                    index,
                    name.as_mut_ptr(),
                    &mut len,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    &mut written,
                )
            };
            if answer == ERROR_NO_MORE_ITEMS {
                break;
            }
            if answer != 0 {
                unsafe { RegCloseKey(handle) };
                return Err(the_message_of(answer as u32));
            }
            let child = String::from_utf16_lossy(&name[..len as usize]);
            if with_keys {
                let mut local: FILETIME = unsafe { std::mem::zeroed() };
                let mut system: SYSTEMTIME = unsafe { std::mem::zeroed() };
                let modified = unsafe {
                    if FileTimeToLocalFileTime(&written, &mut local) != 0
                        && FileTimeToSystemTime(&local, &mut system) != 0
                    {
                        format!(
                            "{:04}{:02}{:02}{:02}{:02}{:02}",
                            system.wYear,
                            system.wMonth,
                            system.wDay,
                            system.wHour,
                            system.wMinute,
                            system.wSecond
                        )
                    } else {
                        String::new()
                    }
                };
                out.push(ahk::desktop::TheRegEntry {
                    name: child.clone(),
                    kind: "KEY".to_string(),
                    root: root.to_string(),
                    subkey: subkey.clone(),
                    modified,
                });
            }
            index += 1;
            if recurse {
                let full = format!("{}\\{}", key, child);
                the_key_is_enumerated(&full, root, with_keys, with_values, recurse, quiet, out)?;
            }
        }
    }
    unsafe { RegCloseKey(handle) };
    Ok(())
}

/// The name of the kind of a value, as the v1 writes it.
fn the_kind_name(kind: u32) -> String {
    match kind {
        REG_SZ => "REG_SZ",
        REG_EXPAND_SZ => "REG_EXPAND_SZ",
        REG_MULTI_SZ => "REG_MULTI_SZ",
        REG_DWORD => "REG_DWORD",
        REG_QWORD => "REG_QWORD",
        REG_BINARY => "REG_BINARY",
        _ => "REG_NONE",
    }
    .to_string()
}

/// The kind of a value and the text of it, as the v2 names them.
fn the_value_of(kind: u32, data: &[u8]) -> (String, String) {
    match kind {
        REG_DWORD => {
            let number = if data.len() >= 4 {
                u32::from_le_bytes([data[0], data[1], data[2], data[3]])
            } else {
                0
            };
            ("REG_DWORD".to_string(), number.to_string())
        }
        REG_QWORD => {
            let number = if data.len() >= 8 {
                u64::from_le_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ])
            } else {
                0
            };
            ("REG_QWORD".to_string(), number.to_string())
        }
        REG_BINARY => {
            let text = data
                .iter()
                .map(|byte| format!("{byte:02X}"))
                .collect::<Vec<String>>()
                .join("");
            ("REG_BINARY".to_string(), text)
        }
        _ => {
            // The text of a value is wide, and the machine gives the
            // bytes of it: the letters are the pairs of them.
            let letters: Vec<u16> = data
                .chunks_exact(2)
                .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                .collect();
            let text = String::from_utf16_lossy(&letters);
            let text = text.trim_end_matches('\0').to_string();
            let kind = if kind == REG_MULTI_SZ {
                "REG_MULTI_SZ"
            } else if kind == REG_EXPAND_SZ {
                "REG_EXPAND_SZ"
            } else {
                "REG_SZ"
            };
            (kind.to_string(), text)
        }
    }
}

/// The kind of a value, as the machine knows it.
fn the_kind_of(kind: &str) -> u32 {
    match kind.trim().to_uppercase().as_str() {
        "REG_EXPAND_SZ" => REG_EXPAND_SZ,
        "REG_MULTI_SZ" => REG_MULTI_SZ,
        "REG_DWORD" => REG_DWORD,
        "REG_QWORD" => REG_QWORD,
        "REG_BINARY" => REG_BINARY,
        _ => REG_SZ,
    }
}

/// Writes a value in a key of the registry.
///
/// The key is made when it is not there, with its ancestors: the
/// documentation of the v2 says so and the binary does it.
pub fn registry_write(key: &str, name: &str, kind: &str, value: &str) -> Result<(), String> {
    let Some((hive, path)) = the_hive_of(key) else {
        return Err("the key of the registry has no hive: the hive is required.".to_string());
    };
    let mut handle: HKEY = std::ptr::null_mut();
    let answer = unsafe {
        RegCreateKeyExW(
            hive,
            Wide::of(path).as_ptr(),
            0,
            std::ptr::null_mut(),
            0,
            KEY_WRITE,
            std::ptr::null(),
            &mut handle,
            std::ptr::null_mut(),
        )
    };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    let kind = the_kind_of(kind);
    let data: Vec<u8> = if kind == REG_DWORD {
        let number = value.trim().parse::<u32>().unwrap_or(0);
        number.to_le_bytes().to_vec()
    } else if kind == REG_QWORD {
        let number = value.trim().parse::<u64>().unwrap_or(0);
        number.to_le_bytes().to_vec()
    } else if kind == REG_BINARY {
        let text = value.trim();
        let mut data = Vec::with_capacity(text.len() / 2);
        let bytes = text.as_bytes();
        let mut index = 0;
        while index + 1 < bytes.len() {
            let pair = &text[index..index + 2];
            match u8::from_str_radix(pair, 16) {
                Ok(byte) => data.push(byte),
                Err(_) => break,
            }
            index += 2;
        }
        data
    } else {
        let mut data: Vec<u8> = value
            .encode_utf16()
            .flat_map(|letter| letter.to_le_bytes())
            .collect();
        data.extend_from_slice(&[0, 0]);
        data
    };
    let answer = unsafe {
        RegSetValueExW(
            handle,
            Wide::of(name).as_ptr(),
            0,
            kind,
            data.as_ptr(),
            data.len() as u32,
        )
    };
    unsafe { RegCloseKey(handle) };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    Ok(())
}

/// Removes a value of a key of the registry.
pub fn registry_delete(key: &str, name: &str) -> Result<(), String> {
    let handle = the_key_to_write(key)?;
    let answer = unsafe { RegDeleteValueW(handle, Wide::of(name).as_ptr()) };
    unsafe { RegCloseKey(handle) };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    Ok(())
}

/// Removes a key of the registry.
pub fn registry_delete_key(key: &str) -> Result<(), String> {
    let Some((hive, path)) = the_hive_of(key) else {
        return Err("the key of the registry has no hive: the hive is required.".to_string());
    };
    let answer = unsafe { RegDeleteKeyW(hive, Wide::of(path).as_ptr()) };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    Ok(())
}

/// Makes a key of the registry, with its ancestors.
pub fn registry_create_key(key: &str) -> Result<(), String> {
    let Some((hive, path)) = the_hive_of(key) else {
        return Err("the key of the registry has no hive: the hive is required.".to_string());
    };
    let mut handle: HKEY = std::ptr::null_mut();
    let answer = unsafe {
        RegCreateKeyExW(
            hive,
            Wide::of(path).as_ptr(),
            0,
            std::ptr::null_mut(),
            0,
            KEY_WRITE,
            std::ptr::null(),
            &mut handle,
            std::ptr::null_mut(),
        )
    };
    if answer != 0 {
        return Err(the_message_of(answer as u32));
    }
    unsafe { RegCloseKey(handle) };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The key that the tests of the registry use.
    ///
    /// The key is under the hive of the user of the run, and its name
    /// carries the number of the process and the name of the test: a key
    /// that a run left behind is marked for deletion by the machine, and
    /// a key that is marked cannot be made again. The tests run at the
    /// same time, and each one has a key of its own.
    fn the_key(what: &str) -> String {
        format!(r"HKCU\Software\ahk-rs-test-{what}-{}", std::process::id())
    }

    #[test]
    fn the_name_of_the_user_is_not_empty() {
        let name = user_name().expect("the machine says the name of the user");
        assert!(!name.is_empty());
    }

    #[test]
    fn the_name_of_the_computer_is_not_empty() {
        let name = computer_name().expect("the machine says the name of the computer");
        assert!(!name.is_empty());
    }

    #[test]
    fn the_version_of_the_operating_system_has_three_numbers() {
        let version = os_version().expect("the machine says the version of the system");
        let numbers: Vec<&str> = version.split('.').collect();
        assert_eq!(numbers.len(), 3, "{version}");
        for number in numbers {
            assert!(
                number.chars().all(|letter| letter.is_ascii_digit()),
                "{version}"
            );
        }
    }

    #[test]
    fn the_language_of_the_session_is_not_empty() {
        let language = language().expect("the machine says the language of the session");
        assert!(!language.is_empty());
    }

    #[test]
    fn the_thread_of_the_script_has_a_number() {
        let id = the_current_thread_id().expect("the machine says the thread of the script");
        assert_ne!(id, 0);
    }

    #[test]
    fn the_drives_of_the_machine_are_not_empty() {
        let drives = drive_list().expect("the machine says its drives");
        assert!(!drives.is_empty());
        for drive in drives {
            assert!(drive.ends_with(':'), "{drive}");
        }
    }

    #[test]
    fn the_folders_of_the_user_are_paths() {
        let folders = folders().expect("the machine says its folders");
        let desktop = folders.desktop.expect("the desktop of the user");
        assert!(std::path::Path::new(&desktop).is_absolute(), "{desktop}");
        let documents = folders.my_documents.expect("the documents of the user");
        assert!(
            std::path::Path::new(&documents).is_absolute(),
            "{documents}"
        );
    }

    #[test]
    fn a_value_of_the_registry_is_written_read_and_removed() {
        let key = the_key("the-value");
        registry_create_key(&key).expect("the key of the registry is made");
        registry_write(&key, "the value", "REG_SZ", "the text of the value")
            .expect("the value is written");
        let (kind, value) = registry_value(&key, "the value")
            .expect("the value is read")
            .expect("the value is there");
        assert_eq!(kind, "REG_SZ");
        assert_eq!(value, "the text of the value");
        registry_delete(&key, "the value").expect("the value is removed");
        assert!(
            registry_value(&key, "the value")
                .expect("the value is read")
                .is_none()
        );
        registry_delete_key(&key).expect("the key of the registry is removed");
    }

    #[test]
    fn a_number_of_the_registry_is_written_as_a_number() {
        let key = the_key("the-number");
        registry_write(&key, "the number", "REG_DWORD", "42").expect("the value is written");
        let (kind, value) = registry_value(&key, "the number")
            .expect("the value is read")
            .expect("the value is there");
        assert_eq!(kind, "REG_DWORD");
        assert_eq!(value, "42");
        let _ = registry_delete_key(&key);
    }

    #[test]
    fn a_key_of_the_registry_without_a_hive_is_refused() {
        let answer = registry_create_key(r"Software\ahk-rs-test");
        assert_eq!(
            answer,
            Err("the key of the registry has no hive: the hive is required.".to_string())
        );
    }
}
