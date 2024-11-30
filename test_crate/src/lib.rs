use test_macro::make_statement;

pub struct Structure;

impl Structure {
    #[make_statement]
    pub fn method(&self) {
        println!("I was here");
    }
}
