fn pattern_match_simple() {
    let num = 3;
    let letter = match num {
        1 => 'A',
        2 => 'B',
        3 => {
            (64 + 1 + 2 as u8) as char
        },
        _ => '#', // rust will not guess
    };
    println!("{}", letter);
}

fn main(){
    pattern_match_simple();
}