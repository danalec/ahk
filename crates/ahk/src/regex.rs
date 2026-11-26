//! A small regular expression engine.
//!
//! AutoHotkey uses PCRE. The language crate has to keep building for a
//! target that has no `std`, and there is no PCRE there: the engine is
//! here instead, a backtracking matcher over the subset of PCRE that the
//! v2 documentation names. The part that is not carried is named in the
//! error, so that a script knows what it asked for and does not get a
//! wrong answer instead.
//!
//! The subset is: the literals, the dot, the character classes with
//! their ranges and the named classes, the groups -- plain, non-capturing,
//! and named, in the three spellings of PCRE: `(?<name>...)`, `(?'name'...)`
//! and `(?P<name>...)` -- and the alternation, the repetition with the
//! counts, the anchors and the word boundary. The look-around, the
//! backreferences and the possessive quantifiers are not carried.

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::mem;

/// The most steps one match may take.
///
/// The budget sits beside the step budget of the runtime: a pattern
/// that runs away must not take the box with it.
pub const MAX_STEPS: usize = 200_000;

/// One item of a character class.
#[derive(Debug, Clone)]
enum ClassItem {
    /// A single character.
    Single(char),
    /// A range of characters, both ends included.
    Range(char, char),
    /// One of the named classes.
    Named(Named),
}

/// The named classes of a pattern.
#[derive(Debug, Clone, Copy)]
enum Named {
    /// The digits.
    Digit,
    /// The letters, the digits and the underscore.
    Word,
    /// The white space.
    Space,
}

impl Named {
    /// Whether the character is in the class.
    fn holds(self, character: char) -> bool {
        match self {
            Named::Digit => character.is_ascii_digit(),
            Named::Word => character.is_alphanumeric() || character == '_',
            Named::Space => character.is_whitespace(),
        }
    }
}

/// One instruction of the program.
#[derive(Debug, Clone)]
enum Inst {
    /// Match one character.
    Char(char),
    /// Match any character but the end of the line.
    Any,
    /// Match one character of a class.
    Class {
        negated: bool,
        items: Vec<ClassItem>,
    },
    /// Try the first branch, and the second when the first fails.
    Split(usize, usize),
    /// Go on.
    Jmp(usize),
    /// Remember where the thread is, for a group.
    Save(usize),
    /// The start of the text.
    AssertStart,
    /// The end of the text.
    AssertEnd,
    /// A word boundary, or the absence of one.
    AssertBoundary(bool),
    /// Change matching options (`(?i:...)`, `(?-i:...)`, `(?m:...)`, `(?s:...)`).
    SetOptions(Options),
    /// Zero-width lookahead (`(?=...)`, `(?!...)`) or lookbehind (`(?<=...)`, `(?<!...)`).
    LookAround {
        positive: bool,
        behind: bool,
        sub: Box<Regex>,
    },
    /// The match is complete.
    Match,
}

/// One thread of the backtracking matcher.
#[derive(Debug, Clone)]
struct Thread {
    /// Where the thread is in the program.
    pc: usize,
    /// Where the thread is in the text.
    sp: usize,
    /// The places the groups started and ended.
    caps: Vec<Option<usize>>,
    /// Active matching options for this thread.
    options: Options,
}

/// The options of a regular expression.
///
/// AutoHotkey spells options ahead of the pattern with a closing parenthesis
/// (`i)abc`, `im)^foo`, `is)a.*b`, `O)foo`, `P)foo`), and PCRE also allows
/// leading inline flags such as `(?i)`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub ignore_case: bool,
    pub multiline: bool,
    pub dotall: bool,
    pub ungreedy: bool,
    pub anchored: bool,
    pub object_mode: bool,
    pub position_mode: bool,
}

/// A pattern that has been compiled.
#[derive(Debug, Clone)]
pub struct Regex {
    program: Vec<Inst>,
    /// The number of the groups, the whole one not included.
    groups: usize,
    /// The name of each group, by the number of it; the whole match has
    /// none.
    names: Vec<Option<String>>,
    /// The options parsed from the front of the pattern.
    options: Options,
}

impl Regex {
    /// Compile a pattern.
    ///
    /// The failure names what the engine could not carry, so that the
    /// script knows the difference between a pattern that is wrong and a
    /// pattern that is out of the subset.
    pub fn new(pattern: &str) -> Result<Regex, String> {
        let (pattern, options) = split_the_options(pattern);
        let mut compiler = Compiler::new(
            pattern.chars().collect(),
            0,
            0,
            vec![None],
            options,
        );
        compiler.compile()?;
        // The whole match is a group too, and it is the first one.
        let groups = compiler.groups + 1;
        // The whole match is a group of its own: it starts at the place
        // the save sits, and ends where the second save is.
        let mut program = vec![Inst::Save(0)];
        program.extend(shift_block(&compiler.program, 1));
        program.push(Inst::Save(1));
        program.push(Inst::Match);
        Ok(Regex {
            program,
            groups,
            names: compiler.names,
            options,
        })
    }

