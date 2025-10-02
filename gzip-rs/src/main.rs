use std::collections::HashMap;

/// 32KiB, specified in RFC 1951
/// Powers of 2 are efficient for bitwise operations and memory
/// addressing. 32KB was chosen as a good balance in the 1990s
/// when DEFLATE was designed.
///
/// To illustrate:
/// - Larger window = better compression (can find matches farther back)
/// - Smaller window = less memory usage and faster compression
const WINDOW_SIZE: usize = 32_768;

/// How far ahead you look when trying to find matches
/// - Standard DEFLATE: 258 bytes lookahead (RFC 1951)
/// - Smaller lookahead = faster but finds shorter matches
/// - Larger lookahead = slower but can find longer matches
///
/// Examples:
///
/// ```
/// // Very fast, minimal compression (small window + short matches)
/// lz77_compress(data, 4096, 32);
///
/// // Balanced (medium window)
/// lz77_compress(data, 8192, 128);
///
/// // Standard DEFLATE (what you should normally use)
/// lz77_compress(data, 32768, 258);
/// ```
const LOOKAHEAD_SIZE: usize = 258;

// LZ77 Token: either a literal byte or a (length, distance) pair
#[derive(Debug, Clone)]
enum Token {
    Literal(u8),
    Reference { length: usize, distance: usize },
}

// Huffman tree node
#[derive(Debug, Clone)]
enum HuffmanNode {
    Leaf { symbol: u16, freq: usize },
    Internal { left: Box<HuffmanNode>, right: Box<HuffmanNode>, freq: usize },
}

impl HuffmanNode {
    fn freq(&self) -> usize {
        match self {
            HuffmanNode::Leaf { freq, .. } => *freq,
            HuffmanNode::Internal { freq, .. } => *freq,
        }
    }
}

/// LZ77 Compression - finds repeated sequences
fn lz77_compress(data: &[u8], window_size: usize, lookahead_size: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        let mut best_length = 0;
        let mut best_distance = 0;

        // Search window starts
        let search_start = pos.saturating_sub(window_size);

        // Look for matches in the search window
        for i in search_start..pos {
            let mut length = 0;

            // Count matching bytes
            while length < lookahead_size
                && pos + length < data.len()
                && data[i + length] == data[pos + length] {
                length += 1;
            }

            // Keep track of best match
            if length > best_length {
                best_length = length;
                best_distance = pos - i;
            }
        }

        // Only use reference if it's at least 3 bytes (worthwhile)
        if best_length >= 3 {
            tokens.push(Token::Reference {
                length: best_length,
                distance: best_distance,
            });
            pos += best_length;
        } else {
            tokens.push(Token::Literal(data[pos]));
            pos += 1;
        }
    }

    tokens
}

/// Build Huffman tree from frequency map
fn build_huffman_tree(frequencies: &HashMap<u16, usize>) -> Option<HuffmanNode> {
    if frequencies.is_empty() {
        return None;
    }

    // Create initial leaf nodes
    let mut nodes: Vec<HuffmanNode> = frequencies
        .iter()
        .map(|(&symbol, &freq)| HuffmanNode::Leaf { symbol, freq })
        .collect();

    // Build tree by repeatedly combining two lowest frequency nodes
    while nodes.len() > 1 {
        // Sort by frequency
        nodes.sort_by_key(|n| std::cmp::Reverse(n.freq()));

        // Take two lowest frequency nodes
        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();

        // Create internal node
        let internal = HuffmanNode::Internal {
            freq: left.freq() + right.freq(),
            left: Box::new(left),
            right: Box::new(right),
        };

        nodes.push(internal);
    }

    nodes.pop()
}

