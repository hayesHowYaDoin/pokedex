use color_eyre::{
    eyre::bail,
    Result,
};
use num_traits::bounds::Bounded;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartFieldValue<const M: i32>(i32);

impl<const M: i32> Bounded for ChartFieldValue<M> {
    fn min_value() -> Self { ChartFieldValue(0) }
    fn max_value() -> Self { ChartFieldValue(M) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartField<const M: i32> {
    value: ChartFieldValue<M>,
    label: String,
}

impl<const M: i32> ChartField<M> {
    pub fn get_value(&self) -> i32 {
        self.value.0
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxChart<const M: i32> {
    fields: Vec<ChartField<M>>,
}

impl<const M: i32> MaxChart<M> {
    pub fn new(values: &[i32], labels: &[&str]) -> Result<Self> {
        if values.len() != labels.len() {
            bail!("values and labels must have the same length");
        }

        let fields = values.iter().zip(labels.iter()).map(|(value, label)| {
            ChartField {
                value: ChartFieldValue(*value),
                label: label.to_string(),
            }
        }).collect();

        Ok(MaxChart{ fields })
    }

    pub fn get_fields(&self) -> &[ChartField<M>] {
        &self.fields
    }

    pub fn get_max_value(&self) -> i32 {
        M
    }
}