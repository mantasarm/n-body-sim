use std::collections::HashMap;

pub struct Translations {
    translations: HashMap<String, String>,
    pub chosen_lang: String
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
        translations.insert("ltcreate".to_string(), "Kurti savo".to_string());
        translations.insert("ltaddbodies".to_string(), "Pridėti kūnus".to_string());
        translations.insert("ltbodymass".to_string(), "Kūno masė: ".to_string());
        translations.insert("ltmoveable".to_string(), "Veikiamas jėgų".to_string());
        translations.insert("ltdir".to_string(), "Kryptis: ".to_string());
        translations.insert("ltinitf".to_string(), "Pradinė kūno jėga: ".to_string());
        translations.insert("ltclear".to_string(), "Išvalyti".to_string());
        translations.insert("ltbodiesinsim".to_string(), "Kūnai simuliacijoje: ".to_string());
        translations.insert("ltobj".to_string(), "Kūnas".to_string());
        translations.insert("ltremove".to_string(), "Naikinti".to_string());
        translations.insert("ltvel".to_string(), "Greitis".to_string());
        translations.insert("ltpos".to_string(), "Pozicija".to_string());
        translations.insert("ltmass".to_string(), "Masė".to_string());
        translations.insert("ltrclick".to_string(), "Paspauskite pelės kairį klavišą lauke, kad pridėti kūnus".to_string());
        translations.insert("lteditor".to_string(), "Redaktorius".to_string());


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
        translations.insert("encreate".to_string(), "Playground".to_string());
        translations.insert("enaddbodies".to_string(), "Add bodies".to_string());
        translations.insert("enbodymass".to_string(), "Body mass: ".to_string());
        translations.insert("enmoveable".to_string(), "Moveable".to_string());
        translations.insert("endir".to_string(), "Direction: ".to_string());
        translations.insert("eninitf".to_string(), "Initial body force: ".to_string());
        translations.insert("enclear".to_string(), "Clear".to_string());
        translations.insert("enbodiesinsim".to_string(), "Bodies in simulation: ".to_string());
        translations.insert("enobj".to_string(), "Object".to_string());
        translations.insert("enremove".to_string(), "Remove".to_string());
        translations.insert("envel".to_string(), "Velocity".to_string());
        translations.insert("enpos".to_string(), "Position".to_string());
        translations.insert("enmass".to_string(), "Mass".to_string());
        translations.insert("enrclick".to_string(), "Right click in the field to add bodies".to_string());
        translations.insert("eneditor".to_string(), "Editor".to_string());


        Self {
            translations,
            chosen_lang: "en".to_string()
        }
    }

    pub fn get(&self, text: &str) -> String {
        self.translations.get(&format!("{}{}", self.chosen_lang, text).to_string()).unwrap().to_string()
    }
}