/// Generate Huffman codes from tree
fn generate_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<u16, String>) {
    match node {
        HuffmanNode::Leaf { symbol, .. } => {
            codes.insert(*symbol, if prefix.is_empty() { "0".to_string() } else { prefix });
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes(left, format!("{}0", prefix), codes);
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}

/// Encode tokens using Huffman coding
fn huffman_encode(tokens: &[Token]) -> (String, HashMap<u16, String>) {
    // Count frequencies
    let mut frequencies = HashMap::new();

    for token in tokens {
        match token {
            Token::Literal(byte) => {
                *frequencies.entry(*byte as u16).or_insert(0) += 1;
            }
            Token::Reference { length, distance } => {
                // Encode length and distance as special symbols
                // In real DEFLATE these use special code ranges
                let length_code = 256 + (*length as u16);
                let distance_code = 512 + (*distance as u16);
                *frequencies.entry(length_code).or_insert(0) += 1;
                *frequencies.entry(distance_code).or_insert(0) += 1;
            }
        }
    }

    // Build Huffman tree and generate codes
    let tree = build_huffman_tree(&frequencies).unwrap();
    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    // Encode the data
    let mut encoded = String::new();
    for token in tokens {
        match token {
            Token::Literal(byte) => {
                encoded.push_str(&codes[&(*byte as u16)]);
            }
            Token::Reference { length, distance } => {
                let length_code = 256 + (*length as u16);
                let distance_code = 512 + (*distance as u16);
                encoded.push_str(&codes[&length_code]);
                encoded.push_str(&codes[&distance_code]);
            }
        }
    }

    (encoded, codes)
}

// Compress data using LZ77 + Huffman (simplified DEFLATE)
fn compress(data: &[u8]) -> (String, HashMap<u16, String>, Vec<Token>) {
    let tokens = lz77_compress(data, WINDOW_SIZE, LOOKAHEAD_SIZE);
    let (encoded, codes) = huffman_encode(&tokens);
    (encoded, codes, tokens)
}

fn main() {
    let data = b"Hello, World! Hello, World! This is a test. Hello, World!";

    println!("Original data: {}", String::from_utf8_lossy(data));
    println!("Original size: {} bytes ({} bits)\n", data.len(), data.len() * 8);

    let (encoded, codes, tokens) = compress(data);

    println!("=== LZ77 Tokens ===");
    for (i, token) in tokens.iter().enumerate().take(20) {
        match token {
            Token::Literal(byte) => println!("{}: Literal '{}'", i, *byte as char),
            Token::Reference { length, distance } => {
                println!("{}: Reference (length: {}, distance: {})", i, length, distance);
            }
        }
    }
    if tokens.len() > 20 {
        println!("... and {} more tokens", tokens.len() - 20);
    }

    println!("\n=== Huffman Codes (sample) ===");
    let mut code_vec: Vec<_> = codes.iter().collect();
    code_vec.sort_by_key(|(symbol, _)| *symbol);
    for (symbol, code) in code_vec.iter().take(10) {
        if **symbol < 256 {
            println!("'{}' ({}): {}", **symbol as u8 as char, symbol, code);
        } else if **symbol < 512 {
            println!("Length {}: {}", **symbol - 256, code);
        } else {
            println!("Distance {}: {}", **symbol - 512, code);
        }
    }

    println!("\n=== Compression Results ===");
    println!("Encoded size: {} bits ({:.2} bytes)",
             encoded.len(),
             encoded.len() as f64 / 8.0);
    println!("Compression ratio: {:.2}%",
             (1.0 - (encoded.len() as f64 / 8.0) / data.len() as f64) * 100.0);
    println!("\nFirst 100 bits of encoded data:\n{}",
             &encoded.chars().take(100).collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz77_compression() {
        let data = b"abcabc";
        let tokens = lz77_compress(data, 100, 100);

        // Should find the repeated "abc"
        assert!(tokens.iter().any(|t| matches!(t, Token::Reference { .. })));
    }

    #[test]
    fn test_huffman_tree() {
        let mut freq = HashMap::new();
        freq.insert(65, 3); // 'A' appears 3 times
        freq.insert(66, 1); // 'B' appears 1 time

        let tree = build_huffman_tree(&freq);
        assert!(tree.is_some());
    }
}
