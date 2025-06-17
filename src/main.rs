use std::collections::HashMap;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use rand::prelude::*;
use rand::rng;

#[derive(Debug, Clone)]
enum Personality {
    Rusher,    // Fast typing, more typos
    Careful,   // Slower, fewer mistakes  
    Refactorer, // Often deletes and rewrites
}

struct TypingSimulator {
    rng: ThreadRng,
    current_personality: Personality,
    personality_counter: u32,
    speed_multiplier: f64, // 1.0 = normal, 0.5 = half speed, 2.0 = double speed
    function_names: Vec<&'static str>,
    variable_names: Vec<&'static str>,
    struct_names: Vec<&'static str>,
    code_templates: Vec<&'static str>,
}

impl TypingSimulator {
    fn new() -> Self {
        Self {
            rng: rng(),
            current_personality: Personality::Careful,
            personality_counter: 0,
            speed_multiplier: 6.0, // Change this to adjust overall speed
            function_names: vec![
                "process_data", "handle_request", "validate_input", "parse_config",
                "connect_database", "serialize_response", "authenticate_user",
                "calculate_hash", "compress_file", "decrypt_message", "build_query",
                "format_output", "check_permissions", "load_settings", "save_cache"
            ],
            variable_names: vec![
                "result", "data", "config", "user", "response", "query", "buffer",
                "content", "payload", "status", "error", "value", "key", "item",
                "count", "index", "path", "url", "token", "session"
            ],
            struct_names: vec![
                "Config", "User", "Request", "Response", "Database", "Cache",
                "Session", "Logger", "Parser", "Handler", "Client", "Server",
                "Message", "Event", "Task", "Job", "Queue", "State"
            ],
            code_templates: vec![
                "fn {fn_name}() -> Result<{type}, Box<dyn std::error::Error>> {\n    let {var} = \"{value}\";\n    println!(\"Processing: {{}}\", {var});\n    Ok({var}.to_string())\n}\n",
                "struct {struct_name} {\n    {field}: String,\n    {field2}: u32,\n    active: bool,\n}\n",
                "impl {struct_name} {\n    fn new({param}: &str) -> Self {\n        Self {\n            {field}: {param}.to_string(),\n            {field2}: 0,\n            active: true,\n        }\n    }\n}\n",
                "// TODO: implement {feature} functionality\nfn {fn_name}({param}: &str) -> Option<String> {\n    if {param}.is_empty() {\n        return None;\n    }\n    Some({param}.to_uppercase())\n}\n",
                "use std::collections::HashMap;\n\nfn {fn_name}() -> HashMap<String, {type}> {\n    let mut {var} = HashMap::new();\n    {var}.insert(\"key\".to_string(), \"{value}\".to_string());\n    {var}\n}\n",
                "async fn {fn_name}({param}: &str) -> Result<String, reqwest::Error> {\n    let {var} = reqwest::get({param}).await?;\n    let {result} = {var}.text().await?;\n    Ok({result})\n}\n",
                "#[derive(Debug, Clone)]\npub struct {struct_name} {\n    pub {field}: Vec<String>,\n    pub {field2}: Option<u32>,\n}\n"
            ],
        }
    }

    fn maybe_switch_personality(&mut self) {
        self.personality_counter += 1;
        if self.personality_counter > self.rng.random_range(5..15) {
            let old_personality = self.current_personality.clone();
            self.current_personality = match self.rng.random_range(0..3) {
                0 => Personality::Rusher,
                1 => Personality::Careful,
                _ => Personality::Refactorer,
            };
            self.personality_counter = 0;
            
            eprintln!("DEBUG: Switching from {:?} to {:?}", old_personality, self.current_personality);
        }
    }

    fn get_typing_delay(&mut self) -> Duration {
        let base_delay = match self.current_personality {
            Personality::Rusher => Duration::from_millis(self.rng.random_range(20..80)),
            Personality::Careful => Duration::from_millis(self.rng.random_range(80..150)),
            Personality::Refactorer => Duration::from_millis(self.rng.random_range(60..120)),
        };
        Duration::from_millis((base_delay.as_millis() as f64 / self.speed_multiplier) as u64)
    }

    fn get_thinking_pause(&mut self) -> Duration {
        let base_pause = match self.current_personality {
            Personality::Rusher => Duration::from_millis(self.rng.random_range(200..800)),
            Personality::Careful => Duration::from_millis(self.rng.random_range(800..2000)),
            Personality::Refactorer => Duration::from_millis(self.rng.random_range(500..1500)),
        };
        Duration::from_millis((base_pause.as_millis() as f64 / self.speed_multiplier) as u64)
    }

    fn should_make_typo(&mut self) -> bool {
        let chance = match self.current_personality {
            Personality::Rusher => 0.08,  // 8% chance
            Personality::Careful => 0.02, // 2% chance  
            Personality::Refactorer => 0.05, // 5% chance
        };
        self.rng.random::<f64>() < chance
    }

