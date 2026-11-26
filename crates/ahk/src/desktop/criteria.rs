//! The criteria that name a window.
//!
//! Both dialects accept criteria that are not a title: `ahk_class`,
//! `ahk_exe`, `ahk_id` and `ahk_pid`. The text that a script gives is
//! read here, once, and the parts that it has are kept apart.
//!
//! The rules were measured on the binaries of v1.1.37.02 and of
//! v2.0.31, and the two agree on every case that was
//! asked:
//!
//! - the name of the criterion is at the start of the text or after a
//!   space, and the space that separates it from the text before it
//!   belongs to the text before it;
//! - the name of the criterion is read without a caixa: `AHK_CLASS` e o
//!   mesmo criterio que `ahk_class`;
//! - the value of the criterion is everything that follows the name of
//!   it, with the spaces in front of it left out, and it ends where the
//!   next criterion begins;
//! - the title is the text that is before the first criterion;
//! - `ahk_class` compares the class of the window, and the case
//!   matters;
//! - `ahk_exe` compares the name of the executable of the window, and
//!   the case does not matter;
//! - `ahk_id` and `ahk_pid` take a whole number, and the number may be
//!   written in hexadecimal with a `0x` in front of it; a number that
//!   is not a number names no window at all;
//! - a criterion without a value names no window;
//! - two criteria of the same kind are both asked, and the last one of
//!   them is the one that counts.

use crate::desktop::TitleMatch;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// The names of the criteria that the machine reads.
const THE_NAMES: [&str; 4] = ["ahk_class", "ahk_exe", "ahk_id", "ahk_pid"];

/// The parts of the text that names a window.
///
/// The title is the text that is left after the criteria are read. It
/// is empty when the text was made only of criteria.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WindowCriteria {
    /// The title of the window that is wanted.
    pub title: String,
    /// The class of the window that is wanted, and the empty text when
    /// the text did not name a class.
    pub class: Option<String>,
    /// The name of the executable of the window that is wanted, and the
    /// empty text when the text did not name one.
    pub exe: Option<String>,
    /// The identifier of the window that is wanted, and the empty text
    /// when the text did not name one.
    ///
    /// The text is kept as it is, and not as a number: a number that is
    /// not a number names no window, and the tree has to know the
    /// difference between a criterion that was not written and a
    /// criterion that cannot be answered.
    pub id: Option<String>,
    /// The process of the window that is wanted, and the empty text
    /// when the text did not name one.
    pub pid: Option<String>,
    /// Whether the target is the window that has the focus.
    ///
    /// The title `A` alone is that window: the v1 and the v2 ask for it
    /// that way. The rule is the macro of the v1 (source `window.h:30-31`
    /// of v1.1.37.02): the text has to be the `A` and nothing else, and
    /// nothing else may be asked for -- a text or an exclusion makes the
    /// `A` a plain title.
    ///
    /// Measured on the binary of v1.1.37.02: the `WinExist("A")` answers
    /// the handle of the window that has the focus (`0x170de4`). Measured
    /// on the binary of v2.0.32: the `WinExist("A")` answers the
    /// identifier of that window (`1510884`).
    pub the_active_window: bool,
}

