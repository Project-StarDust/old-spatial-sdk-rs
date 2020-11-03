extern crate spatial_codegen;

fn main() -> () {
    let schema = spatial_codegen::parser::parse_folder("../test/schema");
    println!("{:#?}", schema);
}
