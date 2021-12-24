pub fn convert(msg:&str) -> Vec<&str> {
    let bytes = msg.as_bytes();
    let mut stringVector = Vec::new();
    let mut cut = 0;

    for (i,&word) in bytes.iter().enumerate() {
        if word == b' '{
            stringVector.push(&msg[cut..i]);
            stringVector.push(&msg[i..].trim());

            break;
        }
    }

    return stringVector;
}
