use std::path::PathBuf;

// Import from the personality module
mod tools {
    pub mod personality;
}

use tools::personality::api;

fn main() {
    println!("Testing Rust Personality Loading");
    println!("=" .repeat(50).as_str());

    // Get all personalities
    let personalities = api::list();
    println!("Total personalities loaded: {}", personalities.len());

    // Check for specific personalities
    let test_names = ["linus", "ada", "hanzo", "guido", "dennis", "ken"];
    for name in test_names.iter() {
        if let Some(p) = api::get(name) {
            println!("✓ {}: {} - {}...", name, p.programmer, &p.description[..50.min(p.description.len())]);
        } else {
            println!("✗ {}: NOT FOUND", name);
        }
    }

    // Check tag filtering
    let pioneers = api::filter_by_tags(&["pioneer".to_string()]);
    println!("\nPioneers found: {}", pioneers.len());

    // Final check
    if personalities.len() == 117 {
        println!("\n✅ SUCCESS: All 117 personalities loaded!");
    } else {
        println!("\n⚠️  WARNING: Expected 117 personalities, found {}", personalities.len());
    }
}