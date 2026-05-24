use crate::ChatWidget;
use ratatui::text::{Line, Span};
use codex_tui_styles::Stylize;

impl ChatWidget {
    pub fn add_provider_output(&mut self) {
        let mut lines: Vec<Line<'static>> = Vec::new();

        // Title
        lines.push(Line::from(vec![
            "Provider Information".bold(),
        ]));
        lines.push(Line::from(""));

        // Current provider
        let provider_name = self.config.provider.as_deref().unwrap_or("default");
        lines.push(Line::from(vec![
            "  Provider:  ".dim(),
            provider_name.cyan().bold(),
        ]));

        // Current model
        let model_name = self.config.model.as_deref().unwrap_or("default");
        lines.push(Line::from(vec![
            "  Model:     ".dim(),
            model_name.cyan(),
        ]));

        // API endpoint / base URL
        let base_url = self.config.base_url.as_deref().unwrap_or("(not configured)");
        lines.push(Line::from(vec![
            "  Base URL:  ".dim(),
            base_url.into(),
        ]));

        lines.push(Line::from(""));

        // Configured parameters
        lines.push(Line::from(vec![
            "Parameters".bold(),
        ]));

        if let Some(ref params) = self.config.model_parameters {
            for (key, value) in params {
                lines.push(Line::from(vec![
                    format!("  {key}: ").dim(),
                    format!("{value}").into(),
                ]));
            }
        } else {
            lines.push(Line::from(vec![
                "  (no custom parameters)".dim(),
            ]));
        }

        lines.push(Line::from(""));

        // Provider status
        lines.push(Line::from(vec![
            "Status".bold(),
        ]));

        let status = if self.last_error.is_some() {
            "error".red()
        } else {
            "connected".green()
        };
        lines.push(Line::from(vec![
            "  Status:    ".dim(),
            status,
        ]));

        if let Some(ref err) = self.last_error {
            lines.push(Line::from(vec![
                "  Error:     ".dim(),
                err.as_str().red(),
            ]));
        }

        self.add_output(lines);
    }
}
