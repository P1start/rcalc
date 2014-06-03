#![crate_id = "rcalc"]
#![crate_type = "bin"]
#![feature(default_type_params, globs, macro_rules)]

//! Polish notation calculator.

extern crate libc;
extern crate collections;

use libc::c_char;
use std::c_str::CString;
use calc::{eval, Environment};
use calc::common::help;
use calc::pretty::pretty_print;
use collections::HashMap;

pub mod calc;

#[link(name = "linenoise")]
extern {
    fn linenoise(p: *c_char) -> *c_char;
    fn linenoiseHistoryAdd(l: *c_char);
}

///Takes a reference to a string for use as a prompt, and returns an option.
///On failure it returns None, which may be the case for ^D or ^C.
pub fn rust_readline(prompt: &str) -> Option<String> {
    if prompt.len() == 0 { 
        return None
    }

    let c_prompt = prompt.to_c_str();

    c_prompt.with_ref(|c_buf| {
        unsafe {
            let ret_str = CString::new(linenoise(c_buf), true);
            if ret_str.is_not_null() {
                ret_str.as_str().map(|ret_str| ret_str.to_str())
            } else {
                None
            }
        }
    })
}

///Adds a string to a history buffer for use by readline. Does not
///take zero length strings.
pub fn rust_add_history(line: &str) {
    if line.len() == 0 {
        return
    }

    let c_line = line.to_c_str();
    c_line.with_ref(|c_line| {
        unsafe {
            linenoiseHistoryAdd(c_line);
        }
    });
}

fn main() {

    //env will hold all user defined variables and functions in hashmaps,
    //to be looked up when called. They're in the main function for
    //persistence.
    let mut env = Environment {
        vars: HashMap::new(),
        funs: HashMap::new()
    };

    loop {
        let expr = match rust_readline(">>> ") {
            Some(val)   => { val.to_str() }
            None        => { continue }
        };
        rust_add_history(expr.as_slice());

        let help_exit_or_eval: Vec<&str> = expr.as_slice().words().collect();
        if help_exit_or_eval.len() == 0 {
            continue
        }

        let result = match help_exit_or_eval.as_slice()[0] {
            "exit" | "(exit" | "(exit)" => { break },

            "help" | "(help" | "(help)" => {
                help(help_exit_or_eval.slice_from(1));
                continue;
            },

            "(" => {
                if help_exit_or_eval.len() >= 2 {
                    match help_exit_or_eval.as_slice()[1] {
                        "exit" | "exit)"    => { break },
                        "help" | "help)"    => {
                            help(help_exit_or_eval.slice_from(2));
                            continue;
                        },
                        _   => eval(expr.as_slice().trim(), &mut env),
                    }
                }
                else {
                    eval(expr.as_slice().trim(), &mut env)
                }
            },

            _   => eval(expr.as_slice().trim(), &mut env)
        };

        pretty_print(&result, &env);
    }
}
