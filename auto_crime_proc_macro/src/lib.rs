// Inspired by https://github.com/m-ou-se/auto-import/blob/main/src/lib.rs

use proc_macro::{TokenStream, TokenTree};
use std::env;
use std::fs::{create_dir_all, remove_dir_all};
use std::io::{stderr, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

const ROUILLE_SOURCE: &str = "https://github.com/joxcat/rouille.git";
const CRIME_SOURCE: &str = concat!(env!("CARGO_WORKSPACE_DIR"), ".crime");
const CRIME_MANIFEST: &str = concat!(env!("CARGO_WORKSPACE_DIR"), ".crime/Cargo.toml");

fn checkout_crimes(offset: u32) {
    remove_dir_all(CRIME_SOURCE).ok();
    create_dir_all(CRIME_SOURCE).expect("Failed to create crime source");

    let out = Command::new("git")
        .args(&[
            &format!("--work-tree={CRIME_SOURCE}/"),
            "checkout",
            &format!("HEAD~{offset}"),
            "--",
            ".",
        ])
        .output()
        .unwrap();

    if !out.status.success() {
        stderr().write_all(&out.stderr).unwrap();
        stdout().write_all(&out.stdout).unwrap();
        panic!("Failed to checkout current repository HEAD~{offset} in {CRIME_SOURCE}");
    }
}

// TODO: magic(crate_ident, specific_commit)
#[proc_macro]
pub fn magic(input: TokenStream) -> TokenStream {
    let ident = if let TokenTree::Ident(ident) = input.into_iter().next().unwrap() {
        ident
    } else {
        panic!("provide the crate name to import as the first arg");
    };

    checkout_crimes(1);
    build_crime_scene();
    hook_crimes();

    TokenStream::from_str(&format!("const __{ident}: &str = \"WORK\";")).unwrap()
}

fn build_crime_scene() {
    if !Path::new("s").exists() {
        // TODO: Release only if required
        let out = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "-p",
                "rouille",
                "--manifest-path",
                CRIME_MANIFEST,
            ])
            .output()
            .unwrap();

        if !out.status.success() {
            stderr().write_all(&out.stderr).unwrap();
            stdout().write_all(&out.stdout).unwrap();
            panic!("Failed to build crime scene in {CRIME_SOURCE}");
        }
    }
}

// Inspired by https://github.com/m-ou-se/nightly-crimes/blob/main/yolo-rustc-bootstrap/src/lib.rs
fn hook_crimes() {
    if !std::env::args_os().any(|arg| arg == "--cfg=yolo_rustc_bootstrap") {
        let trick = env::var_os("AUTO_CRIME")
            .and_then(|s| if s == "1" { Some(s) } else { None })
            .is_none();

        if trick {
            println!("\x1b[1;32m   Hijacking\x1b[m this rustc process");
            println!("\x1b[1;32m     Abusing\x1b[m proc macros");
            println!("\x1b[1;32m    Enabling\x1b[m the forbidden environment variable");
            println!("\x1b[1;32m    Tricking\x1b[m rustc");
            if let (Ok(c), Ok(v)) = (
                std::env::var("CARGO_PKG_NAME"),
                std::env::var("CARGO_PKG_VERSION"),
            ) {
                println!("\x1b[1;32m Recompiling\x1b[m {} v{}", c, v);
            } else {
                println!("\x1b[1;32m Recompiling\x1b[m your crate");
            }
        }

        let args = std::env::args_os();
        let mut args = args
            .map(|arg| {
                if arg.to_string_lossy().starts_with("rouille=") {
                    println!("\x1b[1;32mDoing crimes\x1b[m in rustc");
                    "rouille=/user_home/data/rouille/.crime/target/release/librouille.so".into()
                } else {
                    arg
                }
            }).collect::<Vec<_>>().into_iter().peekable();
		// dbg!(&args);
		
		let command = args.next().unwrap();
		if args.peek().unwrap() == "rustc" {
			args.next().unwrap();
		}
        let status = std::process::Command::new(command)
            .arg("--cfg=yolo_rustc_bootstrap")
            .args(args)
            .env("AUTO_CRIME", "1")
            .status()
            .unwrap();

        if trick && status.success() {
            println!("\x1b[1;32m    Finished\x1b[m the dirty work");
            println!("\x1b[1;32m      Hiding\x1b[m all the evidence");
            println!("\x1b[1;32m  Continuing\x1b[m as if nothing happened");
        }

        std::process::exit(status.code().unwrap_or(101));
    }

    println!("\x1b[1;32m  Destroying\x1b[m stability guarantees");
}