impl WindowCriteria {
    /// Reads the criteria of the text that names a window.
    ///
    /// The text that is given is read from the left, and each criterion
    /// that is found takes the text that follows it. The title is what
    /// is before the first criterion.
    pub fn parse(text: &str) -> WindowCriteria {
        let mut criteria = WindowCriteria::default();
        let found = the_criteria_of(text);
        // The title is the text that is before the first criterion. The
        // space that separates the criterion from the text belongs to the
        // text and not to the criterion: measured on the binary of
        // v2.0.31, one space more in the text makes the
        // title not match the window.
        let first = match found.first() {
            Some((at, _)) => *at,
            None => {
                criteria.title = text.to_string();
                criteria.the_active_window = the_criteria_name_the_active_window(&criteria);
                return criteria;
            }
        };
        let mut title_end = first;
        if title_end > 0 && text.as_bytes()[title_end - 1] == b' ' {
            title_end -= 1;
        }
        criteria.title = text[..title_end].to_string();
        // Each criterion has the text that comes after it, and the text
        // ends where the criterion that comes next begins.
        for (number, (at, name)) in found.iter().enumerate() {
            let start = at + name.len();
            let mut end = match found.get(number + 1) {
                Some((next, _)) => *next,
                None => text.len(),
            };
            // The space that separates the value from the criterion that
            // comes next is not part of the value. The space that is at
            // the end of the text is part of the value: measured on the
            // binary of v2.0.31, `ahk_class Notepad `
            // does not find the window of the `Notepad`.
            if number + 1 < found.len() {
                while end > start && text.as_bytes()[end - 1].is_ascii_whitespace() {
                    end -= 1;
                }
            }
            let value = text[start..end].trim_start().to_string();
            match *name {
                "ahk_class" => criteria.class = Some(value),
                "ahk_exe" => criteria.exe = Some(value),
                "ahk_id" => criteria.id = Some(value),
                "ahk_pid" => criteria.pid = Some(value),
                _ => {}
            }
        }
        criteria
    }

    /// Whether the window answers the criteria.
    ///
    /// The class and the executable and the identifier and the process
    /// are asked only when the text named them. The title is asked only
    /// when there is one, and with the mode that the script has set.
    ///
    /// The target that is the window that has the focus is answered by the
    /// flag of the window: the title `A` alone is that window, and not a
    /// title that a window begins with.
    ///
    /// A criterion whose value is not a number names no window: measured
    /// on the binary of v2.0.31, `ahk_id abc` does not
    /// answer a window, and it does not answer every window either.
    ///
    /// WHY the allowance: the window is asked one thing at a time, and the
    /// callers do not have the same shape of window: the runtime has the
    /// `WindowInfo` and the desktop of the Windows machine has its own.
    #[allow(clippy::too_many_arguments)]
    pub fn matches(
        &self,
        title: &str,
        class: &str,
        exe: &str,
        id: i64,
        pid: i64,
        active: bool,
        mode: TitleMatch,
    ) -> Result<bool, String> {
        // The title `A` alone is the window that has the focus.
        if self.the_active_window {
            return Ok(active);
        }
        if let Some(wanted) = &self.class {
            if wanted.is_empty() || wanted != class {
                return Ok(false);
            }
        }
        if let Some(wanted) = &self.exe {
            if wanted.is_empty() || !wanted.eq_ignore_ascii_case(exe) {
                return Ok(false);
            }
        }
        if let Some(wanted) = &self.id {
            match the_number_of(wanted) {
                Some(number) if number == id => {}
                _ => return Ok(false),
            }
        }
        if let Some(wanted) = &self.pid {
            match the_number_of(wanted) {
                Some(number) if number == pid => {}
                _ => return Ok(false),
            }
        }
        if self.title.is_empty() {
            return Ok(true);
        }
        mode.matches(title, &self.title)
    }
}

/// Whether the title is the `A` that names the window that has the focus.
///
/// The `A` alone is that window, in the v1 and in the v2. A title that
/// carries a criterion beside it is a plain title: the `A ahk_class X` is
/// the title `A` and the class `X`, and not the window that has the focus.
pub fn the_title_is_the_active_window(title: &str) -> bool {
    title == "A" || title == "a"
}

/// Whether the criteria name the window that has the focus.
///
/// The `A` alone is that window, in the v1 and in the v2. A title that
/// carries a criterion beside it is a plain title: the `A ahk_class X` is
/// the title `A` and the class `X`, and not the window that has the focus.
fn the_criteria_name_the_active_window(criteria: &WindowCriteria) -> bool {
    the_title_is_the_active_window(&criteria.title)
        && criteria.class.is_none()
        && criteria.exe.is_none()
        && criteria.id.is_none()
        && criteria.pid.is_none()
}

