"""O `std` do Rust emendado para o piso do Windows XP.

O `std` de Rust e escrito para o Windows de hoje, e ele pergunta a
maquina por funcoes que o Windows XP nao tem: as fibras (`FlsAlloc` e
suas irmas), o leitor do lock de Vista (`GetFileInformationByHandleEx` e
parentes) e o timer de alta resolucao de Windows 10.

Essas perguntas entram na tabela de importacoes do binario como
`raw-dylib`: o carregador do XP procura cada nome no kernel32, e o
kernel32 do XP nao os tem, e o binario nao comeca.

A emenda troca cada uma delas pela da era do XP: o lugar de thread
(`TlsAlloc` e suas irmas) no lugar da fibra, o `GetFileInformationByHandle`
no lugar do leitor de Vista, e o timer de Windows 2000 no lugar do
timer de Windows 10.

O script e idempotente, e cada emenda que nao encontra o texto e um
aviso — o `std` muda de forma entre as versoes, e a emenda que nao
encontra e a que nao precisa.
"""

import os
import pathlib
import re
import sys

# The toolchain whose `std` is emended. The release builds with a
# pinned nightly, and the emend has to follow it; the default is the
# floating `nightly` of the lab.
FERRAMENTA = (
    pathlib.Path.home()
    / ".rustup/toolchains"
    / os.environ.get("AHK_XP_TOOLCHAIN", "nightly-x86_64-pc-windows-msvc")
    / "lib/rustlib/src/rust/library/std/src"
)

