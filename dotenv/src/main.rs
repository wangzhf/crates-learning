extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use std::env;

fn main() {
    // use_method();

    use_macro();
}

fn use_method() {
    dotenv::from_path(".env").ok();

    for (key, value) in env::vars() {
        //        if key == "name".to_string() {
        //            println!("{} : {}", key, value);
        //        }

        println!("{}: {}", key, value);
    }
}

// dotenv! 只加载默认.env文件中的配置
fn use_macro() {
    // println!("macro get value: {}", dotenv!("dotenv/.env"));
}
