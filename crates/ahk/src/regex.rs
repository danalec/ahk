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
//! their ranges and the named classes, the groups and the alternation,
//! the repetition with the counts, the anchors and the word boundary.
//! The look-around, the backreferences and the possessive quantifiers
//! are not carried.

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
}

/// A pattern that has been compiled.
#[derive(Debug, Clone)]
pub struct Regex {
    program: Vec<Inst>,
    /// The number of the groups, the whole one not included.
    groups: usize,
    /// Whether the letters are not to be taken as they are.
    ///
    /// v2 spells the option `(?i)` at the front of the pattern.
    ignore_case: bool,
}

impl Regex {
    /// Compile a pattern.
    ///
    /// The failure names what the engine could not carry, so that the
    /// script knows the difference between a pattern that is wrong and a
    /// pattern that is out of the subset.
    pub fn new(pattern: &str) -> Result<Regex, String> {
        let (pattern, ignore_case) = split_the_options(pattern);
        let mut compiler = Compiler::new(pattern.chars().collect(), 0, 0);
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
            ignore_case,
        })
    }

    /// The number of the groups, the whole one included.
    pub fn group_count(&self) -> usize {
        self.groups
    }

    /// The first match in the text, and where it is.
    ///
    /// The position counts from one, the way AutoHotkey counts them,
    /// and the answer is `None` when there is no match at all.
    pub fn find(&self, text: &str) -> Result<Option<Match>, String> {
        let chars: Vec<char> = text.chars().collect();
        for start in 0..=chars.len() {
            if let Some(caps) = self.exec(&chars, start)? {
                return Ok(Some(Match { caps }));
            }
        }
        Ok(None)
    }

    /// The match that starts at a place.
    pub fn find_at(&self, text: &str, start: usize) -> Result<Option<Match>, String> {
        let chars: Vec<char> = text.chars().collect();
        let start = start.min(chars.len());
        for at in start..=chars.len() {
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
        let mut current: Vec<Thread> = vec![Thread {
            pc: 0,
            sp: start,
            caps: vec![None; self.groups * 2],
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
                                    || (self.ignore_case && same_case(*actual, expected)) =>
                            {
                                thread.sp += 1;
                                thread.pc += 1;
                            }
                            _ => break,
                        },
                        Inst::Any => match chars.get(thread.sp) {
                            Some(actual) if *actual != '\n' => {
                                thread.sp += 1;
                                thread.pc += 1;
                            }
                            _ => break,
                        },
                        Inst::Class { negated, items } => match chars.get(thread.sp) {
                            Some(actual) => {
                                let holds = if self.ignore_case {
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
                            if thread.sp == 0 {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        Inst::AssertEnd => {
                            if thread.sp == chars.len() {
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
                        Inst::Match => return Ok(Some(thread.caps)),
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

    /// The text of the whole match.
    pub fn text(&self, chars: &[char]) -> String {
        let (start, end) = self.span();
        chars[start.min(chars.len())..end.min(chars.len())]
            .iter()
            .collect()
    }

    /// The text of one group, when the group took part in the match.
    pub fn group(&self, index: usize, chars: &[char]) -> Option<String> {
        let start = (*self.caps.get(index * 2)?)?;
        let end = (*self.caps.get(index * 2 + 1)?)?;
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
/// v2 spells the case-insensitive option `(?i)` at the front of the
/// pattern, and the engine carries that one and no other.
fn split_the_options(pattern: &str) -> (String, bool) {
    match pattern.strip_prefix("(?i)") {
        Some(rest) => (rest.to_string(), true),
        None => (pattern.to_string(), false),
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
}

impl Compiler {
    /// A compiler over a pattern.
    fn new(chars: Vec<char>, index: usize, groups: usize) -> Self {
        Self {
            chars,
            index,
            program: Vec::new(),
            groups,
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
            let mut branch = Compiler::new(self.chars.clone(), self.index, self.groups);
            branch.concat()?;
            branches.push(branch.program);
            self.index = branch.index;
            self.groups = branch.groups;
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
            let greedy = if self.peek() == Some('?') {
                self.index += 1;
                false
            } else {
                true
            };
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
                // usually has.
                if self.peek() == Some('?') {
                    if self.peek_next() == Some(':') {
                        self.index += 2;
                    } else {
                        return Err("the engine carries the plain groups and the ones with a colon, and not the others."
                            .into());
                    }
                }
                self.groups += 1;
                let group = self.groups;
                // The branch of the group is compiled on its own, so that
                // the jumps inside it are counted from its own start.
                let mut branch = Compiler::new(self.chars.clone(), self.index, self.groups);
                branch.alternation()?;
                if branch.peek() != Some(')') {
                    return Err("a group is missing its closing parenthesis.".into());
                }
                branch.index += 1;
                self.index = branch.index;
                self.groups = branch.groups;
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
        let error = Regex::new("(?=x)").expect_err("fails");
        assert!(error.contains("colon"), "{}", error);
    }
}
