pub fn bytes_to_symbol(buf: &[u8]) -> String {
    let bytes = buf.iter().filter(|&&byte| byte != 0 ).copied().collect::<Vec<u8>>();
    let symbol = String::from_utf8(
        bytes
    ).unwrap();


    return symbol;
}

