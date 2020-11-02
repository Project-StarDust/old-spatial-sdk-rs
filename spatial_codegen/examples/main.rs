extern crate spatial_codegen;

fn main() -> () {
    let schema = spatial_codegen::parser::parse_file("../test/schema/physics.schema").expect("Can't create schema ast");
    println!("{:?}", schema);
    spatial_codegen::composer::generate_code("./generated", schema).unwrap();
}