# O arquivo, o texto que se troca, e o texto novo.
EMENDAS = [
    (
        "sys/thread_local/guard/windows.rs",
        "let key_result = unsafe { c::FlsAlloc(dtor) };",
        "let key_result = unsafe { c::TlsAlloc() };",
    ),
    (
        "sys/thread_local/guard/windows.rs",
        "let result = unsafe { c::FlsSetValue(key, ptr) };",
        "let result = unsafe { c::TlsSetValue(key, ptr) };",
    ),
    (
        "sys/thread_local/guard/windows.rs",
        "unsafe { c::FlsFree(new_key) };",
        "unsafe { c::TlsFree(new_key) };",
    ),
    (
        "sys/thread_local/guard/windows.rs",
        "unsafe { c::FlsFree(current_key) };",
        "unsafe { c::TlsFree(current_key) };",
    ),
    (
        "sys/thread_local/guard/windows.rs",
        "let res = unsafe { c::IsThreadAFiber() };\n    res == c::TRUE",
        "false",
    ),
    (
        "sys/pal/windows/time.rs",
        "c::CreateWaitableTimerExW(",
        "c::CreateWaitableTimerW(",
    ),
    # O `CreateWaitableTimerW` tem tres parametros, nao os quatro do
    # `ExW`: o quarto (o acesso) e' so do Ex, e a biblioteca de
    # importacao do MinGW decora o nome com @12, nao @16.
    # O flag de alta resolucao vira 0 ANTES de tirar o quarto argumento,
    # para o padrao do 4->3 casar com o terceiro argumento ja' sendo 0.
    (
        "sys/pal/windows/time.rs",
        "c::CREATE_WAITABLE_TIMER_HIGH_RESOLUTION,",
        "0,",
    ),
    (
        "sys/pal/windows/time.rs",
        "c::CreateWaitableTimerW(\n                null(),\n                null(),\n                0,\n                c::TIMER_ALL_ACCESS,\n            )",
        "c::CreateWaitableTimerW(\n                null(),\n                null(),\n                0,\n            )",
    ),
    # O `CreateWaitableTimerW` saiu das ligacoes do `std` (so o `ExW`
    # ficou): a emenda do time.rs precisa dele, e o XP tem-no desde
    # Windows 2000.
    (
        "sys/pal/windows/c/windows_sys.rs",
        'windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const SECURITY_ATTRIBUTES, lptimername : PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> HANDLE);\n'
        'windows_link::link!("kernel32.dll" "system" fn DeleteFileW(',
        'windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExW(lptimerattributes : *const SECURITY_ATTRIBUTES, lptimername : PCWSTR, dwflags : u32, dwdesiredaccess : u32) -> HANDLE);\n'
        'windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerW(lptimerattributes : *const SECURITY_ATTRIBUTES, lptimername : PCWSTR, bmanualreset : BOOL) -> HANDLE);\n'
        'windows_link::link!("kernel32.dll" "system" fn DeleteFileW(',
    ),
    (
        "sys/pal/windows/stack_overflow.rs",
        'let result = unsafe { c::SetThreadStackGuarantee(&mut 0x5000) };\n'
        '    // Reserving stack space is not critical so we allow it to fail in the released build of libstd.\n'
        '    // We still use debug assert here so that CI will test that we haven\'t made a mistake calling the function.\n'
        '    debug_assert_ne!(result, 0, "failed to reserve stack space for exception handling");',
        "// O XP nao tem SetThreadStackGuarantee: a reserva do espaco nao e critica.",
    ),
    (
        "sys/pal/windows/api.rs",
        "let result = c::SetFileInformationByHandle(handle, class, info, size);",
        "let result = 0;",
    ),
    (
        "sys/fs/windows.rs",
        "|buf, sz| unsafe { c::GetFinalPathNameByHandleW(handle, buf, sz, c::VOLUME_NAME_DOS) },",
        "|_buf, _sz| 0,",
    ),
    (
        "sys/fs/windows.rs",
        "|buf, sz| unsafe { c::GetFinalPathNameByHandleW(handle, buf, sz, c::VOLUME_NAME_NT) },",
        "|_buf, _sz| 0,",
    ),
    (
        "sys/fs/windows.rs",
        "let result = unsafe {\n                c::SetFileInformationByHandle(\n                    f.as_raw_handle(),\n                    c::FileRenameInfoEx,\n                    file_rename_info.cast::<c_void>(),\n                    struct_size,\n                )\n            };",
        "let result = 0;",
    ),
    (
        "sys/process/windows.rs",
        "let result = c::CompareStringOrdinal(\n                self.utf16.as_ptr(),\n                self.utf16.len() as _,\n                other.utf16.as_ptr(),\n                other.utf16.len() as _,\n                c::TRUE,\n            );",
        "let fold = |c: u16| if c >= 65 && c <= 90 { c + 32 } else { c }; let result = if self.utf16.len() == other.utf16.len() && self.utf16.iter().zip(other.utf16.iter()).all(|(a, b)| fold(*a) == fold(*b)) { c::CSTR_EQUAL } else if self.utf16 < other.utf16 { c::CSTR_LESS_THAN } else { c::CSTR_GREATER_THAN };",
    ),
    # O `SetEvent` nao esta nas ligacoes do `std`, e o condvar do XP
    # precisa dele: o `CreateEventW` esta la, o irmao nao.
    #
    # A ancora vem com a linha de baixo (`CreateFileW`) porque a emenda
    # tem de ser idempotente: sem ela, o texto velho continua dentro do
    # texto novo, e cada corrida do script acrescenta outro par.
    (
        "sys/pal/windows/c/windows_sys.rs",
        'windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);\n'
        'windows_link::link!("kernel32.dll" "system" fn CreateFileW(',
        'windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);\n'
        'windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);\n'
        'windows_link::link!("kernel32.dll" "system" fn ResetEvent(hevent : HANDLE) -> BOOL);\n'
        'windows_link::link!("kernel32.dll" "system" fn CreateFileW(',
    ),
    # O `println!` pergunta se o descritor e um terminal, e a pergunta
    # cai no truque do msys, que le o nome do cano com o leitor de Vista.
    (
        "sys/io/is_terminal/windows.rs",
        "    msys_tty_on(handle)\n}",
        "    // O leitor de Vista nao esta no XP: sem o truque do msys, o cano nao e um pty.\n"
        "    let _ = handle;\n"
        "    false\n}",
    ),
    # O corpo da funcao some inteiro, e nao so a chamada: o
    # `#[allow(dead_code)]` cala o aviso, mas a referencia ao leitor de
    # Vista continua no codigo, e o `raw-dylib` poe o nome na tabela de
    # importacoes do mesmo jeito. So o corpo vazio tira o simbolo.
    # A funcao e a ultima do arquivo; quem a corta e o O_MSYS, abaixo.
    # A lista de atributos do processo e de Vista. O construtor recusa,
    # e as tres chamadas somem da tabela de importacoes.
    (
        "os/windows/process.rs",
        "        let Ok(attribute_count) = self.attributes.len().try_into() else {",
        "        let Ok(attribute_count): Result<u32, _> = self.attributes.len().try_into() else {",
    ),
    (
        "os/windows/process.rs",
        "        unsafe { sys::c::DeleteProcThreadAttributeList(lp_attribute_list) }",
        "        let _ = lp_attribute_list;",
    ),
    (
        "os/windows/process.rs",
        "        unsafe {\n"
        "            sys::c::InitializeProcThreadAttributeList(\n"
        "                ptr::null_mut(),\n"
        "                attribute_count,\n"
        "                0,\n"
        "                &mut required_size,\n"
        "            )\n"
        "        };",
        "        // O XP nao tem a lista de atributos: o construtor recusa.\n"
        "        let _ = (&mut required_size, attribute_count);\n"
        "        return Err(io::const_error!(\n"
        "            io::ErrorKind::Unsupported,\n"
        '            "ProcThreadAttributeList is a Windows Vista thing",\n'
        "        ));",
    ),
    (
        "os/windows/process.rs",
        "        sys::cvt(unsafe {\n"
        "            sys::c::InitializeProcThreadAttributeList(\n"
        "                attribute_list.as_mut_ptr().cast::<sys::c::_PROC_THREAD_ATTRIBUTE_LIST>(),\n"
        "                attribute_count,\n"
        "                0,\n"
        "                &mut required_size,\n"
        "            )\n"
        "        })?;",
        "        let _ = (&attribute_list, attribute_count);",
    ),
    (
        "os/windows/process.rs",
        "            sys::cvt(unsafe {\n"
        "                sys::c::UpdateProcThreadAttribute(\n"
        "                    attribute_list.as_mut_ptr().cast::<sys::c::_PROC_THREAD_ATTRIBUTE_LIST>(),\n"
        "                    0,\n"
        "                    attribute,\n"
        "                    value.ptr,\n"
        "                    value.size,\n"
        "                    ptr::null_mut(),\n"
        "                    ptr::null_mut(),\n"
        "                )\n"
        "            })?;",
        "            let _ = (&attribute_list, attribute, value);",
    ),
    # A informacao basica do arquivo: o leitor de Vista sai, o
    # `GetFileInformationByHandle` de Windows 2000 entra. Ele da os mesmos
    # atributos e as mesmas tres horas; o `ChangeTime` nao tem de onde sair
    # no XP, e o seu unico chamador (o `remove_dir_all`) so le os atributos.
    (
        "sys/fs/windows.rs",
        "    fn basic_info(&self) -> io::Result<c::FILE_BASIC_INFO> {\n"
        "        unsafe {\n"
        "            let mut info: c::FILE_BASIC_INFO = mem::zeroed();\n"
        "            let size = size_of_val(&info);\n"
        "            cvt(c::GetFileInformationByHandleEx(\n"
        "                self.handle.as_raw_handle(),\n"
        "                c::FileBasicInfo,\n"
        "                (&raw mut info) as *mut c_void,\n"
        "                size as u32,\n"
        "            ))?;\n"
        "            Ok(info)\n"
        "        }\n"
        "    }",
        "    fn basic_info(&self) -> io::Result<c::FILE_BASIC_INFO> {\n"
        "        // O XP nao tem o leitor de Vista: o `GetFileInformationByHandle`\n"
        "        // de Windows 2000 diz o mesmo, com o `ChangeTime` em zero.\n"
        "        unsafe {\n"
        "            let mut by: c::BY_HANDLE_FILE_INFORMATION = mem::zeroed();\n"
        "            cvt(c::GetFileInformationByHandle(self.handle.as_raw_handle(), &mut by))?;\n"
        "            fn hora(t: c::FILETIME) -> i64 {\n"
        "                ((t.dwHighDateTime as i64) << 32) | t.dwLowDateTime as i64\n"
        "            }\n"
        "            Ok(c::FILE_BASIC_INFO {\n"
        "                CreationTime: hora(by.ftCreationTime),\n"
        "                LastAccessTime: hora(by.ftLastAccessTime),\n"
        "                LastWriteTime: hora(by.ftLastWriteTime),\n"
        "                ChangeTime: 0,\n"
        "                FileAttributes: by.dwFileAttributes,\n"
        "            })\n"
        "        }\n"
        "    }",
    ),
    # O `fill_dir_buff` le a pasta com o leitor de Vista. O
    # `NtQueryDirectoryFile` do ntdll faz o mesmo desde o NT 3.1, e a
    # classe 8 enche o buffer com o mesmo `FILE_ID_BOTH_DIR_INFO` que o
    # `DirBuffIter` le. (O numero nao e o 10/11 do
    # `GetFileInformationByHandleEx`: sao duas enumeracoes diferentes.)
    (
        "sys/pal/windows/c/windows_sys.rs",
        'windows_link::link!("ntdll.dll" "system" fn NtSetInformationFile(filehandle : HANDLE, iostatusblock : *mut IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS) -> NTSTATUS);\n'
        'windows_link::link!("ntdll.dll" "system" fn NtWriteFile(',
        'windows_link::link!("ntdll.dll" "system" fn NtSetInformationFile(filehandle : HANDLE, iostatusblock : *mut IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS) -> NTSTATUS);\n'
        '// O XP precisa do leitor de pastas do ntdll: os dois ultimos parametros\n'
        '// sao BOOLEAN (um byte), que as ligacoes do `std` nao declaram.\n'
        'windows_link::link!("ntdll.dll" "system" fn NtQueryDirectoryFile(filehandle : HANDLE, event : HANDLE, apcroutine : PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : FILE_INFORMATION_CLASS, returnsingleentry : u8, filename : *const UNICODE_STRING, restartscan : u8) -> NTSTATUS);\n'
        'windows_link::link!("ntdll.dll" "system" fn NtWriteFile(',
    ),
    (
        "sys/fs/windows.rs",
        "    fn fill_dir_buff(&self, buffer: &mut DirBuff, restart: bool) -> Result<bool, WinError> {\n"
        "        let class =\n"
        "            if restart { c::FileIdBothDirectoryRestartInfo } else { c::FileIdBothDirectoryInfo };\n"
        "\n"
        "        unsafe {\n"
        "            let result = c::GetFileInformationByHandleEx(\n"
        "                self.as_raw_handle(),\n"
        "                class,\n"
        "                buffer.as_mut_ptr().cast(),\n"
        "                buffer.capacity() as _,\n"
        "            );\n"
        "            if result == 0 {\n"
        "                // Apparently ERROR_NO_MORE_FILES also means that the buffer is empty / was left\n"
        "                // unchanged. FIXME: find docs that say this.\n"
        "                let err = api::get_last_error();\n"
        "                if err.code == c::ERROR_NO_MORE_FILES { Ok(false) } else { Err(err) }\n"
        "            } else {\n"
        "                Ok(true)\n"
        "            }\n"
        "        }\n"
        "    }",
        "    fn fill_dir_buff(&self, buffer: &mut DirBuff, restart: bool) -> Result<bool, WinError> {\n"
        "        // O XP nao tem o `GetFileInformationByHandleEx`: o leitor de\n"
        "        // pastas do ntdll entra no lugar dele, e o buffer sai com o mesmo\n"
        "        // formato de sempre.\n"
        "        const FILE_ID_BOTH_DIRECTORY_INFORMATION: c::FILE_INFORMATION_CLASS = 8;\n"
        "        const STATUS_NO_MORE_FILES: c::NTSTATUS = -2147483642;\n"
        "\n"
        "        unsafe {\n"
        "            let mut iosb: c::IO_STATUS_BLOCK = mem::zeroed();\n"
        "            let status = c::NtQueryDirectoryFile(\n"
        "                self.as_raw_handle(),\n"
        "                ptr::null_mut(),\n"
        "                None,\n"
        "                ptr::null_mut(),\n"
        "                &mut iosb,\n"
        "                buffer.as_mut_ptr().cast(),\n"
        "                buffer.capacity() as u32,\n"
        "                FILE_ID_BOTH_DIRECTORY_INFORMATION,\n"
        "                0,\n"
        "                ptr::null(),\n"
        "                restart as u8,\n"
        "            );\n"
        "            if status == 0 {\n"
        "                Ok(true)\n"
        "            } else if status == STATUS_NO_MORE_FILES {\n"
        "                // A pasta acabou e o buffer ficou como estava.\n"
        "                Ok(false)\n"
        "            } else {\n"
        "                Err(WinError::new(c::RtlNtStatusToDosError(status)))\n"
        "            }\n"
        "        }\n"
        "    }",
    ),
    # O `FindFirstFileExW` com `FindExInfoBasic` e' de Vista: no XP ele
    # devolve ERROR_INVALID_PARAMETER (87) e o `read_dir` morre. O
    # `FindExInfoStandard` existe desde o Windows 2000 e enche o mesmo
    # `WIN32_FIND_DATAW` (so que tambem o nome alternativo, que ninguem
    # le). A troca vale para as duas chamadas do arquivo.
    (
        "sys/fs/windows.rs",
        "c::FindExInfoBasic",
        "0",
    ),
    (
        "sys/fs/windows.rs",
        "c::FindExInfoStandard",
        "0",
    ),
    # O link simbolico e de Vista; o XP tem o hard link e mais nada.
    (
        "sys/fs/windows.rs",
        "    let result = cvt(unsafe {\n"
        "        c::CreateSymbolicLinkW(\n"
        "            link.as_ptr(),\n"
        "            original.as_ptr(),\n"
        "            flags | c::SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE,\n"
        "        ) as c::BOOL\n"
        "    });\n"
        "    if let Err(err) = result {\n"
        "        if err.raw_os_error() == Some(c::ERROR_INVALID_PARAMETER as i32) {\n"
        "            // Older Windows objects to SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE,\n"
        "            // so if we encounter ERROR_INVALID_PARAMETER, retry without that flag.\n"
        "            cvt(unsafe {\n"
        "                c::CreateSymbolicLinkW(link.as_ptr(), original.as_ptr(), flags) as c::BOOL\n"
        "            })?;\n"
        "        } else {\n"
        "            return Err(err);\n"
        "        }\n"
        "    }\n"
        "    Ok(())",
        "    // O XP nao tem o link simbolico: ele e de Vista.\n"
        "    let _ = (original, link, flags);\n"
        "    Err(io::const_error!(\n"
        "        io::ErrorKind::Unsupported,\n"
        '        "symlinks are a Windows Vista thing",\n'
        "    ))",
    ),
]

