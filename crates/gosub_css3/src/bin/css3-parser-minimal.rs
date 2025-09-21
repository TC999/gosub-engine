// Minimal CSS3 parser for Windows - avoids gosub dependencies
use std::fs;
use std::time::Instant;
use anyhow::{bail, Result};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("Gosub CSS3 Minimal Parser")
        .version("0.1.0")
        .about("A minimal CSS3 tokenizer for Windows compilation")
        .arg(
            Arg::new("file")
                .help("The CSS file to parse")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("debug")
                .help("Enable debug output")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let file_path: String = matches.get_one::<String>("file").expect("file").to_string();
    let debug = matches.get_flag("debug");

    println!("🎨 Gosub CSS3 Parser for Windows");
    println!("📁 Reading CSS file: {}", file_path);

    // Read the CSS file
    let css_content = if file_path.starts_with("http://") || file_path.starts_with("https://") {
        println!("🌐 Fetching CSS from URL...");
        let mut response = ureq::get(&file_path).call()?;
        if response.status() != 200 {
            bail!("❌ Failed to fetch CSS. Status code: {}", response.status());
        }
        response.body_mut().read_to_string()?
    } else {
        fs::read_to_string(&file_path)?
    };

    println!("📏 File size: {} bytes", css_content.len());

    if debug {
        println!("🔍 Debug mode enabled");
    }

    // Simple CSS tokenization
    let start_time = Instant::now();
    let tokens = tokenize_css(&css_content, debug);
    let parse_time = start_time.elapsed();

    println!("⚡ Tokenization completed in {:?}", parse_time);
    println!("🎯 Found {} tokens", tokens.len());

    // Basic statistics
    let mut rule_count = 0;
    let mut property_count = 0;
    let mut comment_count = 0;

    for token in &tokens {
        match token {
            Token::Selector(_) => rule_count += 1,
            Token::Property(_, _) => property_count += 1,
            Token::Comment(_) => comment_count += 1,
            _ => {}
        }
    }

    println!("\n📊 CSS Statistics:");
    println!("   • Rules: {}", rule_count);
    println!("   • Properties: {}", property_count);
    println!("   • Comments: {}", comment_count);

    if debug && !tokens.is_empty() {
        println!("\n🔍 First 10 tokens:");
        for (i, token) in tokens.iter().take(10).enumerate() {
            println!("   {}: {:?}", i + 1, token);
        }
        if tokens.len() > 10 {
            println!("   ... and {} more tokens", tokens.len() - 10);
        }
    }

    println!("\n✅ Parsing completed successfully!");

    Ok(())
}

#[derive(Debug, Clone)]
enum Token {
    Selector(String),
    Property(String, String),
    Comment(String),
    OpenBrace,
    CloseBrace,
    Semicolon,
    Colon,
    Other(String),
}

fn tokenize_css(css: &str, debug: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = css.chars().collect();

    while i < chars.len() {
        // Skip whitespace
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        
        if i >= chars.len() {
            break;
        }

        // Parse comments
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
            let start = i;
            i += 2;
            
            // Find end of comment
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            
            if i + 1 < chars.len() {
                i += 2; // Skip */
            }
            
            let comment = chars[start..i.min(chars.len())].iter().collect::<String>();
            tokens.push(Token::Comment(comment));
            continue;
        }

        // Parse braces and punctuation
        match chars[i] {
            '{' => {
                tokens.push(Token::OpenBrace);
                i += 1;
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                i += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                i += 1;
            }
            _ => {
                // Parse other text tokens
                let start = i;
                while i < chars.len() && !chars[i].is_whitespace() && !"{}:;".contains(chars[i]) {
                    i += 1;
                }
                
                if i > start {
                    let text = chars[start..i].iter().collect::<String>();
                    
                    // Simple heuristic: if it's before {, it's likely a selector
                    // This is a very basic approach for demonstration
                    if debug {
                        println!("   Token: '{}'", text);
                    }
                    
                    tokens.push(Token::Other(text));
                }
            }
        }
    }

    // Post-process to identify selectors and properties
    let mut processed_tokens = Vec::new();
    let mut i = 0;
    
    while i < tokens.len() {
        match &tokens[i] {
            Token::Other(text) => {
                // Check if next token is {, then this is a selector
                if i + 1 < tokens.len() && matches!(tokens[i + 1], Token::OpenBrace) {
                    processed_tokens.push(Token::Selector(text.clone()));
                }
                // Check if next token is :, then this might be a property
                else if i + 1 < tokens.len() && matches!(tokens[i + 1], Token::Colon) {
                    if i + 2 < tokens.len() {
                        if let Token::Other(value) = &tokens[i + 2] {
                            processed_tokens.push(Token::Property(text.clone(), value.clone()));
                            i += 2; // Skip colon and value
                        } else {
                            processed_tokens.push(tokens[i].clone());
                        }
                    } else {
                        processed_tokens.push(tokens[i].clone());
                    }
                } else {
                    processed_tokens.push(tokens[i].clone());
                }
            }
            _ => {
                processed_tokens.push(tokens[i].clone());
            }
        }
        i += 1;
    }

    processed_tokens
}