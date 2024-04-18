
const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

/*
//Punto 1
impl MySlug for String {
    fn is_slug(&self) -> bool {
        *self==slugify(self)
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }

}


impl MySlug for &str {
    fn is_slug(&self) -> bool {
        if *self==slugify(self){
            true
        } else { false }
    }

    fn to_slug(&self) -> String {
        slugify(self)
    }

}
*/

//Punto 2
impl<T> MySlug for T
    where
        T:?Sized, //Superfluo
        T: AsRef<str> {
    fn is_slug(&self) -> bool {
        if *self.as_ref()==slugify(self.as_ref()){
            true
        } else { false }
    }

    fn to_slug(&self) -> String {
        slugify(self.as_ref())
    }
}


fn slugify(s: &str) -> String {
    let mut result = String::new();
    for c in s.to_lowercase().chars(){
        if c.is_ascii_alphanumeric(){
            result.push(c);
        } else {
            let cc = conv(c);
            if cc == '-' {
                if !result.ends_with('-') {
                    result.push(cc);
                }
            } else {
                result.push(cc);
            }
        }
    }
    if result.len()>1 && result.ends_with('-'){
        result.pop();
    }
    result
}

fn conv(c: char) -> char {
    match SUBS_I.chars().position(|x| x == c) {
        Some(index) => SUBS_O.chars().nth(index).unwrap_or('-'), // Se il carattere non è in SUBS_O, restituisce "-" altrimenti il carattere corretto
        None => '-' // Se il carattere non è in SUBS_I, restituisce "-"
    }
}

fn main() {

    let s1 = String::from("Hello String");
    let s2 = "hello-slice";
    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{} s4:{}", s3, s4); // stampa:  s3:hello-string s4:hello-slice
}

// Sezione per gli unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversione_lettera_accentata() {
        let carattere_accentato = "é";
        let carattere_convertito = slugify(carattere_accentato);
        assert_eq!(carattere_convertito, "e");
    }

    #[test]
    fn test_conversione_lettera_non_accentata() {
        let carattere_non_accentato = "a";
        let carattere_convertito = slugify(carattere_non_accentato);
        assert_eq!(carattere_convertito, "a");
    }

    #[test]
    fn test_conversione_carattere_non_amesso() {
        let carattere_non_amesso = "#";
        let carattere_convertito = slugify(carattere_non_amesso);
        assert_eq!(carattere_convertito, "-");
    }

    #[test]
    fn test_conversione_carattere_accentato_non_compreso() {
        let carattere_accentato_non_compreso = "ῶ";
        let carattere_convertito = slugify(carattere_accentato_non_compreso);
        assert_eq!(carattere_convertito, "-");
    }

    #[test]
    fn test_slugify_stringa_con_spazi() {
        let stringa = "Ciao mondo";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "ciao-mondo");
    }

    #[test]
    fn test_slugify_stringa_caratteri_accentati() {
        let stringa = "àéè";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "aee");
    }

    #[test]
    fn test_slugify_stringa_con_caratteri_non_validi() {
        let stringa = "C#a#o";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "c-a-o");
    }

    #[test]
    fn test_slugify_stringa_con_solo_caratteri_non_validi() {
        let stringa = "###";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "-");
    }

    #[test]
    fn test_slugify_stringa_con_spazio_alla_fine() {
        let stringa = "Ciao ";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "ciao");
    }

    #[test]
    fn test_slugify_stringa_con_caratteri_non_validi_consecutivi_alla_fine() {
        let stringa = "Ciao##";
        let stringa_slugify = slugify(stringa);
        assert_eq!(stringa_slugify, "ciao");
    }


}