# Ha arquivos que nao dao para emendar de pedaco: o condvar inteiro e de
# Vista, e a troca e o arquivo todo. A substituicao e idempotente pelo
# mesmo motivo que a emenda e: se o texto novo ja esta la, nao se faz nada.
SUBSTITUICOES = {
    "sys/sync/condvar/windows7.rs": '''//! A variavel de condicao do Windows XP.
//!
//! O `std` do alvo `win7` espera com o `SleepConditionVariableSRW`, que
//! e de Vista: o kernel32 do XP nao o tem, o nome entra na tabela de
//! importacoes como `raw-dylib`, e o carregador mata o processo antes da
//! primeira instrucao. Aqui a espera e o evento de reposicao automatica
//! do Windows 2000: o `notify_one` arma o evento uma vez, e um evento de
//! reposicao automatica acorda um espera so; o `notify_all` arma uma vez
//! por espera que houver.
//!
//! O evento nasce na primeira espera, porque o `new` e `const`. O lugar
//! dele se disputa com o `compare_exchange`, e quem perde fecha o seu.

use crate::ptr;
use crate::sync::atomic::{AtomicIsize, AtomicU32, Ordering};
use crate::sys::c;
use crate::sys::sync::Mutex;
use crate::time::Duration;

pub struct Condvar {
    /// O `HANDLE` do evento, ou zero enquanto ninguem esperou.
    evento: AtomicIsize,
    /// Quantos esperas estao entre o abrir do lock e o voltar dele.
    esperando: AtomicU32,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

impl Condvar {
    #[inline]
    pub const fn new() -> Condvar {
        Condvar { evento: AtomicIsize::new(0), esperando: AtomicU32::new(0) }
    }

    /// O evento da espera, criado na primeira vez que alguem espera.
    fn evento(&self) -> c::HANDLE {
        let ja = self.evento.load(Ordering::Acquire);
        if ja != 0 {
            return ja as c::HANDLE;
        }
        // Reposicao automatica (`bmanualreset = FALSE`): um `SetEvent`
        // acorda um espera so, que e o que o `notify_one` quer.
        let novo = unsafe { c::CreateEventW(ptr::null_mut(), c::FALSE, c::FALSE, ptr::null()) };
        if novo.is_null() {
            return ptr::null_mut();
        }
        match self.evento.compare_exchange(0, novo as isize, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => novo,
            Err(vencedor) => {
                unsafe { c::CloseHandle(novo) };
                vencedor as c::HANDLE
            }
        }
    }

    /// Abre o lock, espera o evento, fecha o lock de novo.
    ///
    /// A conta dos esperas sobe ANTES de o lock abrir: o `notify_all`
    /// que correr daqui em diante ve este espera e arma o evento para
    /// ele. E como o evento fica armado quando nao ha quem o consuma,
    /// a janela entre o abrir do lock e a espera nao perde o aviso.
    fn espera(&self, mutex: &Mutex, milissegundos: u32) -> bool {
        self.esperando.fetch_add(1, Ordering::SeqCst);
        unsafe { mutex.unlock() };
        let acordado = if self.evento().is_null() {
            // Sem evento nao ha espera de verdade: dorme o que der.
            unsafe { c::Sleep(milissegundos) };
            true
        } else {
            let evento = self.evento.load(Ordering::Acquire) as c::HANDLE;
            unsafe { c::WaitForSingleObject(evento, milissegundos) != c::WAIT_TIMEOUT }
        };
        mutex.lock();
        self.esperando.fetch_sub(1, Ordering::SeqCst);
        acordado
    }

    /// Arma o evento: uma vez para o `notify_one`, uma por espera para o
    /// `notify_all`.
    fn acorda(&self, todos: bool) {
        let evento = self.evento.load(Ordering::Acquire);
        if evento == 0 {
            // Ninguem esperou ainda: nao ha evento e nao ha quem acordar.
            return;
        }
        let espera = self.esperando.load(Ordering::SeqCst);
        let vezes = if todos { espera } else { espera.min(1) };
        for _ in 0..vezes {
            unsafe { c::SetEvent(evento as c::HANDLE) };
        }
    }

    #[inline]
    pub unsafe fn wait(&self, mutex: &Mutex) {
        self.espera(mutex, c::INFINITE);
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        self.espera(mutex, crate::sys::pal::dur2timeout(dur))
    }

    #[inline]
    pub fn notify_one(&self) {
        self.acorda(false)
    }

    #[inline]
    pub fn notify_all(&self) {
        self.acorda(true)
    }
}
''',
    "sys/sync/mutex/windows7.rs": '''//! O mutex do Windows XP: um spin lock de atomos.
//!
//! O `std` do alvo `win7` fecha com `AcquireSRWLockExclusive` e suas
//! irmas, que sao de Vista: o kernel32 do XP nao as tem, os nomes
//! entram na tabela de importacoes como `raw-dylib`, e o carregador
//! mata o processo antes da primeira instrucao. Aqui o lock e um
//! atomo, e a espera que gira cede a thread com o `SwitchToThread`,
//! que e de Windows 2000.
//!
//! O lock nao e recursivo, como o SRWLock nao e: quem fecha duas
//! vezes morre, que e o comportamento da implementacao de Unix e o
//! que o `std` prefere (o #19962 do arquivo original).

use crate::sync::atomic::{AtomicBool, Ordering};
use crate::sys::c;

pub struct Mutex {
    locked: AtomicBool,
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

/// O ponteiro cru do original e o do SRWLock; no piso do XP ninguem
/// o pede (o condvar espera no evento proprio), e a assinatura fica
/// para o `mod` a poder reexportar.
#[inline]
pub unsafe fn raw(m: &Mutex) -> *mut c::SRWLOCK {
    m as *const Mutex as *mut c::SRWLOCK
}

impl Mutex {
    #[inline]
    pub const fn new() -> Mutex {
        Mutex { locked: AtomicBool::new(false) }
    }

    #[inline]
    pub fn lock(&self) {
        let mut spins = 0u32;
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            spins += 1;
            if spins > 100 {
                unsafe { c::SwitchToThread() };
                spins = 0;
            } else {
                crate::hint::spin_loop();
            }
        }
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        self.locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}
''',
}