    /// The options parsed from the pattern.
    pub fn options(&self) -> Options {
        self.options
    }

    /// The number of the groups, the whole one included.
    pub fn group_count(&self) -> usize {
        self.groups
    }

    /// The name of a group, by the number of it.
    ///
    /// The whole match has no name, and neither has a group that the
    /// pattern did not name.
    pub fn name_of(&self, group: usize) -> Option<&str> {
        self.names.get(group)?.as_deref()
    }

    /// The number of the group a name belongs to.
    ///
    /// The name is matched the way PCRE matches it: without regard to the
    /// case. A pattern with two groups of the same name answers the first
    /// one, which is the one PCRE numbers first.
    pub fn group_number(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|candidate| {
            candidate
                .as_deref()
                .map(|candidate| candidate.eq_ignore_ascii_case(name))
                .unwrap_or(false)
        })
    }

    /// The first match in the text, and where it is.
    ///
    /// The position counts from one, the way AutoHotkey counts them,
    /// and the answer is `None` when there is no match at all.
    pub fn find(&self, text: &str) -> Result<Option<Match>, String> {
        self.find_at(text, 0)
    }

    /// The match that starts at a place.
    pub fn find_at(&self, text: &str, start: usize) -> Result<Option<Match>, String> {
        let chars: Vec<char> = text.chars().collect();
        let start = start.min(chars.len());
        let end = if self.options.anchored {
            start
        } else {
            chars.len()
        };
        for at in start..=end {
            if let Some(caps) = self.exec(&chars, at)? {
                return Ok(Some(Match { caps }));
            }
        }
        Ok(None)
    }

    /// The match of the whole text, when the pattern covers it.
    pub fn matches_whole(&self, text: &str) -> Result<bool, String> {
        let chars: Vec<char> = text.chars().collect();
        let Some(caps) = self.exec(&chars, 0)? else {
            return Ok(false);
        };
        Ok(caps[1] == Some(chars.len()))
    }

    /// Run the program from a place in the text.
    fn exec(&self, chars: &[char], start: usize) -> Result<Option<Vec<Option<usize>>>, String> {
        self.exec_with_end(chars, start, None)
    }

    /// Run the program from `start`, optionally requiring the match to end at `required_end`.
    fn exec_with_end(
        &self,
        chars: &[char],
        start: usize,
        required_end: Option<usize>,
    ) -> Result<Option<Vec<Option<usize>>>, String> {
        let mut current: Vec<Thread> = vec![Thread {
            pc: 0,
            sp: start,
            caps: vec![None; self.groups * 2],
            options: self.options,
        }];
        let mut next: Vec<Thread> = Vec::new();
        let mut steps = 0usize;
        loop {
            while let Some(mut thread) = current.pop() {
                loop {
                    steps += 1;
                    if steps > MAX_STEPS {
                        return Err("the pattern took too long to match.".into());
                    }
                    let instruction = self.program[thread.pc].clone();
                    match instruction {
                        Inst::Char(expected) => match chars.get(thread.sp) {
                            Some(actual)
                                if *actual == expected
                                    || (thread.options.ignore_case
                                        && same_case(*actual, expected)) =>
                            {
                                thread.sp += 1;
                                thread.pc += 1;
                            }
                            _ => break,
                        },
                        Inst::Any => match chars.get(thread.sp) {
                            Some(actual) if thread.options.dotall || *actual != '\n' => {
                                thread.sp += 1;
                                thread.pc += 1;
                            }
                            _ => break,
                        },
                        Inst::Class { negated, items } => match chars.get(thread.sp) {
                            Some(actual) => {
                                let holds = if thread.options.ignore_case {
                                    class_holds_folded(&items, *actual)
                                } else {
                                    class_holds(&items, *actual)
                                };
                                if holds != negated {
                                    thread.sp += 1;
                                    thread.pc += 1;
                                } else {
                                    break;
                                }
                            }
                            None => break,
                        },
                        Inst::Split(first, second) => {
                            let mut other = thread.clone();
                            other.pc = second;
                            // The branch that is not the preferred one
                            // waits. It waits behind the ones that were
                            // made later: a repetition has to answer as
                            // many times as it can.
                            next.push(other);
                            thread.pc = first;
                        }
                        Inst::Jmp(target) => thread.pc = target,
                        Inst::Save(slot) => {
                            thread.caps[slot] = Some(thread.sp);
                            thread.pc += 1;
                        }
                        Inst::AssertStart => {
                            let ok = thread.sp == 0
                                || (thread.options.multiline && chars[thread.sp - 1] == '\n');
                            if ok {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        Inst::AssertEnd => {
                            let ok = thread.sp == chars.len()
                                || (thread.options.multiline
                                    && (chars[thread.sp] == '\n'
                                        || (chars[thread.sp] == '\r'
                                            && chars.get(thread.sp + 1) == Some(&'\n'))));
                            if ok {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        Inst::AssertBoundary(wanted) => {
                            let before = thread.sp > 0 && is_word(chars[thread.sp - 1]);
                            let after = thread.sp < chars.len() && is_word(chars[thread.sp]);
                            if (before != after) == wanted {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        Inst::SetOptions(options) => {
                            thread.options = options;
                            thread.pc += 1;
                        }
                        Inst::LookAround {
                            positive,
                            behind,
                            sub,
                        } => {
                            let mut sub_with_opts = (*sub).clone();
                            sub_with_opts.options = thread.options;
                            let matched = if !behind {
                                sub_with_opts.exec(chars, thread.sp)?.is_some()
                            } else {
                                let mut found = false;
                                for lb_start in (0..=thread.sp).rev() {
                                    if sub_with_opts
                                        .exec_with_end(chars, lb_start, Some(thread.sp))?
                                        .is_some()
                                    {
                                        found = true;
                                        break;
                                    }
                                }
                                found
                            };
                            if matched == positive {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        Inst::Match => {
                            if let Some(end) = required_end {
                                if thread.sp != end {
                                    break;
                                }
                            }
                            return Ok(Some(thread.caps));
                        }
                    }
                }
            }
            if next.is_empty() {
                return Ok(None);
            }
            mem::swap(&mut current, &mut next);
            next.clear();
        }
    }
}

/// A match of a pattern.
#[derive(Debug, Clone)]
pub struct Match {
    caps: Vec<Option<usize>>,
}

impl Match {
    /// Where the whole match starts and ends, counting from zero.
    pub fn span(&self) -> (usize, usize) {
        (self.caps[0].unwrap_or(0), self.caps[1].unwrap_or(0))
    }

    /// Where one group starts and ends, counting from zero.
    ///
    /// `None` means the group took no part in the match: the alternative
    /// that holds it did not run, or the optional group was not there.
    pub fn span_of(&self, group: usize) -> Option<(usize, usize)> {
        let start = (*self.caps.get(group * 2)?)?;
        let end = (*self.caps.get(group * 2 + 1)?)?;
        Some((start, end))
    }

    /// The text of the whole match.
    pub fn text(&self, chars: &[char]) -> String {
        let (start, end) = self.span();
        chars[start.min(chars.len())..end.min(chars.len())]
            .iter()
            .collect()
    }

    /// The text of one group, when the group took part in the match.
    pub fn group(&self, index: usize, chars: &[char]) -> Option<String> {
        let (start, end) = self.span_of(index)?;
        Some(chars[start..end].iter().collect())
    }
}

/// Whether a character is in a class.
fn class_holds(items: &[ClassItem], character: char) -> bool {
    items.iter().any(|item| match item {
        ClassItem::Single(expected) => *expected == character,
        ClassItem::Range(low, high) => character >= *low && character <= *high,
        ClassItem::Named(named) => named.holds(character),
    })
}

/// The options of a pattern, and the pattern without them.
///
/// AutoHotkey spells options ahead of the pattern with a closing parenthesis
/// (`i)abc`, `im)^foo`, `is)a.*b`, `O)foo`, `P)foo`, `` `a)foo ``), and PCRE
/// also allows leading inline flag groups such as `(?i)` or `(?im)`.
fn split_the_options(pattern: &str) -> (String, Options) {
    let mut options = Options::default();
    let mut rest = pattern;

    // AutoHotkey leading `options)pattern`: look for the first `)` before any `(`.
    if let Some(close) = rest.find(')') {
        let prefix = &rest[..close];
        if !prefix.contains('(') && is_ahk_option_prefix(prefix) {
            apply_option_flags(prefix, &mut options);
            rest = &rest[close + 1..];
        }
    }

    // Inline leading PCRE flags such as `(?i)` or `(?im)` (must not be `(?:...)`).
    while let Some(after) = rest.strip_prefix("(?") {
        let Some(close) = after.find(')') else {
            break;
        };
        let flags = &after[..close];
        if !flags.is_empty()
            && flags
                .chars()
                .all(|c| matches!(c, 'i' | 'm' | 's' | 'U' | 'x' | '-'))
        {
            apply_option_flags(flags, &mut options);
            rest = &after[close + 1..];
        } else {
            break;
        }
    }

    (rest.to_string(), options)
}

/// Whether `prefix` contains only valid AutoHotkey regex option characters.
fn is_ahk_option_prefix(prefix: &str) -> bool {
    let mut chars = prefix.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '-' | 'i' | 'm' | 's' | 'x' | 'A' | 'D' | 'J' | 'U' | 'X' | 'P'
            | 'S' | 'C' | 'O' | '\n' | '\r' | '\x07' => {}
            '`' => match chars.next() {
                Some('n' | 'r' | 'a') => {}
                _ => return false,
            },
            _ => return false,
        }
    }
    true
}

/// Apply option flags (with optional `-` negation) to `options`.
fn apply_option_flags(flags: &str, options: &mut Options) {
    let mut negated = false;
    let mut chars = flags.chars();
    while let Some(c) = chars.next() {
        match c {
            '-' => negated = true,
            ' ' | '\t' | '\n' | '\r' | '\x07' => {}
            '`' => {
                let _ = chars.next();
            }
            'i' => options.ignore_case = !negated,
            'm' => options.multiline = !negated,
            's' => options.dotall = !negated,
            'U' => options.ungreedy = !negated,
            'A' => options.anchored = !negated,
            'O' => options.object_mode = !negated,
            'P' => options.position_mode = !negated,
            'D' | 'J' | 'X' | 'S' | 'C' | 'x' => {}
            _ => {}
        }
    }
}

/// Whether two characters are the same, whatever the case.
fn same_case(one: char, other: char) -> bool {
    one.eq_ignore_ascii_case(&other)
}

/// Whether a class holds a character, whatever the case.
fn class_holds_folded(items: &[ClassItem], character: char) -> bool {
    if class_holds(items, character) {
        return true;
    }
    class_holds(items, character.to_ascii_lowercase())
        || class_holds(items, character.to_ascii_uppercase())
}

/// Whether a character is one of the letters, the digits or the
/// underscore.
fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

/// The compiler of a pattern.
struct Compiler {
    /// The pattern, one character per entry.
    chars: Vec<char>,
    /// Where the compiler is.
    index: usize,
    /// The program so far.
    program: Vec<Inst>,
    /// The number of the groups, the whole one included.
    groups: usize,
    /// The name of each group, by the number of it.
    names: Vec<Option<String>>,
    /// Active matching options (`i`, `m`, `s`, `U`).
    options: Options,
}

impl Compiler {
    /// A compiler over a pattern.
    fn new(
        chars: Vec<char>,
        index: usize,
        groups: usize,
        names: Vec<Option<String>>,
        options: Options,
    ) -> Self {
        Self {
            chars,
            index,
            program: Vec::new(),
            groups,
            names,
            options,
        }
    }
}

impl Compiler {
    /// The character that comes next.
    fn peek(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    /// The character that comes after the one that comes next.
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    /// Compile the whole pattern.
    fn compile(&mut self) -> Result<(), String> {
        self.alternation()?;
        if self.index < self.chars.len() {
            return Err(format!(
                "the pattern has {:?} left over",
                self.chars[self.index]
            ));
        }
        Ok(())
    }

    /// `branch ('|' branch)*`
    ///
    /// The branches are compiled one by one and put together afterwards:
    /// the layout of an alternation is the one of a choice, and it is
    /// easier to lay out than to write.
    fn alternation(&mut self) -> Result<(), String> {
        let mut branches: Vec<Vec<Inst>> = Vec::new();
        loop {
            let mut branch = Compiler::new(
                self.chars.clone(),
                self.index,
                self.groups,
                self.names.clone(),
                self.options,
            );
            branch.concat()?;
            branches.push(branch.program);
            self.index = branch.index;
            self.groups = branch.groups;
            self.names = branch.names;
            if self.peek() == Some('|') {
                self.index += 1;
                continue;
            }
            break;
        }
        let assembled = assemble(branches);
        let base = self.program.len();
        self.program.extend(shift_block(&assembled, base));
        Ok(())
    }

    /// The pieces of a branch, until the bar or the closing parenthesis.
    fn concat(&mut self) -> Result<(), String> {
        while let Some(character) = self.peek() {
            if character == '|' || character == ')' {
                break;
            }
            self.repeat()?;
        }
        Ok(())
    }

    /// An atom and the repetition that may follow it.
    fn repeat(&mut self) -> Result<(), String> {
        let mut atom = self.atom()?;
        loop {
            let (low, high) = match self.peek() {
                Some('*') => {
                    self.index += 1;
                    (0, None)
                }
                Some('+') => {
                    self.index += 1;
                    (1, None)
                }
                Some('?') => {
                    self.index += 1;
                    (0, Some(1))
                }
                Some('{') => match self.counted()? {
                    Some(counts) => counts,
                    None => break,
                },
                _ => break,
            };
            let mut greedy = if self.peek() == Some('?') {
                self.index += 1;
                false
            } else {
                true
            };
            if self.options.ungreedy {
                greedy = !greedy;
            }
            atom = quantify(&atom, low, high, greedy);
        }
        // The block goes in at the end of the program, so the jumps
        // inside it move with it.
        let base = self.program.len();
        self.program.extend(shift_block(&atom, base));
        Ok(())
    }

    /// The counts of a `{n}`, a `{n,}` or a `{n,m}`.
    ///
    /// A brace that does not hold a count is a literal brace, the way
    /// PCRE reads it.
    fn counted(&mut self) -> Result<Option<(u32, Option<u32>)>, String> {
        let start = self.index;
        self.index += 1;
        let mut low = String::new();
        while let Some(character) = self.peek() {
            if character.is_ascii_digit() {
                low.push(character);
                self.index += 1;
                continue;
            }
            break;
        }
        if low.is_empty() {
            self.index = start;
            return Ok(None);
        }
        let low: u32 = low
            .parse()
            .map_err(|_| "the count of a repetition is too large.".to_string())?;
        match self.peek() {
            Some('}') => {
                self.index += 1;
                Ok(Some((low, Some(low))))
            }
            Some(',') => {
                self.index += 1;
                let mut high = String::new();
                while let Some(character) = self.peek() {
                    if character.is_ascii_digit() {
                        high.push(character);
                        self.index += 1;
                        continue;
                    }
                    break;
                }
                if self.peek() != Some('}') {
                    return Err("a repetition is missing its closing brace.".into());
                }
                self.index += 1;
                if high.is_empty() {
                    Ok(Some((low, None)))
                } else {
                    let high: u32 = high
                        .parse()
                        .map_err(|_| "the count of a repetition is too large.".to_string())?;
                    if high < low {
                        return Err("the counts of a repetition are the wrong way round.".into());
                    }
                    Ok(Some((low, Some(high))))
                }
            }
            _ => {
                self.index = start;
                Ok(None)
            }
        }
    }

    /// One atom of a pattern.
    fn atom(&mut self) -> Result<Vec<Inst>, String> {
        let Some(character) = self.peek() else {
            return Ok(Vec::new());
        };
        self.index += 1;
        match character {
            '(' => {
                // The groups that do not capture are carried, because the
                // groups that do not capture are the ones a pattern
                // usually has. A group may be named, and PCRE spells the
                // name in three ways: `(?<name>`, `(?'name'` and
                // `(?P<name>`.
                let mut capturing = true;
                let mut name: Option<String> = None;
                let mut lookaround: Option<(bool, bool)> = None;
                let mut scoped_options: Option<Options> = None;
                if self.peek() == Some('?') {
                    match self.peek_next() {
                        Some(':') => {
                            self.index += 2;
                            capturing = false;
                        }
                        Some('=') => {
                            self.index += 2;
                            capturing = false;
                            lookaround = Some((true, false));
                        }
                        Some('!') => {
                            self.index += 2;
                            capturing = false;
                            lookaround = Some((false, false));
                        }
                        Some('<') if self.chars.get(self.index + 2) == Some(&'=') => {
                            self.index += 3;
                            capturing = false;
                            lookaround = Some((true, true));
                        }
                        Some('<') if self.chars.get(self.index + 2) == Some(&'!') => {
                            self.index += 3;
                            capturing = false;
                            lookaround = Some((false, true));
                        }
                        Some('<') => {
                            self.index += 2;
                            name = Some(self.group_name('>')?);
                        }
                        Some('\'') => {
                            self.index += 2;
                            name = Some(self.group_name('\'')?);
                        }
                        Some('P') if self.chars.get(self.index + 2) == Some(&'<') => {
                            self.index += 3;
                            name = Some(self.group_name('>')?);
                        }
                        Some('i' | 'm' | 's' | 'U' | 'x' | '-') => {
                            // Inline flag group `(?i:...)` or bare flag `(?i)`.
                            let mut scan = self.index + 1;
                            let mut flags = String::new();
                            while let Some(&c) = self.chars.get(scan) {
                                if matches!(c, 'i' | 'm' | 's' | 'U' | 'x' | '-') {
                                    flags.push(c);
                                    scan += 1;
                                } else {
                                    break;
                                }
                            }
                            match self.chars.get(scan) {
                                Some(':') => {
                                    let mut next_opts = self.options;
                                    apply_option_flags(&flags, &mut next_opts);
                                    scoped_options = Some(next_opts);
                                    capturing = false;
                                    self.index = scan + 1;
                                }
                                Some(')') => {
                                    apply_option_flags(&flags, &mut self.options);
                                    self.index = scan + 1;
                                    return Ok(vec![Inst::SetOptions(self.options)]);
                                }
                                _ => {
                                    return Err("the engine carries the plain groups, the ones with a colon and the named ones, and not the others."
                                        .into());
                                }
                            }
                        }
                        _ => {
                            return Err("the engine carries the plain groups, the ones with a colon and the named ones, and not the others."
                                .into())
                        }
                    }
                }
                if let Some((positive, behind)) = lookaround {
                    let mut branch = Compiler::new(
                        self.chars.clone(),
                        self.index,
                        self.groups,
                        self.names.clone(),
                        self.options,
                    );
                    branch.alternation()?;
                    if branch.peek() != Some(')') {
                        return Err("a group is missing its closing parenthesis.".into());
                    }
                    branch.index += 1;
                    self.index = branch.index;
                    self.groups = branch.groups;
                    self.names = branch.names;
                    let sub_groups = self.groups + 1;
                    let mut sub_prog = vec![Inst::Save(0)];
                    sub_prog.extend(shift_block(&branch.program, 1));
                    sub_prog.push(Inst::Save(1));
                    sub_prog.push(Inst::Match);
                    let sub = Regex {
                        program: sub_prog,
                        groups: sub_groups,
                        names: self.names.clone(),
                        options: self.options,
                    };
                    return Ok(vec![Inst::LookAround {
                        positive,
                        behind,
                        sub: Box::new(sub),
                    }]);
                }
                let group = if capturing {
                    self.groups += 1;
                    let g = self.groups;
                    if self.names.len() <= g {
                        self.names.resize(g + 1, None);
                    }
                    self.names[g] = name;
                    Some(g)
                } else {
                    None
                };
                // The branch of the group is compiled on its own, so that
                // the jumps inside it are counted from its own start.
                let inner_options = scoped_options.unwrap_or(self.options);
                let mut branch = Compiler::new(
                    self.chars.clone(),
                    self.index,
                    self.groups,
                    self.names.clone(),
                    inner_options,
                );
                branch.alternation()?;
                if branch.peek() != Some(')') {
                    return Err("a group is missing its closing parenthesis.".into());
                }
                branch.index += 1;
                self.index = branch.index;
                self.groups = branch.groups;
                self.names = branch.names;
                if let Some(opts) = scoped_options {
                    let mut block = vec![Inst::SetOptions(opts)];
                    block.extend(shift_block(&branch.program, 1));
                    block.push(Inst::SetOptions(self.options));
                    return Ok(block);
                }
                let Some(group) = group else {
                    return Ok(branch.program);
                };
                // The block of the group: the save that opens it, the
                // branch, and the save that closes it. The save is one
                // place before the branch, so the jumps move by one.
                let mut block = vec![Inst::Save(group * 2)];
                for mut instruction in branch.program {
                    match &mut instruction {
                        Inst::Split(first, second) => {
                            *first += 1;
                            *second += 1;
                        }
                        Inst::Jmp(target) => *target += 1,
                        _ => {}
                    }
                    block.push(instruction);
                }
                block.push(Inst::Save(group * 2 + 1));
                Ok(block)
            }
            '[' => self.class(),
            '.' => Ok(vec![Inst::Any]),
            '^' => Ok(vec![Inst::AssertStart]),
            '$' => Ok(vec![Inst::AssertEnd]),
            '\\' => self.escape(),
            ')' => Err("a closing parenthesis without an opening one.".into()),
            other => Ok(vec![Inst::Char(other)]),
        }
    }

    /// The name of a named group, up to the character that closes it.
    ///
    /// A name has to have at least one character in it: `(?<>...)` is not
    /// a named group.
    fn group_name(&mut self, close: char) -> Result<String, String> {
        let mut name = String::new();
        loop {
            let Some(character) = self.peek() else {
                return Err("a named group is missing its closing delimiter.".into());
            };
            self.index += 1;
            if character == close {
                break;
            }
            name.push(character);
        }
        if name.is_empty() {
            return Err("a named group has no name.".into());
        }
        Ok(name)
    }

    /// The escape of a pattern.
    fn escape(&mut self) -> Result<Vec<Inst>, String> {
        let Some(character) = self.peek() else {
            return Err("the pattern ends with a backslash.".into());
        };
        self.index += 1;
        Ok(match character {
            'd' => vec![Inst::Class { negated: false, items: vec![ClassItem::Named(Named::Digit)] }],
            'D' => vec![Inst::Class { negated: true, items: vec![ClassItem::Named(Named::Digit)] }],
            'w' => vec![Inst::Class { negated: false, items: vec![ClassItem::Named(Named::Word)] }],
            'W' => vec![Inst::Class { negated: true, items: vec![ClassItem::Named(Named::Word)] }],
            's' => vec![Inst::Class { negated: false, items: vec![ClassItem::Named(Named::Space)] }],
            'S' => vec![Inst::Class { negated: true, items: vec![ClassItem::Named(Named::Space)] }],
            'b' => vec![Inst::AssertBoundary(true)],
            'B' => vec![Inst::AssertBoundary(false)],
            'n' => vec![Inst::Char('\n')],
            'r' => vec![Inst::Char('\r')],
            't' => vec![Inst::Char('\t')],
            '0' => vec![Inst::Char('\0')],
            'x' => {
                let mut digits = String::new();
                for _ in 0..2 {
                    match self.peek() {
                        Some(digit) if digit.is_ascii_hexdigit() => {
                            digits.push(digit);
                            self.index += 1;
                        }
                        _ => break,
                    }
                }
                if digits.is_empty() {
                    return Err("the hexadecimal escape needs two digits.".into());
                }
                let code = u32::from_str_radix(&digits, 16)
                    .map_err(|_| "the hexadecimal escape is not a number.".to_string())?;
                vec![Inst::Char(char::from_u32(code).unwrap_or('\u{fffd}'))]
            }
            'p' | 'P' | '<' => {
                return Err("the engine carries the plain groups and the ones with a colon, and not the others.".into())
            }
            other => vec![Inst::Char(other)],
        })
    }

    /// A character class: `[abc]`, `[^a-z]`, `[\\d]`.
    fn class(&mut self) -> Result<Vec<Inst>, String> {
        let negated = self.peek() == Some('^');
        if negated {
            self.index += 1;
        }
        let mut items: Vec<ClassItem> = Vec::new();
        let mut first = true;
        loop {
            let Some(character) = self.peek() else {
                return Err("a character class is missing its closing bracket.".into());
            };
            if character == ']' && !first {
                self.index += 1;
                break;
            }
            first = false;
            self.index += 1;
            let low = if character == '\\' {
                let escaped = self.escape()?;
                match escaped.first() {
                    Some(Inst::Char(character)) => *character,
                    Some(Inst::Class {
                        negated,
                        items: named,
                    }) => {
                        // A named class inside a class: the items of the
                        // named class join the class.
                        if *negated {
                            return Err("a negated named class cannot sit inside a class.".into());
                        }
                        items.extend(named.iter().cloned());
                        continue;
                    }
                    _ => {
                        return Err(
                            "only a character or a named class can sit inside a class.".into()
                        );
                    }
                }
            } else {
                character
            };
            // A range: `a-z`.
            if self.peek() == Some('-')
                && self.peek_next() != Some(']')
                && self.peek_next().is_some()
            {
                self.index += 1;
                let Some(high) = self.peek() else {
                    return Err("a character class is missing its closing bracket.".into());
                };
                self.index += 1;
                let high = if high == '\\' {
                    match self.escape()?.first() {
                        Some(Inst::Char(character)) => *character,
                        _ => return Err("only a character can end a range.".into()),
                    }
                } else {
                    high
                };
                items.push(ClassItem::Range(low, high));
                continue;
            }
            items.push(ClassItem::Single(low));
        }
        Ok(vec![Inst::Class { negated, items }])
    }
}

/// Put the branches of an alternation together.
///
/// Every branch but the last gets a split in front of it, and the split
/// falls on the branch that follows when the one before it fails. The
/// branch that is last has nothing to fall back on, so a thread that
/// dies there dies.
fn assemble(branches: Vec<Vec<Inst>>) -> Vec<Inst> {
    // The place each branch starts at, and the place of the split that
    // opens it.
    let mut starts: Vec<usize> = Vec::with_capacity(branches.len());
    let mut splits: Vec<usize> = Vec::with_capacity(branches.len());
    let mut at = 0usize;
    for (index, branch) in branches.iter().enumerate() {
        if index + 1 < branches.len() {
            splits.push(at);
            at += 1;
        }
        starts.push(at);
        at += branch.len() + 1;
    }
    let end = at;

    let mut program: Vec<Inst> = Vec::new();
    for (index, branch) in branches.iter().enumerate() {
        if index + 1 < branches.len() {
            // The split takes the branch that is here first, and the one
            // that follows when it fails.
            let first = program.len() + 1;
            let second = match splits.get(index + 1) {
                Some(next) => *next,
                None => starts[index + 1],
            };
            program.push(Inst::Split(first, second));
            program.extend(shift_block(branch, program.len()));
        } else {
            program.extend(shift_block(branch, program.len()));
        }
        program.push(Inst::Jmp(end));
    }
    program
}

/// Copy a block of instructions, with the jumps inside it moved with it.
///
/// The jumps of a block are counted from the start of the block, so a
/// copy that goes somewhere else has to take its jumps along.
fn shift_block(block: &[Inst], offset: usize) -> Vec<Inst> {
    block
        .iter()
        .map(|instruction| {
            let mut instruction = instruction.clone();
            match &mut instruction {
                Inst::Split(first, second) => {
                    *first += offset;
                    *second += offset;
                }
                Inst::Jmp(target) => *target += offset,
                _ => {}
            }
            instruction
        })
        .collect()
}

/// Apply a repetition to the instructions of an atom.
fn quantify(atom: &[Inst], low: u32, high: Option<u32>, greedy: bool) -> Vec<Inst> {
    // The copies of the atom that the counts ask for.
    let mut program: Vec<Inst> = Vec::new();
    for _ in 0..low {
        let base = program.len();
        program.extend(shift_block(atom, base));
    }
    match high {
        Some(high) => {
            // The copies that may or may not be there.
            for _ in low..high {
                let split = program.len();
                program.push(Inst::Split(0, 0));
                let body = program.len();
                program.extend(shift_block(atom, body));
                let end = program.len();
                program[split] = if greedy {
                    Inst::Split(body, end)
                } else {
                    Inst::Split(end, body)
                };
            }
        }
        None => {
            // The rest, as many times as it takes.
            let split = program.len();
            program.push(Inst::Split(0, 0));
            let body = program.len();
            program.extend(shift_block(atom, body));
            program.push(Inst::Jmp(split));
            let end = program.len();
            program[split] = if greedy {
                Inst::Split(body, end)
            } else {
                Inst::Split(end, body)
            };
        }
    }
    program
}

/// The position of the first match, counting from one.
pub fn find(text: &str, pattern: &str) -> Result<Option<usize>, String> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find(text)?.map(|found| found.span().0 + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches(pattern: &str, text: &str) -> bool {
        Regex::new(pattern)
            .expect("compiles")
            .find(text)
            .expect("runs")
            .is_some()
    }

    #[test]
    fn a_literal_is_a_literal() {
        assert!(matches("abc", "xxabcxx"));
        assert!(!matches("abc", "xxabxx"));
    }

    #[test]
    fn the_dot_is_any_character() {
        assert!(matches("a.c", "abc"));
        assert!(matches("a.c", "a c"));
    }

    #[test]
    fn the_stars_repeat() {
        assert!(matches("ab*c", "ac"));
        assert!(matches("ab*c", "abbbc"));
        assert!(!matches("ab+c", "ac"));
    }

    #[test]
    fn the_question_mark_makes_it_optional() {
        assert!(matches("ab?c", "ac"));
        assert!(matches("ab?c", "abc"));
    }

    #[test]
    fn a_class_holds_the_letters_it_names() {
        assert!(matches("[abc]+", "bbb"));
        assert!(!matches("[abc]+", "xyz"));
        assert!(matches("[^abc]+", "xyz"));
    }

    #[test]
    fn a_range_holds_the_letters_between() {
        assert!(matches("[a-z]+", "hello"));
        assert!(!matches("[a-z]+", "HELLO"));
    }

    #[test]
    fn the_named_classes_hold_what_they_say() {
        assert!(matches("\\d+", "abc123"));
        assert!(!matches("\\d+", "abc"));
        assert!(matches("\\w+", "a_1"));
        assert!(matches("\\s+", "a b"));
    }

    #[test]
    fn a_group_is_captured() {
        let regex = Regex::new("(\\d+)-(\\d+)").expect("compiles");
        let found = regex.find("12-34").expect("runs").expect("a match");
        let chars: Vec<char> = "12-34".chars().collect();
        assert_eq!(found.group(1, &chars).as_deref(), Some("12"));
        assert_eq!(found.group(2, &chars).as_deref(), Some("34"));
    }

    #[test]
    fn the_alternation_takes_the_first_branch_that_holds() {
        assert!(matches("cat|dog", "a dog here"));
        assert!(matches("cat|dog", "a cat here"));
    }

    #[test]
    fn the_anchors_ask_for_the_ends() {
        assert!(matches("^abc", "abcdef"));
        assert!(!matches("^abc", "xabcdef"));
        assert!(matches("abc$", "xxabc"));
    }

    #[test]
    fn the_count_says_how_many_times() {
        assert!(matches("a{3}", "aaa"));
        assert!(!matches("a{3}", "aa"));
        assert!(matches("a{2,}", "aaaa"));
        assert!(matches("a{2,3}", "aaa"));
        assert!(!matches("a{2,3}", "a"));
    }

    #[test]
    fn the_position_counts_from_one() {
        assert_eq!(find("abc123", "\\d").expect("runs"), Some(4));
        assert_eq!(find("abc", "\\d").expect("runs"), None);
    }

    #[test]
    fn a_pattern_the_engine_does_not_carry_says_so() {
        let error = Regex::new("(?R)").expect_err("fails");
        assert!(error.contains("colon"), "{}", error);
    }

    #[test]
    fn lookahead_lookbehind_and_inline_flag_groups_work() {
        // Pattern from Gdip_TextToGraphics in Gdip_All.ahk
        let colour = Regex::new("iO)C(?!(entre|enter))([a-f\\d]+)").expect("compiles");
        assert!(colour.find("Centre").expect("runs").is_none());
        let found = colour
            .find("Centre cFF112233")
            .expect("runs")
            .expect("matches hex color");
        let chars: Vec<char> = "Centre cFF112233".chars().collect();
        // Group 1 is `(entre|enter)` inside the negative lookahead; group 2 is `([a-f\d]+)`.
        assert_eq!(found.group(2, &chars).as_deref(), Some("FF112233"));

        // Pattern from Gdip_All.ahk
        assert!(matches("^(?i:BMP|PNG|JPG)$", "png"));
        assert!(!matches("^(?i:BMP|PNG|JPG)$", "gif"));

        // Lookbehind + lookahead
        assert!(matches("(?<=a)b(?=c)", "abc"));
        assert!(!matches("(?<=a)b(?=c)", "xbc"));
        assert!(!matches("(?<=a)b(?=c)", "abx"));
    }

    #[test]
    fn ahk_options_prefix_case_insensitive_multiline_dotall_and_ungreedy() {
        assert!(matches("i)(disable|check)", "Check"));
        assert!(matches("i)(disable|check)", "DISABLE"));
        assert!(matches("im)^foo$", "bar\nFOO\nbaz"));
        assert!(matches("is)a.*b", "a\nb"));
        assert!(!matches("a.*b", "a\nb"));
        let ungreedy = Regex::new("U)a.+b").expect("compiles");
        let found = ungreedy.find("axbyb").expect("runs").expect("a match");
        let chars: Vec<char> = "axbyb".chars().collect();
        assert_eq!(found.text(&chars), "axb");
    }

    #[test]
    fn non_capturing_group_does_not_shift_group_numbers() {
        let regex = Regex::new("(?:foo)(\\d+)").expect("compiles");
        let found = regex.find("foo42").expect("runs").expect("a match");
        let chars: Vec<char> = "foo42".chars().collect();
        assert_eq!(found.group(1, &chars).as_deref(), Some("42"));
    }
}
