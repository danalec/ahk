//! The pickers of the era of XP: the common dialogs and the shell.
//!
//! The picker of the modern tree is the `IFileDialog` of the shell,
//! which is of Vista: the class is not registered on Windows XP
//! (measured on the two guests of the lab: the answer is
//! `0x80040154`). On XP the tree asks the common dialogs of comdlg32
//! instead -- `GetOpenFileNameW` and `GetSaveFileNameW`, which are of
//! Windows 2000 -- and, for the folders, the `SHBrowseForFolderW` of
//! the shell.
//!
//! The flags of the script are the `FOS_*` of the picker of Vista, and
//! the common dialogs take the `OFN_*`, which carry the same numbers
//! for the flags that the two share. The folder picker is the one
//! thing the common dialogs do not have, and the shell answers it.

use windows_sys::Win32::System::Com::CoTaskMemFree;
use windows_sys::Win32::UI::Controls::Dialogs::{
    GetOpenFileNameW, GetSaveFileNameW, OPENFILENAMEW, OFN_ALLOWMULTISELECT, OFN_CREATEPROMPT,
    OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_NOCHANGEDIR, OFN_NODEREFERENCELINKS,
    OFN_OVERWRITEPROMPT, OFN_PATHMUSTEXIST,
};
use windows_sys::Win32::UI::Shell::{
    SHBrowseForFolderW, SHGetPathFromIDListW, BROWSEINFOW, BIF_NEWDIALOGSTYLE,
    BIF_RETURNONLYFSDIRS,
};

use super::sys::Wide;

/// `FOS_PICKFOLDERS`: the picker asks for a folder instead of a file.
const FOS_PICKFOLDERS: u32 = 0x20;

/// The picker of the era of XP, gathering what the script asks before
/// it is shown.
pub struct Classic {
    save: bool,
    options: u32,
    title: String,
    filters: Vec<(String, String)>,
    file_name: String,
    folder: String,
}

impl Classic {
    pub fn new(save: bool) -> Self {
        Classic {
            save,
            options: 0,
            title: String::new(),
            filters: Vec::new(),
            file_name: String::new(),
            folder: String::new(),
        }
    }

    pub fn the_options_are_set(&mut self, options: u32) {
        self.options = options;
    }

