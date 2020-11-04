extern crate spatial_codegen;

fn main() -> () {
    let schema = spatial_codegen::parse_folder("../schema");
    println!("{:#?}", schema);
    let result = spatial_codegen::composer::generate_code("../test/src/generated", schema);
    println!("{:#?}", result);
}
