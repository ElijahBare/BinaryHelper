pub fn dec2binaryfluff(mut decimal:i128, length:i32) -> String {

    let mut ret = "".to_string();

    if decimal == 0 {
        return decimal.to_string();
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        ret = bits.chars().rev().collect::<String>().parse::<String>().unwrap();
    }

    let mut fluff = "".to_string();
    for _ in ret.len()..length as usize {
        fluff.push_str("0")
    }

    let ret_final = format!("{}{}", fluff, ret.to_string());
    ret_final.to_string()
}

pub fn dec2binary(mut decimal:i128) -> String {

    let mut ret = "".to_string();

    if decimal == 0 {
        return decimal.to_string();
    } else {
        let mut bits = String::new();

        while decimal > 0 {
            if decimal % 2 == 0 {
                bits.push_str("0");
            } else {
                bits.push_str("1");
            }

            decimal /= 2;
        }

        ret = bits.chars().rev().collect::<String>().parse::<String>().unwrap();
    }

    ret.to_string()
}

pub fn binary_str2dec(mut bin:String) -> i128 {
    let mut decimal:i128 = 0 as i128;
    for char in 0..bin.chars().count(){
        if bin.as_bytes()[char] as char == '1'{
            let expo:f64 = (bin.chars().count() - char - 1) as f64;
            decimal += (2 as f64).powi(expo as f64 as i32) as i128;
        }
    }
    return decimal as i128;
}

pub fn binary_str2dec_str(mut bin:String) -> String {
    let mut decimal:i128 = 0 as i128;
    for char in 0..bin.chars().count(){
        if bin.as_bytes()[char] as char == '1'{
            let expo:f64 = (bin.chars().count() - char - 1) as f64;
            decimal += (2 as f64).powi(expo as f64 as i32) as i128;
        }
    }
    return decimal.to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_2_dec() {
        let result = binary_str2dec("101".to_string());
        assert_eq!(result, 5);
    }

    #[test]
    fn str_2_dec_str() {
        let result = binary_str2dec_str("111001010100111101000111011100111101110001001000000001111111".to_string());
        assert_eq!(result, "1032719007549259903");
    }

    #[test]
    fn dec_2_binary_string_fluff() {
        let result = dec2binaryfluff(10,7);
        assert_eq!(result, "0001010");
    }

    #[test]
    fn dec_2_binary_string() {
        let result = dec2binary(10);
        assert_eq!(result, "1010");
    }

}



