use std::env;
use std::path::PathBuf;
use tera::Tera;

#[derive(Clone)]
pub struct TemplateEngine {
    pub tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let template_dir = Self::find_template_dir();
        let template_pattern = template_dir.join("**/*.html");

        tracing::info!("📂 Looking for templates in: {:?}", template_dir);

        let mut tera = match Tera::new(template_pattern.to_str().unwrap()) {
            Ok(t) => {
                tracing::info!("✅ Loaded {} templates", t.templates.len());
                t
            }
            Err(e) => {
                tracing::error!("❌ Template error: {}", e);
                tracing::error!("   Tried path: {:?}", template_pattern);
                std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![".html"]);
        Self { tera }
    }

    fn find_template_dir() -> PathBuf {
        if let Ok(dir) = env::var("TEMPLATE_DIR") {
            tracing::info!("🔧 Using TEMPLATE_DIR from environment: {}", dir);
            return PathBuf::from(dir);
        }

        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let templates = exe_dir.join("templates");
                if templates.exists() {
                    tracing::info!("🔧 Using templates next to executable");
                    return templates;
                }
            }
        }

        tracing::info!("🔧 Using templates from current directory");
        PathBuf::from("templates")
    }
}
