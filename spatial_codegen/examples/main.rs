use spatial_codegen::AST;

fn main() -> () {
    let schema = AST::from("../schema");
    println!("{:#?}", schema);
    let result = schema.generate("../test/src/generated");
    println!("{:#?}", result);
}
