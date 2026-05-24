use crate::protocol::{CanvasLayout, Component};

/// Instruction produced by the A2UI protocol engine telling the renderer
/// what to do.
#[derive(Debug, Clone, PartialEq)]
pub enum RenderPlan {
    /// Clear the entire canvas.
    Clear,
    /// Render a new component.
    Render { id: String, component: Component },
    /// Update an existing component.
    Update { id: String },
    /// Remove a component.
    Remove { id: String },
    /// Change the overall layout.
    Relayout { layout: CanvasLayout },
}

/// Minimal text-mode renderer for the canvas (stub for TUI integration).
/// In a full implementation this would produce ratatui widgets.
pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, plan: &RenderPlan) -> String {
        match plan {
            RenderPlan::Clear => "[Canvas cleared]".to_string(),
            RenderPlan::Render { id, component } => {
                format!("[Render {id}: {:?}]", component_type_name(component))
            }
            RenderPlan::Update { id } => format!("[Update {id}]"),
            RenderPlan::Remove { id } => format!("[Remove {id}]"),
            RenderPlan::Relayout { layout } => format!("[Relayout: {layout:?}]"),
        }
    }
}

fn component_type_name(component: &Component) -> &'static str {
    match component {
        Component::Text { .. } => "text",
        Component::Button { .. } => "button",
        Component::Input { .. } => "input",
        Component::Table { .. } => "table",
        Component::Chart { .. } => "chart",
        Component::Form { .. } => "form",
        Component::Image { .. } => "image",
        Component::Code { .. } => "code",
    }
}
