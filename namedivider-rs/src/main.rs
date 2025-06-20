use clap::{Parser, Subcommand};
use namedivider_rs::divider::basic_name_divider::get_basic_name_divider;
use namedivider_rs::divider::divided_name::DividedName;
use namedivider_rs::divider::gbdt_name_divider::get_gbdt_name_divider;
use namedivider_rs::divider::name_divider::NameDivider;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn divide_name(divider: &Box<dyn NameDivider>, undivided_name: &String) -> DividedName {
    divider.divide_name(undivided_name)
}

fn create_divider(mode: &String) -> Box<dyn NameDivider> {
    if mode == "basic" {
        let basic_divider =
            get_basic_name_divider(" ".to_string(), true, "kanji_feature".to_string(), false);
        Box::new(basic_divider)
    } else {
        let gbdt_divider =
            get_gbdt_name_divider(" ".to_string(), true, "kanji_feature".to_string());
        Box::new(gbdt_divider)
    }
}

fn read_file(file_path: &String) -> String {
    let path = Path::new(file_path);
    let display = path.display();
    let mut file = match File::open(file_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    s
}

#[derive(Parser)]
struct AppArg {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Name {
        undivided_name: String,
        #[clap(default_value = "basic")]
        mode: String,
    },
    File {
        undivided_name_text: String,
        #[clap(default_value = "basic")]
        mode: String,
    },
    Accuracy {
        divided_name_text: String,
        #[clap(default_value = "basic")]
        mode: String,
    },
}

fn main() {
    let cli = AppArg::parse();
    match cli.action {
        Action::Name {
            undivided_name,
            mode,
        } => {
            let divider = create_divider(&mode);
            let divided_name = divide_name(&divider, &undivided_name);
            println!(
                "{}{}{}",
                divided_name.family, divided_name.separator, divided_name.given
            );
        }
        Action::File {
            undivided_name_text,
            mode,
        } => {
            let file_contents = read_file(&undivided_name_text);
            let divider = create_divider(&mode);
            for undivided_name in file_contents.lines() {
                let divided_name = divide_name(&divider, &undivided_name.to_string());
                println!(
                    "{}{}{}",
                    divided_name.family, divided_name.separator, divided_name.given
                );
            }
        }
        Action::Accuracy {
            divided_name_text,
            mode,
        } => {
            let file_contents = read_file(&divided_name_text);
            let divider = create_divider(&mode);
            let mut total: f64 = 0.0;
            let mut ng: f64 = 0.0;
            for divided_name in file_contents.lines() {
                let divided_name_orig = divided_name.clone();
                let undivided_name = divided_name.replace(' ', "");
                let divided_name = divide_name(&divider, &undivided_name.to_string());
                let divided_name_str =
                    divided_name.family + &divided_name.separator + &divided_name.given;
                total += 1.0;
                if *divided_name_orig != divided_name_str {
                    ng += 1.0;
                    println!("{}, {}", divided_name_orig, divided_name_str)
                }
            }
            println!("{}", (1.0 - (ng / total)));
        }
    }
}
