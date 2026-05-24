use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::chatwidget::protocol::ProviderInfo;

/// Provider information popup showing current provider details
pub struct ProviderPopup {
    pub provider_info: ProviderInfo,
}

impl ProviderPopup {
    pub fn new(provider_info: ProviderInfo) -> Self {
        Self { provider_info }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        // Provider name
        let provider_block = Block::default()
            .title(" Provider ".cyan().bold())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let provider_text = format!("  {}", self.provider_info.name);
        let provider_para = Paragraph::new(provider_text.bold())
            .block(provider_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(provider_para, chunks[0]);

        // Model name
        let model_block = Block::default()
            .title(" Model ".cyan().bold())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let model_text = format!("  {}", self.provider_info.model);
        let model_para = Paragraph::new(model_text.green())
            .block(model_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(model_para, chunks[1]);

        // API endpoint
        let endpoint_block = Block::default()
            .title(" API Endpoint ".cyan().bold())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let endpoint_text = format!("  {}", self.provider_info.endpoint);
        let endpoint_para = Paragraph::new(endpoint_text.dim())
            .block(endpoint_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(endpoint_para, chunks[2]);

        // Status
        let status_block = Block::default()
            .title(" Status ".cyan().bold())
            .borders(Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

        let status_text = if self.provider_info.is_configured {
            "  Connected".green()
        } else {
            "  Not Configured".red()
        };
        let status_para = Paragraph::new(status_text)
            .block(status_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(status_para, chunks[3]);

        // Parameters
        if !self.provider_info.parameters.is_empty() {
            let params_block = Block::default()
                .title(" Parameters ".cyan().bold())
                .borders(Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

            let params_text = self
                .provider_info
                .parameters
                .iter()
                .map(|(k, v)| format!("  {}: {}", k.dim(), v))
                .collect::<Vec<_>>()
                .join("\n");

            let params_para = Paragraph::new(params_text)
                .block(params_block)
                .wrap(Wrap { trim: true });
            frame.render_widget(params_para, chunks[4]);
        }
    }
}
