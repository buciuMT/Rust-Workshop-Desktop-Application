use std::{iter::Peekable, str::Chars};

use lazy_static::lazy_static;
use regex::Regex;
use slint::include_modules;

include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();
        ui.set_tfocus(false);
        ui.set_textarea(match new_input.as_str() {
            "C" => "".into(),
            "=" => {
                let res = evaluate(current_text.as_str()).into();
                let new_windows = res_window::new();
                
                res
            }
            _ => current_text + new_input.as_str(),
        });
    });
    ui.run()
}

type Num = f64;

fn uint_parse(i: &mut Peekable<Chars>) -> Option<(u32, u32)> {
    let c = i.peek();
    if c.is_some_and(|x| x.is_digit(10)) {
        let mut nr = c.unwrap().to_digit(10).unwrap();
        let mut p = 10;
        i.next();
        while let Some(c) = i.peek() {
            if c.is_digit(10) {
                nr = nr * 10 + c.to_digit(10).unwrap();
                p *= 10;
                i.next();
            }
        }
        Some((nr, p))
    } else {
        None
    }
}

fn nr_parse(n: u32, i: &mut Peekable<Chars>) -> Num {
    let res = if let Some((f, p)) = uint_parse(i) {
        (n * p + f) as Num
    } else {
        n as Num
    };
    if !i.peek().is_some_and(|x| *x == '.') {
        return res;
    }
    i.next();
    if let Some((f, p)) = uint_parse(i) {
        res + f as Num / p as Num
    } else {
        res
    }
}

fn expr_u(input: &mut Peekable<Chars>) -> Option<f64> {
    if let Some(c) = input.next() {
        match c {
            '(' => {
                let res = expr(input);
                if input.next() == Some(')') {
                    res
                } else {
                    None
                }
            }
            '-' => expr_u(input).map(|x| -x),
            '+' => expr_u(input),
            '0'..='9' => Some(nr_parse(c.to_digit(10).unwrap(), input)),
            '.' => {
                if let Some((n, p)) = uint_parse(input) {
                    Some(n as Num / p as Num)
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

fn expr_t(input: &mut Peekable<Chars>) -> Option<f64> {
    if let Some(mut lhs) = expr_u(input) {
        while let Some(c) = input.peek() {
            let c = c.clone();
            match c {
                '*' => {
                    input.next();
                    if let Some(rhs) = expr_u(input) {
                        lhs *= rhs;
                    } else {
                        return None;
                    }
                }
                '/' => {
                    input.next();
                    if let Some(rhs) = expr_u(input) {
                        lhs /= rhs;
                    } else {
                        return None;
                    }
                }
                _ => break,
            }
        }
        Some(lhs)
    } else {
        None
    }
}

fn expr_s(input: &mut Peekable<Chars>) -> Option<f64> {
    if let Some(mut lhs) = expr_t(input) {
        while let Some(c) = input.peek() {
            let c = c.clone();
            match c {
                '+' => {
                    input.next();
                    if let Some(rhs) = expr_t(input) {
                        lhs += rhs;
                    } else {
                        return None;
                    }
                }
                '-' => {
                    input.next();
                    if let Some(rhs) = expr_t(input) {
                        lhs -= rhs;
                    } else {
                        return None;
                    }
                }
                _ => break,
            }
        }
        Some(lhs)
    } else {
        None
    }
}

fn expr(input: &mut Peekable<Chars>) -> Option<f64> {
    if let Some(c) = input.peek() {
        match c {
            '(' => {
                input.next();
                let res = expr(input);
                if input.next() == Some(')') {
                    res
                } else {
                    None
                }
            }
            _ => expr_s(input),
        }
    } else {
        None
    }
}

fn evaluate(input: &str) -> String {
    let mut inp = input.chars().peekable();
    if let Some(res) = expr(&mut inp) {
        if inp.next() == None {
            format!("{}", res)
        } else {
            "Parse Err".to_string()
        }
    } else {
        "NaN".to_string()
    }
}
