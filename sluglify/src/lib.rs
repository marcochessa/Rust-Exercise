
pub fn slugify(s: &str) -> String{
    let mut result = String::new();

    if s.len() == 0 {
        return result;
    }

    for x in s.to_lowercase().chars(){
        let tmp = conv(x);
        if tmp.is_alphanumeric() {
            result.push(tmp);
        }else{
            if result.len()>0{
                if result.chars().last().unwrap() != '-' {
                    result.push('-');
                }
            }else{
                result.push('-');
            }
        }
    }

    if s.len() > 1 && result.chars().last().unwrap() == '-' {
        result.pop();
    }

    return result;
}

fn conv(c: char) -> char {
    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

    for (i, x) in SUBS_I.chars().enumerate(){
        if c == x {
            return SUBS_O.chars().nth(i).unwrap();
        }
    }
    return c;
}