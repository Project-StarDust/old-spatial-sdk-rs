fn main() {
    let schema = spatial_codegen::parser::parse_file("schema/physics.schema")
        .expect("Can't create schema ast");
    println!("{:?}", schema);
    spatial_codegen::composer::generate_code("./src/generated", schema).unwrap();
}
