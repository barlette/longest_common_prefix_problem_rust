use std::cmp;

fn longest_common_prefix(strs: Vec<String>) -> String {
    let conv_strs: Vec<&str> = strs.iter().map(|s| &**s).collect();
    if conv_strs.is_empty() {
        return "".to_string();
    } else {
        let mut min_len = i32::MAX;
        for str in conv_strs {
            min_len = cmp::min(min_len, str.len().try_into().unwrap());
        }
        let mut low = 1;
        let mut high =  min_len;

        while low <= high {
            let middle = (low + high) / 2;
            if isCommonPrefix(&conv_strs, &middle) {
                low = middle + 1;
            }
            else {
                high = middle - 1;
            }  
        }
        return "".to_string();
    }
    
}

fn isCommonPrefix(strs: &Vec<&str>, len: &i32) -> bool {
    let str1: &str = &strs[0].get(0..*len as usize).unwrap();
    
    //let mut slice_strs: Vec<&str> = [].to_vec();
    // strs[1..].clone_from_slice(&slice_strs);
    for str in &strs[1..] {
        if !str.starts_with(str1) {
            return false;
        } 
    } 
    return true;
}


fn main() {
    let strs:Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];

    println!("{}", longest_common_prefix(strs));
    
}
