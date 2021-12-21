pub fn convert(string:&str) -> Vec<&str>{
    let bytes = string.as_bytes();
    let mut stringVector:Vec<&str> = Vec::new();
    let mut corte = 0; 

    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b' ' {
            stringVector.push(&string[corte..i]);

            corte = i;
        }
    }

    if stringVector.is_empty() {
        stringVector.push(&string[..]);
    }

    return stringVector;
}