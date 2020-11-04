fn main() {
    let schema = spatial_codegen::parse_folder("../schema");
    spatial_codegen::composer::generate_code("./src/generated", schema).expect("Can't produce schema");
}
