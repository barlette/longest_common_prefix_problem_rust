use std::{cmp, ops::Index};

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
        
        
           
    } else {
        let mut min_len = i32::MAX;
        for str in strs {
            min_len = cmp::min(min_len, str.len().try_into().unwrap());
        }
        let low = 1;
        let high =  min_len;

        while low <= high {
            let middle = (low + high) / 2;
            if isCommonPrefix(strs, middle) {
                low = middle + 1;
            }
            else {
                high = middle - 1;
            }
               
        }


        return "".to_string();
    }
    
}

fn isCommonPrefix(strs: Vec<String>, len: i32) -> bool {
    let str1 = &strs[0].index(0..len);
    for (int i = 1; i < strs.length; i++)
        if (!strs[i].startsWith(str1))
            return false;
    return true;
}


fn main() {
    let strs:Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];

    println!("{}", longest_common_prefix(strs));
    
}
