#[derive(Debug, Default, serde::Serialize)]
#[serde(transparent)]
pub struct Status {
    pub data: Vec<StatusData>,
}

impl Status {
    pub fn new() -> Status {
        Status { data: vec![] }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct StatusData {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub markup: Markup,
    pub full_text: String,
}

impl StatusData {
    pub fn attach_right(&mut self, right: StatusData, separator: &str) {
        self.name = format!("{}_{}", self.name, right.name);
        self.full_text = format!("{}{}{}", self.full_text, separator, right.full_text);
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.full_text.is_empty()
    }
}

impl Default for StatusData {
    fn default() -> Self {
        StatusData {
            name: String::new(),
            color: None,
            markup: Markup::Pango,
            full_text: String::new(),
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub enum Markup {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pango")]
    Pango,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializing() {
        let sd = StatusData {
            name: "test".to_string(),
            color: None,
            markup: Markup::Pango,
            full_text: "test text".to_string(),
        };

        dbg!(serde_json::to_string(&sd)).unwrap();
    }
}
