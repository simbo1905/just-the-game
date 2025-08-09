use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use handlebars::Handlebars;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("{}", "📦 Just Learn Just Game - Build Tool".blue().bold());
    println!("{}", "=".repeat(50).dimmed());

    // Step 1: Load all question sets
    println!("{}", "Loading question sets...".yellow());
    let question_sets = load_question_sets()?;
    println!("  {} Loaded {} question sets", "→".green(), question_sets.len());

    // Step 2: Load CSS and JS
    println!("\n{}", "Loading assets...".yellow());
    let css_content = fs::read_to_string("css/style.css")
        .context("Failed to read css/style.css")?;
    println!("  {} Loaded CSS ({}kb)", "→".green(), css_content.len() / 1024);
    
    let js_engine = fs::read_to_string("js/game-engine.js")
        .context("Failed to read js/game-engine.js")?;
    let js_ui = fs::read_to_string("js/ui.js")
        .context("Failed to read js/ui.js")?;
    let js_content = format!("{}\n\n{}", js_engine, js_ui);
    println!("  {} Loaded JS ({}kb)", "→".green(), js_content.len() / 1024);

    // Step 3: Generate build metadata
    println!("\n{}", "Generating build metadata...".yellow());
    let build_info = generate_build_info();
    println!("  {} Version: {}", "→".green(), build_info.version);
    println!("  {} Timestamp: {}", "→".green(), build_info.timestamp);

    // Step 4: Prepare template data
    let template_data = json!({
        "question_sets_json": serde_json::to_string_pretty(&question_sets)?,
        "default_question_set": question_sets.get("questions")
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .or_else(|| {
                // If "questions" doesn't exist, use first available set
                question_sets.iter().next()
                    .map(|(_, v)| serde_json::to_string_pretty(v).unwrap_or_default())
            }),
        "css_content": css_content,
        "js_content": js_content,
        "build_timestamp": build_info.timestamp,
        "build_timestamp_unix": build_info.timestamp_unix,
        "version": build_info.version,
    });

    // Step 5: Render template
    println!("\n{}", "Rendering template...".yellow());
    let output = render_template(template_data)?;
    
    // Step 6: Write output
    let output_path = "index.html";
    fs::write(output_path, output)
        .context(format!("Failed to write {}", output_path))?;
    
    let output_size = fs::metadata(output_path)?.len() / 1024;
    println!("  {} Written {} ({}kb)", "→".green(), output_path, output_size);

    // Success!
    println!("\n{}", "=".repeat(50).dimmed());
    println!("✨ {} Build complete!", "Success!".green().bold());
    println!("🎯 Open index.html in any browser to play");
    
    Ok(())
}

fn load_question_sets() -> Result<HashMap<String, Value>> {
    let mut question_sets = HashMap::new();
    let data_dir = Path::new("data");
    
    for entry in WalkDir::new(data_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        if filename.starts_with("questions") && filename.ends_with(".json") {
            let key = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            
            let content = fs::read_to_string(path)?;
            let data: Value = serde_json::from_str(&content)?;
            
            // Get title for display
            let title = data.get("metadata")
                .and_then(|m| m.get("title"))
                .and_then(|t| t.as_str())
                .unwrap_or(&key);
            
            println!("    {} {}: {}", "•".dimmed(), key.cyan(), title.dimmed());
            question_sets.insert(key, data);
        }
    }
    
    if question_sets.is_empty() {
        anyhow::bail!("No question files found in data/");
    }
    
    Ok(question_sets)
}

struct BuildInfo {
    version: String,
    timestamp: String,
    timestamp_unix: i64,
}

fn generate_build_info() -> BuildInfo {
    let timestamp = Local::now();
    
    // Try to get git version
    let version = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| format!("git-{}", s.trim()))
        .unwrap_or_else(|| "dev-build".to_string());
    
    BuildInfo {
        version,
        timestamp: timestamp.to_rfc3339(),
        timestamp_unix: timestamp.timestamp(),
    }
}

fn render_template(data: Value) -> Result<String> {
    // Use the Handlebars template
    let template_path = Path::new("templates/index.hbs");
    
    if !template_path.exists() {
        anyhow::bail!("Template not found at templates/index.hbs");
    }
    
    let template_content = fs::read_to_string(template_path)?;
    
    let mut handlebars = Handlebars::new();
    // Disable HTML escaping since we're embedding raw HTML/JS/CSS
    handlebars.register_escape_fn(handlebars::no_escape);
    
    let rendered = handlebars.render_template(&template_content, &data)
        .context("Failed to render template")?;
    
    Ok(rendered)
}

