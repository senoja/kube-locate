use std::io::{Write, stdout, stdin};

use termion::clear;
use termion::cursor::{self, DetectCursorPos};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{self, IntoRawMode};
use termion::style;

use super::kubectl;

pub fn run() -> Result<(),&'static str> {
    let contexts = kubectl::get_contexts()?;
    let current_context = kubectl::get_context()?;

    let mut namespaces = kubectl::get_namespaces_for_context(&current_context)?;
    let mut current_namespace = kubectl::get_namespace_for_context(&current_context)?;

    // TODO: check tty
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", cursor::Hide).unwrap();

    // Setup screen location and clear line
    let (x, y) = stdout.cursor_pos().unwrap();
    write!(stdout, "{}{}", cursor::Goto(x, y), clear::AfterCursor).unwrap();

    if let Some(selected_context) = interactive_selection(&mut stdout, x, y, contexts, &current_context) {
        if selected_context != current_context {
            namespaces = kubectl::get_namespaces_for_context(&selected_context)?;
            current_namespace = kubectl::get_namespace_for_context(&selected_context)?;
        }

        if let Some(selected_namespace) = interactive_selection(&mut stdout, x, y, namespaces, &current_namespace) {
            write!(stdout, "{}", cursor::Show).unwrap();
            return kubectl::set_context_namespace(&selected_context, &selected_namespace)
        }

    }

    write!(stdout, "{}", cursor::Show).unwrap();
    Ok(())
}

fn interactive_selection(stdout: &mut raw::RawTerminal<std::io::Stdout>, x: u16, mut y: u16, items: Vec<String>, starting_item: &str) -> Option<String> {
    let mut list = DisplayList::new(items, starting_item);
    list.render(stdout);

    // Resample y when the printed list scrolls the terminal
    // TODO: cleanup casting
    if let Ok(term_size) = termion::terminal_size() {
        let adjustment: i16 = (y + list.items.len() as u16) as i16 - term_size.1 as i16;
        if adjustment > 0 {
            y -= adjustment as u16;
        }
    }

    for c in stdin().keys() {
        write!(stdout, "{}{}", cursor::Goto(x, y), clear::AfterCursor).unwrap();
        stdout.flush().unwrap();

        match c.unwrap() {
            Key::Char('j') => {
                list.position.increment();
            },
            Key::Char('k') => {
                list.position.decrement();
            },
            Key::Up => {
                list.position.decrement();
            },
            Key::Down => {
                list.position.increment();
            },
            Key::Char('\n') => {
                return Some(list.current_item());
            },
            Key::Ctrl('c') => break,
            _              => (),
        }

        list.render(stdout);

        stdout.flush().unwrap();
    }

    None
}

struct DisplayList {
    items: Vec<String>,
    position: Position,
}

impl DisplayList {
    fn new(items: Vec<String>, starting_item: &str) -> Self {
        let cur = match items.clone().iter().position(|e| e == starting_item) {
            Some(position) => position,
            None => 0,
        };

        let max = items.len() - 1;

        let position = Position{max,cur};

        DisplayList {
            items,
            position
        }
    }

    fn current_item(&self) -> String {
      self.items[self.position.cur].clone()
    }

    fn render(&self, terminal: &mut raw::RawTerminal<std::io::Stdout>) {
        for (i,e) in self.items.iter().enumerate() {
            if i == self.position.cur {
                write!(terminal, "{} {} {} \r\n", style::Invert, e, style::Reset).unwrap();
            } else {
                write!(terminal, " {} \r\n", e).unwrap();
            }
        }
    }
}

struct Position {
    max: usize,
    cur: usize
}

impl Position {
    fn increment(&mut self) {
        if self.cur == self.max {
            self.cur = 0;
        } else {
            self.cur += 1;
        }
    }

    fn decrement(&mut self) {
        if self.cur == 0 {
            self.cur = self.max;
        } else {
            self.cur -= 1;
        }
    }
}

