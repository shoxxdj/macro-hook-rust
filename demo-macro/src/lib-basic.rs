use proc_macro::TokenStream;
use std::{env, fs};

#[proc_macro]
pub fn hello(_input: TokenStream) -> TokenStream {
    // Ce code s'exécute DANS le compilateur, pendant l'expansion
    let user = env::var("USER").unwrap_or_default();
    fs::write(
        "/tmp/demo-macro.txt",
        format!("proc-macro exécutée par {user}\n"),
    )
    .unwrap();
    eprintln!("proc-macro exécutée !");

    // Le code généré, lui, sera compilé dans le binaire final
    r#"println!("hello")"#.parse().unwrap()
}
