use locale_config;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_get_locale(){
        assert_eq!(get_locale(), "fr");
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
}
