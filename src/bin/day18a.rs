const INPUT: &str = include_str!("../../inputs/day18.txt");

struct Parser {
    input: &'static str,
}

impl Parser {
    fn eat(&mut self, pat: &str) -> bool {
        if let Some(remaining) = self.input.strip_prefix(pat) {
            self.input = remaining;
            true
        } else {
            false
        }
    }
    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }
    fn parse_number(&mut self) -> i64 {
        let res = self.input[0..1].parse().unwrap();
        self.input = &self.input[1..];
        res
    }
    fn parse_term(&mut self) -> i64 {
        if self.eat("(") {
            let res = self.parse_expr();
            assert!(self.eat(")"));
            res
        } else {
            self.parse_number()
        }
    }
    fn parse_expr(&mut self) -> i64 {
        self.skip_whitespace();
        let mut res = self.parse_term();
        loop {
            self.skip_whitespace();
            if self.eat("+") {
                self.skip_whitespace();
                res += self.parse_term();
            } else if self.eat("*") {
                self.skip_whitespace();
                res *= self.parse_term();
            } else {
                break;
            }
        }
        res
    }
    fn parse(input: &'static str) -> i64 {
        Parser { input }.parse_expr()
    }
}

fn main() {
    let res: i64 = INPUT.lines().map(Parser::parse).sum();

    println!("{}", res);
}
