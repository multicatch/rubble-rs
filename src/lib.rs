pub mod template;
pub mod parser;
pub mod evaluator;
mod ast;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

