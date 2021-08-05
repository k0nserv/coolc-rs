use common::Token;
use lrlex::LexerBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    LexerBuilder::<u8>::new().process_file_in_src("cool.l")?;
    Ok(())
}
