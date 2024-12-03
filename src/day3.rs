enum State {
    IDLE,
    M,
    U,
    L,
    Parenthesis,
    Op1Dig1(u32),
    Op1Dig2(u32),
    Op1Dig3(u32),
    Comma(u32),
    Op2Dig1(u32, u32),
    Op2Dig2(u32, u32),
    Op2Dig3(u32, u32),
    D,
    O,
    N,
    Apost,
    T,
    DoParenthesis,
    DontParenthesis,
}

pub fn day3(input: &str) {
    let mut accum = 0;
    let mut accum2 = 0;
    let mut state = State::IDLE;
    let mut enabled = true;
    for c in input.as_bytes() {
        match (state, *c) {
            (_, b'm') => state = State::M,
            (_, b'd') => state = State::D,
            (State::M, b'u') => state = State::U,
            (State::U, b'l') => state = State::L,
            (State::L, b'(') => state = State::Parenthesis,
            (State::Parenthesis, digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op1Dig1((digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op1Dig1(digit), b',') => state = State::Comma(digit),
            (State::Op1Dig1(prev), digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op1Dig2(prev * 10 + (digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op1Dig2(digit), b',') => state = State::Comma(digit),
            (State::Op1Dig2(prev), digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op1Dig3(prev * 10 + (digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op1Dig3(digit), b',') => state = State::Comma(digit),
            (State::Comma(a), digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op2Dig1(a, (digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op2Dig1(a, b), b')') => {
                accum += a * b;
                if enabled {
                    accum2 += a * b;
                }
                state = State::IDLE
            }
            (State::Op2Dig1(a, prev), digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op2Dig2(a, prev * 10 + (digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op2Dig2(a, b), b')') => {
                accum += a * b;
                if enabled {
                    accum2 += a * b;
                }
                state = State::IDLE
            }
            (State::Op2Dig2(a, prev), digit) => {
                if digit >= b'0' && digit <= b'9' {
                    state = State::Op2Dig3(a, prev * 10 + (digit - b'0') as u32)
                } else {
                    state = State::IDLE
                }
            }
            (State::Op2Dig3(a, b), b')') => {
                accum += a * b;
                if enabled {
                    accum2 += a * b;
                }
                state = State::IDLE
            }
            (State::D, b'o') => state = State::O,
            (State::O, b'n') => state = State::N,
            (State::N, b'\'') => state = State::Apost,
            (State::Apost, b't') => state = State::T,
            (State::O, b'(') => state = State::DoParenthesis,
            (State::DoParenthesis, b')') => {
                enabled = true;
                state = State::IDLE
            }
            (State::T, b'(') => state = State::DontParenthesis,
            (State::DontParenthesis, b')') => {
                enabled = false;
                state = State::IDLE
            }
            _ => state = State::IDLE,
        }
    }

    println!("{accum}");
    println!("{accum2}");
}

#[test]
fn test0() {
    day3("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
}