# O leitor de Vista do arquivo: o `GetFileInformationByHandle` de
# Windows 2000 responde a mesma pergunta.
O_LEITOR = re.compile(
    r"cvt\(c::GetFileInformationByHandleEx\(\s*self\.handle\.as_raw_handle\(\),\s*"
    r"c::FileAttributeTagInfo,\s*\(&raw mut attr_tag\)\.cast\(\),\s*"
    r"size_of::<c::FILE_ATTRIBUTE_TAG_INFO>\(\)\.try_into\(\)\.unwrap\(\),\s*\)\)\?;"
)


# O truque do msys e a ultima funcao do arquivo, e ela e inteira feita
# do leitor de Vista: corta-se da assinatura ate o fim.
O_MSYS = re.compile(r"(?s)(?:#\[allow\(dead_code\)\]\n)?fn msys_tty_on\b.*\Z")

O_MSYS_NOVO = '''fn msys_tty_on(_handle: BorrowedHandle<'_>) -> bool {
    // O XP nao tem o `GetFileInformationByHandleEx`, que e como o `std`
    // le o nome do cano para achar o pty do msys. Sem o nome nao ha
    // truque: um cano nao e um terminal.
    false
}
'''


def emenda(arquivo, antes, depois):
    caminho = FERRAMENTA / arquivo
    if not caminho.exists():
        print(f"AVISO: {arquivo} nao esta no `std` desta ferramenta")
        return False
    texto = caminho.read_text(encoding="utf-8")
    if antes not in texto:
        if depois in texto:
            return True
        print(f"AVISO: {arquivo}: nao achei o texto de {antes[:44]!r}")
        return False
    caminho.write_text(texto.replace(antes, depois), encoding="utf-8")
    print(f"emendado: {arquivo}: {antes[:48]!r}")
    return True


