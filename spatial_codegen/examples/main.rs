extern crate spatial_codegen;

fn main() -> () {
    let schema = spatial_codegen::parse_folder("../test/schema");
    println!("{:#?}", schema);
}
