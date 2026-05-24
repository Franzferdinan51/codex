use serde::{Deserialize, Serialize};

/// Messages that agents can send to the Live Canvas to render interactive
/// components.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CanvasMessage {
    /// Clear the canvas.
    Clear,
    /// Render a single component.
    RenderComponent {
        id: String,
        component: Component,
    },
    /// Update an existing component in place.
    UpdateComponent {
        id: String,
        patch: ComponentPatch,
    },
    /// Remove a component.
    RemoveComponent {
        id: String,
    },
    /// Set canvas layout mode.
    SetLayout {
        layout: CanvasLayout,
    },
}

/// Top-level layout strategy for the canvas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasLayout {
    /// Stack components vertically.
    Stack,
    /// Two-column split.
    Split,
    /// Free-form absolute positioning.
    Freeform,
}

/// A renderable UI component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Component {
    Text { content: String, style: Option<TextStyle> },
    Button { label: String, action: String },
    Input { placeholder: Option<String>, value: Option<String> },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Chart { kind: ChartKind, data: ChartData },
    Form { fields: Vec<FormField> },
    Image { src: String, alt: Option<String> },
    Code { language: Option<String>, code: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TextStyle {
    Heading1,
    Heading2,
    Body,
    Caption,
    Monospace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartKind {
    Bar,
    Line,
    Pie,
    Scatter,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub labels: Vec<String>,
    pub datasets: Vec<Dataset>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    pub label: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormField {
    pub name: String,
    pub label: String,
    pub field_type: FieldType,
    pub required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Number,
    Boolean,
    Select { options: Vec<String> },
    Multiline,
}

/// Patch operations for updating an existing component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentPatch {
    pub content: Option<String>,
    pub style: Option<TextStyle>,
    pub value: Option<String>,
    pub rows: Option<Vec<Vec<String>>>,
    pub data: Option<ChartData>,
}

/// Protocol state machine for the canvas.
#[derive(Debug, Default)]
pub struct A2uiProtocol {
    components: Vec<(String, Component)>,
    layout: CanvasLayout,
}

impl A2uiProtocol {
    pub fn handle_message(
        &mut self,
        msg: CanvasMessage,
    ) -> anyhow::Result<crate::render::RenderPlan> {
        use CanvasMessage::*;

        match msg {
            Clear => {
                self.components.clear();
                Ok(crate::render::RenderPlan::Clear)
            }
            RenderComponent { id, component } => {
                self.components.push((id.clone(), component.clone()));
                Ok(crate::render::RenderPlan::Render { id, component })
            }
            UpdateComponent { id, patch } => {
                if let Some((_, comp)) = self.components.iter_mut().find(|(i, _)| i == &id) {
                    apply_patch(comp, patch)?;
                }
                Ok(crate::render::RenderPlan::Update { id })
            }
            RemoveComponent { id } => {
                self.components.retain(|(i, _)| i != &id);
                Ok(crate::render::RenderPlan::Remove { id })
            }
            SetLayout { layout } => {
                self.layout = layout;
                Ok(crate::render::RenderPlan::Relayout { layout: self.layout.clone() })
            }
        }
    }
}

fn apply_patch(component: &mut Component, patch: ComponentPatch) -> anyhow::Result<()> {
    if let Some(content) = patch.content {
        match component {
            Component::Text { content: c, .. } => *c = content,
            Component::Button { label, .. } => *label = content,
            Component::Code { code, .. } => *code = content,
            _ => {}
        }
    }
    if let Some(style) = patch.style {
        if let Component::Text { style: s, .. } = component {
            *s = Some(style);
        }
    }
    if let Some(value) = patch.value {
        if let Component::Input { value: v, .. } = component {
            *v = Some(value);
        }
    }
    if let Some(rows) = patch.rows {
        if let Component::Table { rows: r, .. } = component {
            *r = rows;
        }
    }
    if let Some(data) = patch.data {
        if let Component::Chart { data: d, .. } = component {
            *d = data;
        }
    }
    Ok(())
}
