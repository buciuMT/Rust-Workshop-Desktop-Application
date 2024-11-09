use lazy_static::lazy_static;
use regex::Regex;
use slint::include_modules;

include_modules!();

lazy_static! {
    static ref VALID_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();
        ui.set_tfocus(false);
        ui.set_textarea(match new_input.as_str() {
            "C" => {println!("test");"".into()},
            _=> current_text+new_input.as_str(),
            _ => todo!()
        });

        // TASK: Adaugă logica pentru cazurile:
        // - "C": Golirea zonei de text
        // - "=": Calcularea rezultatului și afișarea acestuia
    });

    ui.run()
}

// TASK: Completează funcția `evaluate`, care verifică validitatea expresiei și returnează rezultatul sau un mesaj de eroare
// HINT: Folosește regex-ul `VALID_EXPRESSION` pentru a verifica dacă `input` este o expresie validă.
// Dacă expresia este validă, apelează funcția `compute`. Dacă nu, returnează un mesaj de eroare, cum ar fi "Invalid Expression".
fn evaluate(input: &str) -> String {
    todo!() // <-- Înlocuiește `todo!()` cu implementarea funcției
}

// TASK: Implementează funcția `compute` pentru a realiza operațiile de bază (+, -, *, /) și a returna rezultatul
// HINT: Parcurge simbolurile de operare și folosește `.split()` pentru a împărți `input` în două părți: înainte și după simbol.
// Convertește fiecare parte în `f64` și returnează rezultatul în funcție de simbol.
fn compute(input: &str) -> Option<f64> {
    // TASK: Inițializează simbolurile de operare (+, -, *, /)
    // HINT: Creează o listă `let symbols = ["+", "-", "*", "/"];`

    None // <-- Returnează None dacă nu găsește niciun simbol valid
}
