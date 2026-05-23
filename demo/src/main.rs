use rustyorm_core::query::select::Select;

fn main() {
    let query = Select::new("users")
        .column("id")
        .column("name")
        .where_eq("id", 1)
        .where_eq("name", "john");

    println!("{}", query.to_sql());
    println!("{:#?}", query.params());
}
