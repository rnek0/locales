use locale_config;
use std::str;
use std::ffi::CStr;


// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn my_string_safe() -> String {
    unsafe {
        CStr::from_ptr(listEnv()).to_string_lossy().into_owned()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    

    #[test]
    fn test_get_locale(){
        assert_eq!(get_locale(), "fr");
    }

    #[test]
    fn test_my_string_safe(){
        assert_eq!(my_string_safe(), "fr");
    }
}

/// output: fr
fn get_locale() -> String {
    let my_locale = locale_config::Locale::user_default();
    let mut lang = String::from("");
    
    for loc in my_locale.tags(){
        lang = String::from(format!("{:?}",loc.1));
        // linux   : LanguageRange { language: "fr" }
        // windows : LanguageRange { language: "fr-FR" }
    }

    if cfg!(target_os = "windows"){
        let (first, _) = lang.split_at(2);
        lang = String::from(first);
    }
    let v: Vec<&str> = lang.split(' ').collect();
    let lang = v[3].replace("\"", "");
    lang
}

fn main() {

    

    println!("{}", get_locale());

    println!("Je viens du C {}", my_string_safe());
}
