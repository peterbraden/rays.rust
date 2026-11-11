//! Arithmetic expressions written as text, for shapes defined by a function: the isosurface's
//! f(x, y, z) and the parametric surface's x(u, v), y(u, v) and z(u, v). The syntax is close to
//! POV-Ray's functions: numbers, named variables, + - * / and ^ (power, binding tightest and to
//! the right), parentheses, the constants `pi` and `e`, and these functions:
//!
//! sin cos tan asin acos atan sinh cosh tanh sqrt abs exp ln log (base 10) floor ceil sqr
//! atan2(y, x) pow(a, b) min(a, b, ..) max(a, b, ..) mod(a, b) noise3d(x, y, z)
//! select(a, b, c): b if a < 0, else c; select(a, b, c, d): b if a < 0, c if a = 0, else d.

use std::fmt;
use crate::noise::PerlinNoise;

static PERLIN: std::sync::LazyLock<PerlinNoise> = std::sync::LazyLock::new(PerlinNoise::new);

#[derive(Debug, Clone)]
enum Node {
    Num(f64),
    Var(usize),
    Neg(Box<Node>),
    Bin(char, Box<Node>, Box<Node>),
    Call(String, Vec<Node>),
}

/// A parsed expression over a fixed list of variables.
#[derive(Debug, Clone)]
pub struct Expr {
    root: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExprError(pub String);

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Functions and how many arguments they take (min, max).
fn arity(name: &str) -> Option<(usize, usize)> {
    Some(match name {
        "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh" | "tanh" | "sqrt" | "abs"
        | "exp" | "ln" | "log" | "floor" | "ceil" | "sqr" => (1, 1),
        "atan2" | "pow" | "mod" => (2, 2),
        "min" | "max" => (1, usize::MAX),
        "noise3d" => (3, 3),
        "select" => (3, 4),
        _ => return None,
    })
}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    vars: &'a [&'a str],
}

impl Parser<'_> {
    fn err<T>(&self, msg: &str) -> Result<T, ExprError> {
        Err(ExprError(format!("{} at character {}", msg, self.pos + 1)))
    }

    fn skip_space(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.skip_space();
        self.chars.get(self.pos).copied()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expr(&mut self) -> Result<Node, ExprError> {
        let mut lhs = self.term()?;
        while let Some(op) = self.peek().filter(|c| *c == '+' || *c == '-') {
            self.pos += 1;
            lhs = Node::Bin(op, Box::new(lhs), Box::new(self.term()?));
        }
        Ok(lhs)
    }

    fn term(&mut self) -> Result<Node, ExprError> {
        let mut lhs = self.unary()?;
        while let Some(op) = self.peek().filter(|c| *c == '*' || *c == '/') {
            self.pos += 1;
            lhs = Node::Bin(op, Box::new(lhs), Box::new(self.unary()?));
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> Result<Node, ExprError> {
        if self.eat('-') {
            return Ok(Node::Neg(Box::new(self.unary()?)));
        }
        if self.eat('+') {
            return self.unary();
        }
        self.power()
    }

    fn power(&mut self) -> Result<Node, ExprError> {
        let base = self.atom()?;
        if self.eat('^') {
            // Right associative, and above unary minus: -x^2 is -(x^2), 2^-1 is 0.5.
            return Ok(Node::Bin('^', Box::new(base), Box::new(self.unary()?)));
        }
        Ok(base)
    }

    fn atom(&mut self) -> Result<Node, ExprError> {
        match self.peek() {
            Some('(') => {
                self.pos += 1;
                let e = self.expr()?;
                if !self.eat(')') {
                    return self.err("expected ')'");
                }
                Ok(e)
            },
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let start = self.pos;
                while self.pos < self.chars.len() && (self.chars[self.pos].is_ascii_digit() || self.chars[self.pos] == '.') {
                    self.pos += 1;
                }
                // An exponent, as in 1e-3.
                if self.pos < self.chars.len() && (self.chars[self.pos] == 'e' || self.chars[self.pos] == 'E') {
                    let save = self.pos;
                    self.pos += 1;
                    if self.pos < self.chars.len() && (self.chars[self.pos] == '+' || self.chars[self.pos] == '-') {
                        self.pos += 1;
                    }
                    if self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                        while self.pos < self.chars.len() && self.chars[self.pos].is_ascii_digit() {
                            self.pos += 1;
                        }
                    } else {
                        self.pos = save;
                    }
                }
                let text: String = self.chars[start..self.pos].iter().collect();
                text.parse().map(Node::Num).or_else(|_| self.err(&format!("bad number '{}'", text)))
            },
            Some(c) if c.is_alphabetic() || c == '_' => {
                let start = self.pos;
                while self.pos < self.chars.len() && (self.chars[self.pos].is_alphanumeric() || self.chars[self.pos] == '_') {
                    self.pos += 1;
                }
                let name: String = self.chars[start..self.pos].iter().collect();
                if self.eat('(') {
                    let Some((lo, hi)) = arity(&name) else { return self.err(&format!("unknown function '{}'", name)) };
                    let mut args = vec![];
                    if !self.eat(')') {
                        loop {
                            args.push(self.expr()?);
                            if self.eat(')') { break; }
                            if !self.eat(',') { return self.err("expected ',' or ')'"); }
                        }
                    }
                    if args.len() < lo || args.len() > hi {
                        return self.err(&format!("'{}' takes {} arguments, got {}",
                            name, if lo == hi { lo.to_string() } else { format!("{} or more", lo) }, args.len()));
                    }
                    return Ok(Node::Call(name, args));
                }
                if let Some(i) = self.vars.iter().position(|v| *v == name) {
                    return Ok(Node::Var(i));
                }
                match name.as_str() {
                    "pi" => Ok(Node::Num(std::f64::consts::PI)),
                    "e" => Ok(Node::Num(std::f64::consts::E)),
                    _ => self.err(&format!("unknown name '{}' (variables are {})", name, self.vars.join(", "))),
                }
            },
            Some(c) => self.err(&format!("unexpected '{}'", c)),
            None => self.err("unexpected end"),
        }
    }
}