def substituicao(arquivo, novo):
    """Troca o arquivo inteiro. Idempotente: se ja e o texto novo, nada."""
    caminho = FERRAMENTA / arquivo
    if not caminho.exists():
        print(f"AVISO: {arquivo} nao esta no `std` desta ferramenta")
        return False
    texto = caminho.read_text(encoding="utf-8")
    if texto == novo:
        return True
    caminho.write_text(novo, encoding="utf-8")
    print(f"substituido: {arquivo} (o arquivo inteiro)")
    return True


def principal():
    if not FERRAMENTA.exists():
        print(f"o `std` nao esta em {FERRAMENTA}")
        return 1
    for arquivo, antes, depois in EMENDAS:
        emenda(arquivo, antes, depois)
    for arquivo, novo in SUBSTITUICOES.items():
        substituicao(arquivo, novo)

    # O `CreateWaitableTimerW` tem tres parametros; o quarto (o acesso) e'
    # so do ExW. O padrao literal pode nao casar com a indentacao do nightly,
    # entao tira-se a linha do TIMER_ALL_ACCESS por regex (so aparece nesta
    # chamada no time.rs).
    caminho = FERRAMENTA / "sys/pal/windows/time.rs"
    if caminho.exists():
        texto = caminho.read_text(encoding="utf-8")
        novo = re.sub(r"\n[ \t]*c::TIMER_ALL_ACCESS,", "", texto)
        if novo != texto:
            caminho.write_text(novo, encoding="utf-8")
            print("emendado: time.rs: o 4o argumento (TIMER_ALL_ACCESS) do CreateWaitableTimerW")

    caminho = FERRAMENTA / "sys/io/is_terminal/windows.rs"
    if caminho.exists():
        texto = caminho.read_text(encoding="utf-8")
        if O_MSYS.search(texto) and "c::GetFileInformationByHandleEx(" in texto:
            caminho.write_text(O_MSYS.sub(O_MSYS_NOVO, texto), encoding="utf-8")
            print("emendado: sys/io/is_terminal/windows.rs: o truque do msys")

    caminho = FERRAMENTA / "sys/fs/windows.rs"
    if caminho.exists():
        texto = caminho.read_text(encoding="utf-8")
        achado = O_LEITOR.search(texto)
        if achado:
            texto = texto.replace(
                achado.group(0),
                "attr_tag.FileAttributes = info.dwFileAttributes;",
            )
            caminho.write_text(texto, encoding="utf-8")
            print("emendado: sys/fs/windows.rs: o leitor de Vista")
        else:
            print("AVISO: o leitor de Vista nao esta no `std` desta ferramenta")
    return 0


if __name__ == "__main__":
    sys.exit(principal())
