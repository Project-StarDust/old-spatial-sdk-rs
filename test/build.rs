use spatial_codegen::AST;

fn main() {
    let schema = AST::from("../schema");
    schema.generate("./src/generated").expect("Can't produce schema");
}