    pub fn the_title_is_written(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn the_filter_is_written(&mut self, kinds: &[(&str, &str)]) {
        self.filters = kinds
            .iter()
            .map(|(name, pattern)| (name.to_string(), pattern.to_string()))
            .collect();
    }

    pub fn the_file_name_is_written(&mut self, name: &str) {
        self.file_name = name.to_string();
    }

    pub fn the_folder_is_set(&mut self, path: &str) {
        self.folder = path.to_string();
    }

    /// Show the picker, and answer with the paths the user chose.
    pub fn the_picker_is_shown(&self, many: bool) -> Result<Option<Vec<String>>, String> {
        if self.options & FOS_PICKFOLDERS != 0 {
            return self.the_folder_is_shown();
        }
        self.the_files_are_shown(many)
    }

    /// The common dialogs of comdlg32: the files that the user chose.
    fn the_files_are_shown(&self, many: bool) -> Result<Option<Vec<String>>, String> {
        // The filter is the text of the machine: each kind is a name
        // and a pattern, each ended in zero, and the whole ended in
        // one zero more.
        let mut filter: Vec<u16> = Vec::new();
        for (name, pattern) in &self.filters {
            filter.extend(name.encode_utf16());
            filter.push(0);
            filter.extend(pattern.encode_utf16());
            filter.push(0);
        }
        filter.push(0);

        let title = Wide::of(&self.title);
        let folder = Wide::of(&self.folder);
        // The buffer of the file is large: the choice of many files
        // carries the folder and every name in it.
        let mut buffer = [0u16; 32768];
        for (at, unit) in self.file_name.encode_utf16().enumerate() {
            if at < buffer.len() - 1 {
                buffer[at] = unit;
            }
        }

        let mut flags = OFN_EXPLORER | OFN_NOCHANGEDIR;
        // The flags of the script that the common dialogs share.
        if self.options & 0x200 != 0 {
            flags |= OFN_ALLOWMULTISELECT;
        }
        if self.options & 0x1000 != 0 {
            flags |= OFN_FILEMUSTEXIST;
        }
        if self.options & 0x800 != 0 {
            flags |= OFN_PATHMUSTEXIST;
        }
        if self.options & 0x2000 != 0 {
            flags |= OFN_CREATEPROMPT;
        }
        if self.options & 0x2 != 0 {
            flags |= OFN_OVERWRITEPROMPT;
        }
        if self.options & 0x100000 != 0 {
            flags |= OFN_NODEREFERENCELINKS;
        }
        if many {
            flags |= OFN_ALLOWMULTISELECT;
        }

        let mut ofn: OPENFILENAMEW = unsafe { core::mem::zeroed() };
        ofn.lStructSize = core::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.lpstrFilter = if self.filters.is_empty() {
            core::ptr::null()
        } else {
            filter.as_ptr()
        };
        ofn.lpstrFile = buffer.as_mut_ptr();
        ofn.nMaxFile = buffer.len() as u32;
        ofn.lpstrInitialDir = if self.folder.is_empty() {
            core::ptr::null()
        } else {
            folder.as_ptr()
        };
        ofn.lpstrTitle = if self.title.is_empty() {
            core::ptr::null()
        } else {
            title.as_ptr()
        };
        ofn.Flags = flags;

        let answer = unsafe {
            if self.save {
                GetSaveFileNameW(&mut ofn)
            } else {
                GetOpenFileNameW(&mut ofn)
            }
        };
        if answer == 0 {
            // The user closed the picker without choosing anything.
            return Ok(None);
        }
        Ok(Some(the_paths_of_the_buffer(&buffer)))
    }

    /// The shell of the machine: the folder that the user chose.
    fn the_folder_is_shown(&self) -> Result<Option<Vec<String>>, String> {
        let title = Wide::of(&self.title);
        let mut info: BROWSEINFOW = unsafe { core::mem::zeroed() };
        info.lpszTitle = if self.title.is_empty() {
            core::ptr::null()
        } else {
            title.as_ptr()
        };
        // The new style carries the button that makes a new folder,
        // which is the option 1 of the v2.
        info.ulFlags = BIF_RETURNONLYFSDIRS | BIF_NEWDIALOGSTYLE;
        let item = unsafe { SHBrowseForFolderW(&info) };
        if item.is_null() {
            return Ok(None);
        }
        let mut path = [0u16; 32768];
        let answer = unsafe { SHGetPathFromIDListW(item, path.as_mut_ptr()) };
        unsafe { CoTaskMemFree(item.cast()) };
        if answer == 0 {
            return Ok(None);
        }
        let end = path.iter().position(|&c| c == 0).unwrap_or(path.len());
        Ok(Some(vec![String::from_utf16_lossy(&path[..end])]))
    }
}

/// The paths of the buffer of the common dialogs.
///
/// With the choice of many files, the buffer carries the folder and
/// then every name, each ended in zero, and the whole ended in one
/// zero more; with one file, the buffer is the whole path.
fn the_paths_of_the_buffer(buffer: &[u16]) -> Vec<String> {
    let mut texts: Vec<String> = Vec::new();
    let mut start = 0;
    for at in 0..buffer.len() {
        if buffer[at] == 0 {
            if at == start {
                break;
            }
            texts.push(String::from_utf16_lossy(&buffer[start..at]));
            start = at + 1;
        }
    }
    if texts.len() <= 1 {
        return texts;
    }
    // The first text is the folder, and the rest are the names in it.
    let folder = texts.remove(0);
    texts
        .into_iter()
        .map(|name| {
            let mut path = folder.clone();
            if !path.ends_with('\\') {
                path.push('\\');
            }
            path.push_str(&name);
            path
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_of_one_file_is_the_whole_path() {
        let mut buffer = [0u16; 64];
        let path = "C:\\folder\\file.txt";
        for (at, unit) in path.encode_utf16().enumerate() {
            buffer[at] = unit;
        }
        let paths = the_paths_of_the_buffer(&buffer);
        assert_eq!(paths, vec![path.to_string()]);
    }

    #[test]
    fn the_buffer_of_many_files_joins_the_folder() {
        let mut buffer = [0u16; 128];
        let text = "C:\\folder\0a.txt\0b.txt\0";
        for (at, unit) in text.encode_utf16().enumerate() {
            buffer[at] = unit;
        }
        let paths = the_paths_of_the_buffer(&buffer);
        assert_eq!(
            paths,
            vec!["C:\\folder\\a.txt".to_string(), "C:\\folder\\b.txt".to_string()]
        );
    }
}
