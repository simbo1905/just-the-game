use anyhow::{anyhow, Context, Result};
use std::env;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::handler::viewport::Viewport;
use chromiumoxide::page::Page;
use chromiumoxide::page::ScreenshotParams;
use clap::Parser;
use colored::*;
use futures::StreamExt;
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Parser, Debug)]
#[command(author, version, about = "Data-driven browser test runner for just-learn-just game", long_about = None)]
struct Args {
    /// Run tests in headless mode
    #[arg(long)]
    headless: bool,

    /// Filter question sets by name pattern
    #[arg(long)]
    filter: Option<String>,

    /// Show verbose output including console logs
    #[arg(long, short = 'v')]
    verbose: bool,

    /// Path to the HTML file to test
    #[arg(long, default_value = "./index.html")]
    html_path: String,

    /// Timeout for page operations in seconds
    #[arg(long, default_value = "10")]
    timeout: u64,

    /// Run only one easy and one hard question set (first found of each)
    #[arg(long)]
    first_per_mode: bool,
}

#[derive(Debug, Deserialize)]
struct QuestionSet {
    metadata: Metadata,
    questions: Vec<Question>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Metadata {
    title: String,
    description: String,
    mode: String,
    #[serde(rename = "targetAge")]
    target_age: String,
    subject: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Question {
    id: String,
    question: String,
    choices: Vec<String>,
    #[serde(rename = "correctAnswer")]
    correct_answer: usize,
    explanation: String,
}

#[derive(Debug)]
struct TestCase {
    filename: String,
    key: String,
    metadata: Metadata,
    questions: Vec<Question>,
}

#[derive(Debug, Clone)]
struct TestResult {
    name: String,
    mode: String,
    passed: bool,
    error: Option<String>,
}

struct GameTestRunner {
    args: Args,
    browser: Browser,
    test_cases: Vec<TestCase>,
}

impl GameTestRunner {
    fn sanitize_for_filename(input: &str) -> String {
        input
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
            .collect()
    }

    async fn try_screenshot(&self, page: &Page, name_prefix: &str) {
        let _ = std::fs::create_dir_all("test_output");
        if let Ok(bytes) = page.screenshot(ScreenshotParams::default()).await {
            let filename = format!("test_output/{}.png", name_prefix);
            let _ = std::fs::write(filename, bytes);
        }
    }

    fn js_string_literal(input: &str) -> String {
        // Use JSON encoding to produce a safe JS string literal
        serde_json::to_string(input).unwrap_or_else(|_| "\"\"".to_string())
    }

    async fn new(args: Args) -> Result<Self> {
        println!("{}", "🎮 Starting Game Tests".bright_blue().bold());
        println!();

        // Discover test cases
        let test_cases = Self::discover_test_cases(&args)?;
        
        if test_cases.is_empty() {
            anyhow::bail!("No question sets found to test!");
        }

        // Setup browser
        let mut builder = BrowserConfig::builder()
            .args(vec!["--no-sandbox", "--disable-setuid-sandbox"])
            .viewport(Some(Viewport {
                width: 1280,
                height: 800,
                device_scale_factor: Some(1.0),
                emulating_mobile: false,
                is_landscape: false,
                has_touch: false,
            }));

        if !args.headless {
            builder = builder.with_head();
        }

        // Try to resolve Chrome/Chromium executable for local runs
        if let Ok(exec_override) = env::var("CHROME") {
            builder = builder.chrome_executable(exec_override);
        } else {
            #[cfg(target_os = "macos")]
            {
                let default_mac = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
                if std::path::Path::new(default_mac).exists() {
                    builder = builder.chrome_executable(default_mac);
                }
            }
        }

        let browser_config = builder.build().map_err(|e| anyhow!(e))?;

        let (browser, mut handler) = Browser::launch(browser_config)
            .await
            .context("Failed to launch browser")?;

        // Spawn browser handler
        tokio::spawn(async move {
            while let Some(_) = handler.next().await {}
        });

        Ok(Self {
            args,
            browser,
            test_cases,
        })
    }

    fn discover_test_cases(args: &Args) -> Result<Vec<TestCase>> {
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            anyhow::bail!("data/ directory not found");
        }

        let mut test_cases = Vec::new();

        // Find all questions*.json files
        for entry in std::fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                if filename.starts_with("questions") && filename.ends_with(".json") {
                    // Extract key (filename without extension)
                    let key = filename.trim_end_matches(".json");
                    
                    // Apply filter if provided
                    if let Some(filter) = &args.filter {
                        if !key.contains(filter) {
                            continue;
                        }
                    }

                    // Load and parse JSON
                    let content = std::fs::read_to_string(&path)
                        .with_context(|| format!("Failed to read {}", path.display()))?;
                    
                    let question_set: QuestionSet = serde_json::from_str(&content)
                        .with_context(|| format!("Failed to parse {}", path.display()))?;

                    test_cases.push(TestCase {
                        filename: filename.to_string(),
                        key: key.to_string(),
                        metadata: question_set.metadata,
                        questions: question_set.questions,
                    });
                }
            }
        }

