// get the song's or the playlist's name by conveting the message into a vector
pub fn getName(msg:&str) -> Vec<&str> {
    let bytes = msg.as_bytes();
    let mut stringVector = Vec::new();
    let cut = 0;

    for (i,&word) in bytes.iter().enumerate() {
        if word == b' '{
            stringVector.push(&msg[cut..i]);
            stringVector.push(&msg[i..].trim());

            break;
        }
    }

    return stringVector;
}

// convert the message with the configuration into a vector
pub fn getConfig(msg:&str) -> Vec<&str> {
    let bytes = msg.as_bytes();
    let mut stringVector = Vec::new();
    let mut cut = 0;

    for (i,&word) in bytes.iter().enumerate() {
        if word == b' ' || i == bytes.len() - 1 {
            stringVector.push(msg[cut..i + 1].trim());

            cut = i;
        }
    }

    println!("{:?}", stringVector);

    return stringVector;
}