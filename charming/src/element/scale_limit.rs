use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleLimit {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i64>,
}

impl ScaleLimit {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    pub fn min<F: Into<i64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<i64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }
}
