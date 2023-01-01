fn main() {
    let schema = recipe_graphql::build_schema().finish().sdl();
    std::fs::write("schema.graphql", schema).unwrap();
}
