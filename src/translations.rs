use std::collections::HashMap;

pub struct Translations {
    translations: HashMap<String, String>
}

impl Translations {
    pub fn new() -> Self {
        let mut translations = HashMap::<String, String>::new();

        translations.insert("ltsettings".to_string(), "Parametrai".to_string());
        translations.insert("ltchooselang".to_string(), "Pasirinkite kalbą: ".to_string());
        translations.insert("ltpattern".to_string(), "raštas".to_string());
        translations.insert("ltchoose".to_string(), "Pasirinkite raštą: ".to_string());
        translations.insert("ltsimspeed".to_string(), "Simuliacijos greitis".to_string());
        translations.insert("ltshowtrail".to_string(), "Rodyti trajektoriją".to_string());
        translations.insert("ltshowbodies".to_string(), "Rodyti kūnus".to_string());
        translations.insert("ltpause".to_string(), "Sustabdyti".to_string());
        translations.insert("ltrestart".to_string(), "Paleisti iš naujo".to_string());
        translations.insert("ltwasd".to_string(), "Naudokite W, A, S ir D klavišus judėjimui".to_string());
        translations.insert("ltqe".to_string(), "Naudokite Q ir E klavišus pritraukti ir atitraukti vaizdą".to_string());
        translations.insert("ltcredits".to_string(), "Darba atiliko Mantas Armalys".to_string());

        translations.insert("ensettings".to_string(), "Settings".to_string());
        translations.insert("enchooselang".to_string(), "Choose language: ".to_string());
        translations.insert("enpattern".to_string(), "pattern".to_string());
        translations.insert("enchoose".to_string(), "Choose pattern: ".to_string());
        translations.insert("ensimspeed".to_string(), "Simulation speed".to_string());
        translations.insert("enshowtrail".to_string(), "Show trails".to_string());
        translations.insert("enshowbodies".to_string(), "Show bodies".to_string());
        translations.insert("enpause".to_string(), "Pause".to_string());
        translations.insert("enrestart".to_string(), "Restart".to_string());
        translations.insert("enwasd".to_string(), "Use the W, A, S and D keys to move".to_string());
        translations.insert("enqe".to_string(), "Use the Q and E keys to zoom".to_string());
        translations.insert("encredits".to_string(), "Made by mantasarm".to_string());


        Self {
            translations
        }
    }

    pub fn get(&self, lang: &String, text: &str) -> String {
        self.translations.get(&format!("{}{}", lang, text).to_string()).unwrap().to_string()
    }
}