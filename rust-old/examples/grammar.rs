//! Grammar profiles — morphological typology, word order, case systems.

fn main() {
    println!("=== Grammar Profiles ===\n");

    for code in varna::grammar::all_codes() {
        let g = varna::grammar::by_code(code).unwrap();
        let info = varna::registry::info(code).unwrap();
        println!("{} ({code})", info.name);
        println!("  Morphology: {:?}", g.morphology);
        println!("  Word order: {:?}", g.word_order);
        if g.case_count > 0 {
            println!("  Cases: {}", g.case_count);
        }
        if g.has_gender {
            println!("  Genders: {}", g.gender_count);
        }
        if g.has_dual {
            println!("  Has dual number");
        }
        if g.has_classifiers {
            println!("  Uses classifiers");
        }
        println!();
    }

    // Compare typologies
    println!("=== Typology Comparison ===\n");
    let isolating: Vec<_> = varna::grammar::all_codes()
        .iter()
        .filter(|c| {
            varna::grammar::by_code(c).unwrap().morphology == varna::grammar::Morphology::Isolating
        })
        .copied()
        .collect();
    let agglutinative: Vec<_> = varna::grammar::all_codes()
        .iter()
        .filter(|c| {
            varna::grammar::by_code(c).unwrap().morphology
                == varna::grammar::Morphology::Agglutinative
        })
        .copied()
        .collect();

    println!("  Isolating: {isolating:?}");
    println!("  Agglutinative: {agglutinative:?}");
}
