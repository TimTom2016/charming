use serde::ser::SerializeSeq;
use serde::Serialize;

pub enum Datum {
    Number(f64),
    String(String),
}

impl Serialize for Datum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Datum::Number(number) => number.serialize(serializer),
            Datum::String(string) => string.serialize(serializer),
        }
    }
}

impl From<f64> for Datum {
    fn from(number: f64) -> Self {
        Datum::Number(number)
    }
}

impl From<u64> for Datum {
    fn from(number: u64) -> Self {
        Datum::Number(number as f64)
    }
}

impl From<&str> for Datum {
    fn from(string: &str) -> Self {
        Datum::String(string.to_string())
    }
}

impl From<String> for Datum {
    fn from(string: String) -> Self {
        Datum::String(string)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct InputSelector {
    from_dataset_index: u64,
    from_transform_result: u64,
}

pub type DataFrame = Vec<Vec<Datum>>;

pub struct Dataset {
    source: DataFrame,
    transforms: Vec<serde_json::Value>,
    input_selector: InputSelector,
}

impl Dataset {
    pub fn new() -> Self {
        Self {
            source: vec![],
            transforms: vec![],
            input_selector: InputSelector {
                from_dataset_index: 0,
                from_transform_result: 0,
            },
        }
    }

    pub fn source(mut self, source: DataFrame) -> Self {
        self.source = source;
        self
    }

    pub fn transform(mut self, transform: &str) -> Self {
        self.transforms
            .push(serde_json::from_str(transform).unwrap());
        self
    }

    pub fn from_dataset_index(mut self, from_dataset_index: u64) -> Self {
        self.input_selector.from_dataset_index = from_dataset_index;
        self
    }

    pub fn from_transform_result(mut self, from_transform_result: u64) -> Self {
        self.input_selector.from_transform_result = from_transform_result;
        self
    }
}

impl Serialize for Dataset {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct SourceHelper<'a> {
            source: &'a DataFrame,
        }

        #[derive(Serialize)]
        struct TransformHelper<'a> {
            transform: &'a serde_json::Value,
        }

        let mut s = serializer.serialize_seq(Some(2 + self.transforms.len()))?;
        s.serialize_element(&SourceHelper {
            source: &self.source,
        })?;
        for transform in &self.transforms {
            s.serialize_element(&TransformHelper { transform })?;
        }
        s.serialize_element(&self.input_selector)?;
        s.end()
    }
}