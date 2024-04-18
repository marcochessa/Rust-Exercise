//use std::env;
use clap::Parser;
const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

#[derive(Parser, Debug)]
struct Args {
    slug_in: String, // input string
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
    /* UTILIZZO ARGS CLASSICO
    // Ottieni gli argomenti della riga di comando
    let args: Vec<String> = env::args().collect();

    // Verifica se ci sono abbastanza argomenti
    if args.len() != 2 {
        println!("Usage: {} <stringa>", args[0]);
        return;
    }

    // Estrai la stringa dall'argomento della riga di comando
    let input_string = &args[1];
    */


     /* UTILIZZO ARGS CON CLAP */

    let args = Args::parse();


    // Chiama la funzione slugify con la stringa fornita
    let slugified_string = slugify(&args.slug_in);

    // Stampa l'output con il prefisso "slug:"
    println!("slug: {}", slugified_string);
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