        // Sort test cases for consistent ordering
        test_cases.sort_by(|a, b| a.filename.cmp(&b.filename));

        // Print discovered test cases
        println!("{} Found {} question set(s) to test:", "📋".green(), test_cases.len());
        for tc in &test_cases {
            println!(
                "  {} {}: {} (mode: {})",
                "•".dimmed(),
                tc.key.bright_white(),
                tc.metadata.title,
                if tc.metadata.mode == "easy" {
                    tc.metadata.mode.green()
                } else {
                    tc.metadata.mode.yellow()
                }
            );
        }
        println!();

        Ok(test_cases)
    }

    async fn run_all_tests(&self) -> Vec<TestResult> {
        let mut results = Vec::new();

        // Optionally restrict to first easy and first hard
        let selected_cases: Vec<&TestCase> = if self.args.first_per_mode {
            let mut easy: Option<&TestCase> = None;
            let mut hard: Option<&TestCase> = None;
            for tc in &self.test_cases {
                match tc.metadata.mode.as_str() {
                    "easy" if easy.is_none() => easy = Some(tc),
                    "hard" if hard.is_none() => hard = Some(tc),
                    _ => {}
                }
                if easy.is_some() && hard.is_some() { break; }
            }
            [easy, hard].into_iter().flatten().collect()
        } else {
            self.test_cases.iter().collect()
        };

        for test_case in selected_cases {
            let result = self.run_test_case(test_case).await;
            let passed = result.passed;
            results.push(result);
            // Fail fast: stop after first failure
            if !passed { break; }
        }

        results
    }

    async fn run_test_case(&self, test_case: &TestCase) -> TestResult {
        println!(
            "{} Testing: {} ({})",
            "🧪".bright_blue(),
            test_case.metadata.title.bright_white(),
            test_case.metadata.mode
        );

        match self.test_question_set(test_case).await {
            Ok(_) => {
                println!("  {} Passed\n", "✅".green());
                TestResult {
                    name: test_case.metadata.title.clone(),
                    mode: test_case.metadata.mode.clone(),
                    passed: true,
                    error: None,
                }
            }
            Err(e) => {
                println!("  {} Failed: {}\n", "❌".red(), e);
                TestResult {
                    name: test_case.metadata.title.clone(),
                    mode: test_case.metadata.mode.clone(),
                    passed: false,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    async fn test_question_set(&self, test_case: &TestCase) -> Result<()> {
        // Create new page for this test
        let page = self.browser.new_page("about:blank").await?;
        
        // Set up console listener if verbose
        if self.args.verbose {
            let mut console_events = page.event_listener::<chromiumoxide::cdp::js_protocol::runtime::EventConsoleApiCalled>().await?;
            
            tokio::spawn(async move {
                while let Some(event) = console_events.next().await {
                    if let Some(args) = event.args.get(0) {
                        if let Some(value) = &args.value {
                            if let Some(text) = value.as_str() {
                                println!("    {} {}", "🌐".dimmed(), text.dimmed());
                            }
                        }
                    }
                }
            });
        }

        // Navigate to game
        let html_path = std::fs::canonicalize(&self.args.html_path)
            .context("Failed to resolve HTML path")?;
        let file_url = format!("file://{}", html_path.display());
        
        page.goto(&file_url)
            .await
            .context("Failed to navigate to game")?;

        // Wait for start screen
        self.wait_for_element(&page, "#start-screen").await?;
        
        // Verify start screen is visible
        let start_visible = page
            .evaluate("document.getElementById('start-screen').style.display !== 'none'")
            .await?
            .into_value::<bool>()?;
        
        if !start_visible {
            self.try_screenshot(&page, "fail_start_not_visible").await;
            anyhow::bail!("Start screen not visible");
        }

        println!("    Testing start screen... ✓");

        // Click the tile for this question set
        let tile_selector = format!("[data-key='{}']", test_case.key);
        // Ensure tiles are rendered before clicking
        self.wait_for_element(&page, ".question-set-tile").await?;
        // Debug: list available tiles and take a screenshot of the start screen
        if let Ok(eval) = page
            .evaluate("Array.from(document.querySelectorAll('.question-set-tile')).map(el => el.dataset.key)")
            .await
        {
            if let Ok(keys) = eval.into_value::<Vec<String>>() {
                println!("    Available tiles: {:?}", keys);
            }
        }
        self.try_screenshot(&page, &format!("tiles_present_before_click_{}", test_case.key)).await;
        self.wait_for_element(&page, &tile_selector).await?;
        // Before click, take a screenshot for debugging
        let _ = std::fs::create_dir_all("test_output");
        if let Ok(bytes) = page.screenshot(ScreenshotParams::default()).await {
            let _ = std::fs::write(format!("test_output/before_click_{}.png", test_case.key), bytes);
        }
        self.click_element(&page, &tile_selector).await
            .with_context(|| format!("Failed to find tile with data-key='{}'", test_case.key))?;

        // Wait for game screen
        self.wait_for_element(&page, "#game-screen").await?;
        
        // Test questions per mode
        let total_questions = test_case.questions.len();
        if test_case.metadata.mode == "hard" {
            for question_num in 1..=total_questions {
                // Wait for choice buttons
                self.wait_for_element(&page, ".choice-button").await?;
                // Always click the first choice
                self.click_element(&page, ".choice-button:nth-child(1)").await?;
                // Wait for result screen and click next
                self.wait_for_element(&page, "#result-screen").await?;
                self.click_element(&page, "#next-button").await?;
                println!("    [hard] Testing question {}/{}... ✓", question_num, total_questions);
            }
        } else {
            for (i, question) in test_case.questions.iter().enumerate() {
                let question_num = i + 1;
                // Wait for choice buttons
                self.wait_for_element(&page, ".choice-button").await?;

                if question_num == 1 {
                    // First do a wrong answer attempt (stay on game screen), then correct
                    let wrong_index = if question.correct_answer == 0 { 1 } else { 0 };
                    let wrong_selector = format!(".choice-button:nth-child({})", wrong_index + 1);
                    self.click_element(&page, &wrong_selector).await?;
                    // Small pause, still on game screen
                    sleep(Duration::from_millis(500)).await;
                    // Now answer correctly
                    self.click_correct_answer(&page, question).await?;
                } else {
                    // For remaining questions, answer correctly directly
                    self.click_correct_answer(&page, question).await?;
                }

                // Wait for result screen and click next, unless it's the final question where next leads to finish
                self.wait_for_element(&page, "#result-screen").await?;
                self.click_element(&page, "#next-button").await?;
                println!("    [easy] Testing question {}/{}... ✓", question_num, total_questions);
            }
        }

        // Should be on finish screen
        self.wait_for_element(&page, "#finish-screen").await?;
        // After finishing, take a screenshot
        if let Ok(bytes) = page.screenshot(ScreenshotParams::default()).await {
            let _ = std::fs::write(format!("test_output/finish_{}.png", test_case.key), bytes);
        }
        
        // Note: Skipping Play Again (reload) to avoid invalidating devtools context

        // Close page
        page.close().await?;

        Ok(())
    }

    

    async fn click_correct_answer(&self, page: &Page, question: &Question) -> Result<()> {
        let correct_selector = format!(".choice-button:nth-child({})", question.correct_answer + 1);
        self.click_element(page, &correct_selector).await
    }

    async fn wait_for_element(&self, page: &Page, selector: &str) -> Result<()> {
        let timeout = Duration::from_secs(self.args.timeout);
        let start = std::time::Instant::now();
        
        loop {
            let exists = match page
                .evaluate(format!(
                    "document.querySelector({}) !== null",
                    Self::js_string_literal(selector)
                ))
                .await
            {
                Ok(eval) => eval.into_value::<bool>().unwrap_or(false),
                Err(_) => false,
            };
            
            if exists {
                return Ok(());
            }
            
            if start.elapsed() > timeout {
                let safe = Self::sanitize_for_filename(selector);
                let name = format!("fail_timeout_wait_for_{}", safe);
                self.try_screenshot(page, &name).await;
                anyhow::bail!("Timeout waiting for element: {}", selector);
            }
            
            sleep(Duration::from_millis(100)).await;
        }
    }

    async fn click_element(&self, page: &Page, selector: &str) -> Result<()> {
        page.evaluate(format!(
            "document.querySelector({}).click()",
            Self::js_string_literal(selector)
        ))
            .await
            .with_context(|| format!("Failed to click element: {}", selector))?;
        Ok(())
    }

    fn print_summary(&self, results: &[TestResult]) {
        println!("{}", "📊 Test Summary".bright_blue().bold());
        println!("{}", "================================".dimmed());
        
        for result in results {
            if result.passed {
                println!(
                    "{} {} {} ({})",
                    "✅".green(),
                    "PASSED".green().bold(),
                    result.name,
                    result.mode
                );
            } else {
                println!(
                    "{} {} {} ({})",
                    "❌".red(),
                    "FAILED".red().bold(),
                    result.name,
                    result.mode
                );
                if let Some(error) = &result.error {
                    println!("    {}", error.red());
                }
            }
        }
        
        println!();
        
        let passed = results.iter().filter(|r| r.passed).count();
        let total = results.len();
        
        println!("Results: {}/{} question sets passed", passed, total);
        
        if passed == total {
            println!("{} Success! All tests passed!", "🎉".green());
        } else {
            println!("{} Some tests failed", "💔".red());
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut runner = GameTestRunner::new(args).await?;
    let results = runner.run_all_tests().await;
    runner.print_summary(&results);
    
    // Return appropriate exit code
    let all_passed = results.iter().all(|r| r.passed);
    // Close browser before exiting to avoid background task lingering
    runner.browser.close().await.ok();
    std::process::exit(if all_passed { 0 } else { 1 });
}