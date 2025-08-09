use anyhow::{Context, Result};
use colored::*;
use jsonschema::JSONSchema;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("{}", "🔍 Just Learn Just Game - Schema Validator".blue().bold());
    println!("{}", "=".repeat(50).dimmed());

    let data_dir = Path::new("data");
    if !data_dir.exists() {
        anyhow::bail!("Data directory not found at: {}", data_dir.display());
    }

    // Load schema
    let schema_path = data_dir.join("schema.json");
    let schema_content = fs::read_to_string(&schema_path)
        .context(format!("Failed to read schema at {}", schema_path.display()))?;
    
    // Parse schema once to avoid lifetime issues
    let schema: &'static Value = Box::leak(Box::new(
        serde_json::from_str(&schema_content)
            .context("Failed to parse schema.json")?
    ));
    
    let compiled = JSONSchema::compile(schema)
        .context("Failed to compile JSON schema")?;
    
    println!("✅ Schema loaded: {}\n", schema_path.display());

    // Find and validate all question files
    let mut results = Vec::new();
    let mut file_count = 0;

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
            file_count += 1;
            print!("Validating {}... ", filename.cyan());
            
            let content = fs::read_to_string(path)?;
            let instance: Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}", "❌ JSON PARSE ERROR".red());
                    println!("  {} {}", "→".red(), e);
                    results.push((filename.to_string(), false));
                    continue;
                }
            };

            // Extract metadata for display
            let metadata = instance.get("metadata");
            let title = metadata
                .and_then(|m| m.get("title"))
                .and_then(|t| t.as_str())
                .unwrap_or("No title");
            
            let question_count = instance
                .get("questions")
                .and_then(|q| q.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            match compiled.validate(&instance) {
                Ok(_) => {
                    println!("{}", "✅".green());
                    println!("  {} Title: {}", "→".dimmed(), title);
                    println!("  {} Questions: {}", "→".dimmed(), question_count);
                    results.push((filename.to_string(), true));
                }
                Err(errors) => {
                    println!("{}", "❌".red());
                    for error in errors {
                        println!("  {} {}", "→".red(), error);
                    }
                    results.push((filename.to_string(), false));
                }
            }
            println!();
        }
    }

    // Summary
    println!("{}", "=".repeat(50).dimmed());
    println!("{}", "📊 Validation Summary".yellow().bold());
    println!("{}", "=".repeat(50).dimmed());
    
    let valid_count = results.iter().filter(|(_, valid)| *valid).count();
    let invalid_count = file_count - valid_count;
    
    for (filename, valid) in &results {
        let symbol = if *valid { "✅" } else { "❌" };
        let name = if *valid { 
            filename.green() 
        } else { 
            filename.red() 
        };
        println!("{} {}", symbol, name);
    }
    
    println!();
    if invalid_count == 0 {
        println!("🎉 {} All {} files are valid!", 
                 "Success!".green().bold(), 
                 file_count);
        Ok(())
    } else {
        println!("{} {} of {} files failed validation", 
                 "⚠️ Warning:".yellow().bold(),
                 invalid_count,
                 file_count);
        process::exit(1);
    }
}