/// The criteria of the text, and where each one begins.
///
/// The name of the criterion is at the start of the text or after a
/// space: the text `xxahk_class` is a title and not a criterion. The
/// name is read without the case: measured on the binary of v2.0.31 `AHK_CLASS Notepad` finds the same window that
/// `ahk_class Notepad` finds.
fn the_criteria_of(text: &str) -> Vec<(usize, &'static str)> {
    let mut found = Vec::new();
    for (at, _) in text.char_indices() {
        if at != 0 && text.as_bytes()[at - 1] != b' ' {
            continue;
        }
        for name in THE_NAMES {
            if text[at..].len() >= name.len()
                && text[at..at + name.len()].eq_ignore_ascii_case(name)
            {
                found.push((at, name));
                break;
            }
        }
    }
    found
}

/// The number of the text, or `None` when the text is not a number.
///
/// The number may be written in hexadecimal with a `0x` in front of it.
/// Measured on the binary of v2.0.31: `ahk_id 0x421812`
/// names the same window that `ahk_id 4331538` names.
///
/// The rule is the one of the v1 (source `util.h:464-467` of v1.1.37.02,
/// `ATOU64`): the number is hexadecimal when it starts with a `0x`, and it
/// is decimal otherwise.
pub fn the_number_of(text: &str) -> Option<i64> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        return i64::from_str_radix(hex, 16).ok();
    }
    text.parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn um_texto_sem_criterio_e_o_titulo() {
        let criteria = WindowCriteria::parse("the title of the window");
        assert_eq!(criteria.title, "the title of the window");
        assert_eq!(criteria.class, None);
        assert_eq!(criteria.exe, None);
        assert_eq!(criteria.id, None);
        assert_eq!(criteria.pid, None);
    }

    #[test]
    fn a_classe_e_lida_e_o_titulo_e_o_que_sobra() {
        let criteria = WindowCriteria::parse("ahk_class Notepad");
        assert_eq!(criteria.class.as_deref(), Some("Notepad"));
        assert_eq!(criteria.title, "");
    }

    #[test]
    fn o_espaco_entre_o_nome_e_o_valor_pode_faltar() {
        let criteria = WindowCriteria::parse("ahk_classNotepad");
        assert_eq!(criteria.class.as_deref(), Some("Notepad"));
        assert_eq!(criteria.title, "");
    }

    #[test]
    fn o_valor_do_criterio_tem_os_espacos_do_titulo() {
        // Medido no binario do v2.0.31: a classe
        // `ahk Test Class` eAchada por `ahk_class ahk Test Class`.
        let criteria = WindowCriteria::parse("ahk_class ahk Test Class");
        assert_eq!(criteria.class.as_deref(), Some("ahk Test Class"));
        assert_eq!(criteria.title, "");
    }

    #[test]
    fn o_valor_para_onde_o_criterio_que_vem_depois_comeca() {
        let criteria = WindowCriteria::parse("ahk_class ahk Test Class ahk_pid 42");
        assert_eq!(criteria.class.as_deref(), Some("ahk Test Class"));
        assert_eq!(criteria.pid.as_deref(), Some("42"));
    }

    #[test]
    fn o_titulo_que_esta_antes_do_criterio_e_ido() {
        let criteria = WindowCriteria::parse("the title ahk_class Notepad");
        assert_eq!(criteria.class.as_deref(), Some("Notepad"));
        assert_eq!(criteria.title, "the title");
    }

    #[test]
    fn o_espaco_que_separa_o_titulo_do_criterio_e_do_titulo() {
        // Measured on the binary of v2.0.31: one space
        // more makes the title not match the window.
        let criteria = WindowCriteria::parse("the title  ahk_class Notepad");
        assert_eq!(criteria.title, "the title ");
    }

    #[test]
    fn o_nome_do_criterio_e_lido_sem_a_caixa() {
        // Medido no binario do v2.0.31: `AHK_CLASS
        // Notepad` acha a mesma janela que `ahk_class Notepad`.
        let criteria = WindowCriteria::parse("AHK_CLASS Notepad");
        assert_eq!(criteria.class.as_deref(), Some("Notepad"));
        assert_eq!(criteria.title, "");
    }

    #[test]
    fn o_nome_do_criterio_precisa_de_uma_palavra_antes() {
        let criteria = WindowCriteria::parse("xxahk_class Notepad");
        assert_eq!(criteria.class, None);
        assert_eq!(criteria.title, "xxahk_class Notepad");
    }

    #[test]
    fn um_numero_que_nao_e_numero_nao_casa_com_nenhuma_janela() {
        let criteria = WindowCriteria::parse("ahk_id abc");
        assert!(!criteria
            .matches("the title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
    }

    #[test]
    fn o_executavel_e_lido() {
        let criteria = WindowCriteria::parse("ahk_exe notepad.exe");
        assert_eq!(criteria.exe.as_deref(), Some("notepad.exe"));
        assert_eq!(criteria.title, "");
    }

    #[test]
    fn o_identificador_e_lido_em_decimal_e_em_hexadecimal() {
        assert_eq!(
            the_number_of(WindowCriteria::parse("ahk_id 4331538").id.as_deref().unwrap()),
            Some(4331538)
        );
        assert_eq!(
            the_number_of(WindowCriteria::parse("ahk_id 0x421812").id.as_deref().unwrap()),
            Some(4331538)
        );
        assert_eq!(
            the_number_of(WindowCriteria::parse("ahk_id 0X421812").id.as_deref().unwrap()),
            Some(4331538)
        );
    }

    #[test]
    fn o_processo_e_lido() {
        let criteria = WindowCriteria::parse("ahk_pid 32808");
        assert_eq!(the_number_of(criteria.pid.as_deref().unwrap()), Some(32808));
    }

    #[test]
    fn o_espaco_que_fica_no_fim_do_texto_e_do_valor() {
        // Measured on the binary of v2.0.31: `ahk_class
        // Notepad ` does not find the window of the `Notepad`.
        let criteria = WindowCriteria::parse("ahk_class Notepad ");
        assert_eq!(criteria.class.as_deref(), Some("Notepad "));
    }

    #[test]
    fn um_criterio_sem_valor_nao_nomeia_nenhuma_janela() {
        let criteria = WindowCriteria::parse("ahk_class");
        assert!(!criteria
            .matches("the title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
        let criteria = WindowCriteria::parse("ahk_exe");
        assert!(!criteria
            .matches("the title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
    }

    #[test]
    fn dois_criterios_da_mesma_especie_o_ultimo_e_o_que_conta() {
        let criteria = WindowCriteria::parse("ahk_class zzz ahk_class Notepad");
        assert_eq!(criteria.class.as_deref(), Some("Notepad"));
    }

    #[test]
    fn a_classe_compara_a_caixa() {
        let criteria = WindowCriteria::parse("ahk_class Notepad");
        assert!(criteria
            .matches("the title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
        assert!(!criteria
            .matches("the title", "notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
    }

    #[test]
    fn o_executavel_compara_sem_a_caixa() {
        let criteria = WindowCriteria::parse("ahk_exe NOTEPAD.EXE");
        assert!(criteria
            .matches("the title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Exact)
            .unwrap());
    }

    #[test]
    fn o_identificador_e_o_processo_sao_perguntados() {
        let criteria = WindowCriteria::parse("ahk_id 7 ahk_pid 9");
        assert!(criteria
            .matches("the title", "Notepad", "notepad.exe", 7, 9, false, TitleMatch::Exact)
            .unwrap());
        assert!(!criteria
            .matches("the title", "Notepad", "notepad.exe", 7, 8, false, TitleMatch::Exact)
            .unwrap());
    }

    #[test]
    fn um_texto_so_de_criterios_nao_e_um_titulo() {
        let criteria = WindowCriteria::parse("ahk_class Notepad");
        assert!(criteria
            .matches("any title", "Notepad", "notepad.exe", 1, 2, false, TitleMatch::Starts)
            .unwrap());
    }
}
