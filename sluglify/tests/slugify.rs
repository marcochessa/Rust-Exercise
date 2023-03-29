use sluglify::slugify;

#[test]
fn test_one_word() { assert_eq!( slugify("CiaoRust"), "ciaorust" )}
#[test]
fn test_accented_char() { assert_eq!( slugify("àòèù"), "aoeu" )}
#[test]
fn test_empty() { assert_eq!( slugify(""), "" )}
#[test]
fn test_whitespaces_and_not_valid_char() { assert_eq!( slugify("ciao /() come [ stai??"), "ciao-come-stai" )}
#[test]
fn test_only_not_valid_char() { assert_eq!( slugify("  & () %"), "" )}
#[test]
fn test_only_one_not_valid_char() { assert_eq!( slugify("?"), "-" )}