fn eval(n: &Node, vars: &[f64]) -> f64 {
    match n {
        Node::Num(x) => *x,
        Node::Var(i) => vars[*i],
        Node::Neg(a) => -eval(a, vars),
        Node::Bin(op, a, b) => {
            let (a, b) = (eval(a, vars), eval(b, vars));
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => a.powf(b),
            }
        },
        Node::Call(name, args) => {
            let a = |i: usize| eval(&args[i], vars);
            match name.as_str() {
                "sin" => a(0).sin(),
                "cos" => a(0).cos(),
                "tan" => a(0).tan(),
                "asin" => a(0).asin(),
                "acos" => a(0).acos(),
                "atan" => a(0).atan(),
                "sinh" => a(0).sinh(),
                "cosh" => a(0).cosh(),
                "tanh" => a(0).tanh(),
                "sqrt" => a(0).sqrt(),
                "abs" => a(0).abs(),
                "exp" => a(0).exp(),
                "ln" => a(0).ln(),
                "log" => a(0).log10(),
                "floor" => a(0).floor(),
                "ceil" => a(0).ceil(),
                "sqr" => { let x = a(0); x * x },
                "atan2" => a(0).atan2(a(1)),
                "pow" => a(0).powf(a(1)),
                "mod" => a(0) % a(1),
                "min" => (0..args.len()).map(a).fold(f64::INFINITY, f64::min),
                "max" => (0..args.len()).map(a).fold(f64::NEG_INFINITY, f64::max),
                "noise3d" => PERLIN.noise(a(0), a(1), a(2)),
                "select" => {
                    let c = a(0);
                    if c < 0. { a(1) } else if args.len() == 3 || c == 0. { a(2) } else { a(3) }
                },
                _ => unreachable!("checked when parsed"),
            }
        },
    }
}

impl Expr {
    /// Parse `text`, in which the names in `vars` are variables, given in that order to `eval`.
    pub fn parse(text: &str, vars: &[&str]) -> Result<Expr, ExprError> {
        let mut p = Parser { chars: text.chars().collect(), pos: 0, vars };
        let root = p.expr()?;
        if p.peek().is_some() {
            return p.err("unexpected text after the expression");
        }
        Ok(Expr { root })
    }

    pub fn eval(&self, vars: &[f64]) -> f64 {
        eval(&self.root, vars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(s: &str, x: f64) -> f64 {
        Expr::parse(s, &["x", "y"]).unwrap().eval(&[x, 10.])
    }

    #[test]
    fn test_precedence() {
        assert_eq!(ev("1 + 2 * 3", 0.), 7.);
        assert_eq!(ev("(1 + 2) * 3", 0.), 9.);
        assert_eq!(ev("-x^2", 3.), -9.);
        assert_eq!(ev("2^3^2", 0.), 512.);
        assert_eq!(ev("2^-1", 0.), 0.5);
        assert_eq!(ev("x - y - 1", 0.), -11.);
        assert_eq!(ev("8 / 2 / 2", 0.), 2.);
        assert_eq!(ev("1e-3 * 1E3", 0.), 1.);
    }

    #[test]
    fn test_functions() {
        assert!((ev("sin(pi / 2) + cos(0)", 0.) - 2.).abs() < 1e-12);
        assert_eq!(ev("max(x, 2, y)", 1.), 10.);
        assert_eq!(ev("min(x)", 1.), 1.);
        assert_eq!(ev("select(x, 1, 2)", -1.), 1.);
        assert_eq!(ev("select(x, 1, 2, 3)", 0.), 2.);
        assert_eq!(ev("select(x, 1, 2, 3)", 1.), 3.);
        assert_eq!(ev("sqr(x) + abs(-1)", 3.), 10.);
        assert!(ev("noise3d(x, 0.5, 0.25)", 0.3).abs() <= 1.5);
    }

    #[test]
    fn test_errors() {
        assert!(Expr::parse("x +", &["x"]).is_err());
        assert!(Expr::parse("q", &["x"]).unwrap_err().0.contains("unknown name 'q'"));
        assert!(Expr::parse("frob(x)", &["x"]).unwrap_err().0.contains("unknown function"));
        assert!(Expr::parse("pow(x)", &["x"]).unwrap_err().0.contains("takes 2"));
        assert!(Expr::parse("(x", &["x"]).is_err());
        assert!(Expr::parse("x x", &["x"]).is_err());
    }
}
