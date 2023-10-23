use crate::token::Token;
#[allow(dead_code)]
pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut comment = String::new();
    let mut bracket_count = 0;
    for char in src.chars() {
        let token;
        match char {
            '>' => token = Some(Token::MoveRight),
            '<' =>token = Some(Token::MoveLeft),
            '+' =>token = Some(Token::Increment),
            '-' =>token = Some(Token::Decrement),
            '.' =>token = Some(Token::Print),
            ',' =>token = Some(Token::Read),
            '[' => {token = Some(Token::OpenLoop);
                bracket_count += 1;},
            ']' => {token = Some(Token::CloseLoop);
                bracket_count -= 1;}
            c => {
                token = None;
                comment.push(c);
            },
        };
        if let Some(token) = token {
            match !comment.is_empty() {
                true => {
                    tokens.push(Token::Comment(comment.clone()));
                    comment.clear();
                }
                false => (),
            }
            tokens.push(token);
        }
        if bracket_count < 0 {
            panic!("Unmatched brakets")
        }
    };
    match !comment.is_empty() {
        true => {
                tokens.push(Token::Comment(comment.clone()));
                comment.clear();
            }
        false => (),
    }

    if bracket_count != 0 {
        panic!("Unmatched brackets");
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tokenize(){
        let src = "++>><<";
        let tokens = tokenize(src);
        assert_eq!(tokens, vec![Token::Increment, Token::Increment, Token::MoveRight, Token::MoveRight, Token::MoveLeft, Token::MoveLeft]);
        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn all_tokenize(){
        let src = "++>><<,.[]Comment";
        let tokens = tokenize(src);
        assert_eq!(tokens.len(), 11);
    }
    #[test]
    #[should_panic]
    fn unbalanced_brakets(){
        let src = "[][][";
        let _tokens = tokenize(src);
    }
    #[test]
    #[should_panic]
    fn mismatched_brakets(){
        let src = "++][";
        let _tokens = tokenize(src);
    }
}
