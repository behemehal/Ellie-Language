#![allow(warnings)]
#![feature(drain_filter)]
#![feature(let_chains)]
use core::alloc::Layout;

extern crate alloc;
pub mod utils;
pub mod mapper;
pub mod runtime;
pub mod collectors;

use std::env;
use std::path::Path;
use std::{io::Read, fs};
use fs::File;

fn main() {
    if env::args().any(|x| x == "--v") || env::args().any(|x| x == "--version") {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("v{}", VERSION);
    } else if env::args().any(|x| x == "--h") ||env::args().any(|x| x == "--help") {
        println!("Usage: ellie [options] [file path]");
        println!("Options:");
        println!("\t--version           || --v : Show Version");
        println!("\t--help              || --h : Show Help");
        println!("\t--enable-fs         || --df  : Enable file system access");
        println!("\t--enable-network    || --en  : Enable network access");
    } else {
        let args = env::args().collect::<Vec<String>>().drain(1..).collect::<Vec<String>>();
        if args.len() == 0 {
            println!("No file present\n--h for help");
        } else {
            let file_args = args.clone().drain_filter(|x| x.contains(".")).collect::<Vec<String>>();
            let file_arg_check = file_args.first();

            if file_arg_check != None {
                let file_arg = file_arg_check.unwrap();
                let file = Path::new(&file_arg.clone());
                let mut file_content = Vec::new();
                let file = File::open(&file_arg.clone());
                if file.is_err() {
                        println!("File not found ~{}", &file_arg.clone());
                } else {
                        file.unwrap().read_to_end(&mut file_content).expect("Unable to read");
                        let code_string = String::from_utf8(file_content);
                        if code_string.is_err() {
                            println!("Unable to read file ~{}", file_arg.clone())
                        } else {
                            let code = code_string.unwrap();
                            let syntax = mapper::map(code.clone());
    
                            let map = env::args().any(|x| x == "--map");
                            let map_errors = env::args().any(|x| x == "--map-errors");
                            if map {
                                println!("{:#?}", syntax.items);
                            } else if map_errors {
                                if syntax.syntax_errors.len() != 0 {
                                    for error in syntax.syntax_errors.clone() {
                                        println!("+{}:{}*", self.pos.0+1, self.pos.1+1);
                                        //println!("-{}", utils::get_line(code.clone(), syntax.syntax_errors[0].pos.line));
                                        //println!("-{}", utils::arrow(syntax.syntax_errors[0].pos.colmn as usize));
                                        println!("-{}*", error.message);
                                    }
                                } else {
                                    println!("no_error");
                                }
                            } else {
                                if syntax.syntax_errors.len() != 0 {
                                    println!("{}:{}:{}", file_arg.clone(), syntax.syntax_errors[0].pos.line, syntax.syntax_errors[0].pos.colmn);
                                    println!("{}", utils::get_line(code.clone(), syntax.syntax_errors[0].pos.line));
                                    println!("{}", utils::arrow(syntax.syntax_errors[0].pos.colmn as usize));
                                    println!("{}", syntax.syntax_errors[0].message);
                                } else {
                                    //println!("{:#?}", syntax);
                                    let raw_source = env::args().any(|x| x == "--raw-compile");
                                    if raw_source {
                                        //let mut wraw = File::create("compiled.wraw").expect("Unable to create file");
                                        //let serialized = serde_json::to_string(&point).unwrap();
                                        //for i in &syntax.clone().items {                                                                                                                                                                  
                                        //    write!(wraw, "{:?}", i);                                                                                                                         
                                        //} 
                                        println!("Pre-compiled raw generation not supported yet");
                                    } else {
                                        runtime::run(syntax.items, runtime::types::RuntimeOptions {
                                            allow_functions: true,
                                            allow_variables: true,
                                            allow_loop: true,
                                            global_variables: Vec::new(),
                                            global_functions: Vec::new()
                                        });
                                        //println!("compiled: {:#?}", syntax)
                                    }
                                }
                            }
                            //if syntax.message != "false".to_string() {
        
                            //} else {
                            //    let compiled = mapper::map(code.clone());
                            //    if compiled.errors.len() != 0 {
                            //        let error  = &compiled.errors.clone()[0];
                            //        println!("{}:{}:{}", error.file, error.pos.line, error.pos.colmn);
                            //        println!("{}", error.line);
                            //        println!("{}", lib::arrow((error.pos.colmn as usize) + 1));
                            //        println!("{}", error.message);
                            //    } else {
                            //        println!("{:#?}", compiled);
                            //    }
                            //}
                        }
                }
            } else {
                println!("File not found ~{}", args[1]);
            }
        }
    }
    // NzUxNDY4MTQyNTcxNTUyODU5.X1JhPg.dzMk6RqnqQ_DZUVT7p5yfCccOOM
}
