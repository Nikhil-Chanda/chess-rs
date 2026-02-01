pub fn parse_square(s: &str) -> Option<(usize, usize)> {
    if s.len() != 2 {
        return None;
    }
    
    let chars: Vec<char> = s.chars().collect();
    let file = chars[0];
    let rank = chars[1];
    
    let col = match file {
        'a'..='h' => (file as u8 - b'a') as usize,
        _ => return None,
    };
    let row = match rank {
        '1'..='8' => (rank as u8 - b'1') as usize,
        _ => return None,
    };

    Some((row, col))
}