use std::collections::VecDeque;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stk = VecDeque::new();
        for c in expression.chars() {
            if c != ')' && c != ',' {
                stk.push_back(c);
            } else if c == ')' {
                let mut exp = Vec::new();
                while let Some(&top) = stk.back() {
                    if top == '(' {
                        break;
                    }
                    stk.pop_back();
                    exp.push(top == 't');
                }
                stk.pop_back();
                if let Some(&op) = stk.back() {
                    stk.pop_back();
                    let mut v = exp[0];
                    if op == '&' {
                        for &b in &exp {
                            v &= b;
                        }
                    } else if op == '|' {
                        for &b in &exp {
                            v |= b;
                        }
                    } else {
                        v = !v;
                    }
                    if v {
                        stk.push_back('t');
                    } else {
                        stk.push_back('f');
                    }
                }
            }
        }

        stk.back() == Some(&'t')
    }
}
struct Solution {}
pub fn run() {
    tracing::info!("{}", Solution::parse_bool_expr("&(|(f))".to_string()));
}
