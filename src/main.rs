mod lexer;
#[cfg(test)]
mod tests;

use lexer::Lexer;

fn main() {
    let input = r#"
        // Define a simple function
        fn add(a: int, b: int) -> int {
            return a + b;
        }

        // Test the function
        let x = 42;
        let y = 13;
        let result = add(x, y);
        print result;
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("Input Program:\n{}\n", input);
    println!("Tokens:");
    for token in tokens {
        println!(
            "Line {:<3} Col {:<3} | {:<12} | {:?}",
            token.line, token.column, token.lexeme, token.token
        );
    }
}
