//! Generates and round-trips a large blank PPTX for repeatable scale testing.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn elapsed(started: Instant) -> Duration {
    Instant::now().saturating_duration_since(started)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let slide_count = args.next().as_deref().unwrap_or("35000").parse::<usize>()?;
    if slide_count == 0 {
        return Err("slide count must be at least one".into());
    }
    let output = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(format!("target/large-{slide_count}.pptx")));

    let started = Instant::now();
    let mut session = slides_pptx::load(&slides_pptx::create_blank_pptx())?;
    let seed = session.deck().slides[0].clone();
    session
        .deck_mut()
        .slides
        .reserve(slide_count.saturating_sub(1));
    for index in 1..slide_count {
        let mut slide = seed.clone();
        slide.id = format!("synthetic-slide-{index}");
        session.deck_mut().slides.push(slide);
    }
    let model_time = elapsed(started);

    let started = Instant::now();
    let bytes = slides_pptx::save(&session)?;
    let initial_save_time = elapsed(started);
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, &bytes)?;

    let started = Instant::now();
    let loaded = slides_pptx::load(&bytes)?;
    let load_time = elapsed(started);
    if loaded.deck().slides.len() != slide_count {
        return Err(format!(
            "round-trip changed slide count: expected {slide_count}, got {}",
            loaded.deck().slides.len()
        )
        .into());
    }

    let started = Instant::now();
    let saved_again = slides_pptx::save(&loaded)?;
    let no_edit_save_time = elapsed(started);
    if saved_again != bytes {
        return Err("no-edit save was not byte-identical".into());
    }

    println!("slides={slide_count}");
    println!("bytes={}", bytes.len());
    println!("model_ms={}", model_time.as_millis());
    println!("initial_save_ms={}", initial_save_time.as_millis());
    println!("load_ms={}", load_time.as_millis());
    println!("no_edit_save_ms={}", no_edit_save_time.as_millis());
    println!("output={}", output.display());
    Ok(())
}