    fn make_typo(&mut self, c: char) -> char {
        let keyboard = "qwertyuiopasdfghjklzxcvbnm";
        let pos = keyboard.find(c.to_ascii_lowercase());
        
        if let Some(idx) = pos {
            let nearby: Vec<char> = keyboard.chars().enumerate()
                .filter(|(i, _)| (*i as i32 - idx as i32).abs() <= 2)
                .map(|(_, ch)| ch)
                .collect();
            
            if !nearby.is_empty() {
                return *nearby.choose(&mut self.rng).unwrap();
            }
        }
        
        // Fallback random char
        *"qwertyuiop".chars().collect::<Vec<_>>().choose(&mut self.rng).unwrap()
    }

    fn type_char(&self, c: char) {
        print!("{}", c);
        io::stdout().flush().unwrap();
    }

    fn backspace(&self) {
        print!("\x08 \x08");
        io::stdout().flush().unwrap();
    }

    fn type_text_with_errors(&mut self, text: &str) {
        for c in text.chars() {
            // Maybe pause to "think"
            if " \n{}();".contains(c) && self.rng.random::<f64>() < 0.1 {
                thread::sleep(self.get_thinking_pause());
            }

            // Check if we should make a typo (only on letters)
            if c.is_alphabetic() && self.should_make_typo() {
                let typo = self.make_typo(c);
                self.type_char(typo);
                thread::sleep(self.get_typing_delay());
                
                // Pause before realizing mistake
                let mistake_pause = Duration::from_millis(self.rng.random_range(100..500));
                thread::sleep(Duration::from_millis((mistake_pause.as_millis() as f64 / self.speed_multiplier) as u64));
                
                // Backspace and correct
                self.backspace();
                thread::sleep(Duration::from_millis(50));
                self.type_char(c);
            } else {
                self.type_char(c);
            }
            
            thread::sleep(self.get_typing_delay());
        }
    }

    fn should_refactor(&mut self) -> bool {
        matches!(self.current_personality, Personality::Refactorer) 
            && self.rng.random::<f64>() < 0.15
    }

    fn delete_and_retype(&mut self, original: &str) {
        eprintln!("DEBUG: Refactoring code block");
        
        // Delete some characters
        let delete_count = self.rng.random_range(5..original.len().min(30));
        for _ in 0..delete_count {
            self.backspace();
            thread::sleep(Duration::from_millis(30));
        }
        
        // Pause to "think" about rewrite
        let rewrite_pause = Duration::from_millis(self.rng.random_range(800..2000));
        thread::sleep(Duration::from_millis((rewrite_pause.as_millis() as f64 / self.speed_multiplier) as u64));
        
        // Retype with slight variations
        let remaining = &original[original.len().saturating_sub(delete_count)..];
        self.type_text_with_errors(remaining);
    }

    fn generate_code(&mut self) -> String {
        let template = self.code_templates.choose(&mut self.rng).unwrap();
        
        let mut replacements = HashMap::new();
        replacements.insert("{fn_name}", *self.function_names.choose(&mut self.rng).unwrap());
        replacements.insert("{struct_name}", *self.struct_names.choose(&mut self.rng).unwrap());
        replacements.insert("{var}", *self.variable_names.choose(&mut self.rng).unwrap());
        replacements.insert("{param}", *self.variable_names.choose(&mut self.rng).unwrap());
        replacements.insert("{field}", *self.variable_names.choose(&mut self.rng).unwrap());
        replacements.insert("{field2}", *self.variable_names.choose(&mut self.rng).unwrap());
        replacements.insert("{result}", *self.variable_names.choose(&mut self.rng).unwrap());
        replacements.insert("{type}", if self.rng.random() { "String" } else { "u32" });
        replacements.insert("{value}", if self.rng.random() { "default" } else { "test_data" });
        replacements.insert("{feature}", *["authentication", "caching", "validation", "logging"].choose(&mut self.rng).unwrap());

        let mut code = template.to_string();
        for (placeholder, replacement) in replacements {
            code = code.replace(placeholder, replacement);
        }
        
        code
    }

    fn run(&mut self) {
        eprintln!("DEBUG: Starting Byte Surge - Press Ctrl+C to stop");
        
        loop {
            self.maybe_switch_personality();
            
            let code = self.generate_code();
            
            // Maybe add some thinking time before starting
            if self.rng.random::<f64>() < 0.3 {
                thread::sleep(self.get_thinking_pause());
            }
            
            // Check if we should refactor partway through
            let should_refactor = self.should_refactor();
            let refactor_point = if should_refactor {
                Some(self.rng.random_range(code.len() / 3..code.len() * 2 / 3))
            } else {
                None
            };
            
            for (i, c) in code.char_indices() {
                // Check if we should refactor at this point
                if let Some(refactor_pos) = refactor_point {
                    if i == refactor_pos {
                        self.delete_and_retype(&code[i..]);
                        break;
                    }
                }
                
                self.type_text_with_errors(&c.to_string());
            }
            
            // Add some breathing room between code blocks
            self.type_text_with_errors("\n\n");
            let block_pause = Duration::from_millis(self.rng.random_range(500..1500));
            thread::sleep(Duration::from_millis((block_pause.as_millis() as f64 / self.speed_multiplier) as u64));
        }
    }
}

fn main() {
    let mut simulator = TypingSimulator::new();
    simulator.run();
}