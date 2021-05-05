mod lib;
use lib::Alias;

fn main() {
    let alias = Alias {
        name: String::from("l"),
        body: String::from("ls -l"),
    };
    println!("{}", alias.to_nu());
}
