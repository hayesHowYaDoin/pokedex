use color_eyre::{
    eyre::bail,
    Result,
};
use num_traits::bounds::Bounded;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartFieldValue<const M: u32>(u32);

impl<const M: u32> Bounded for ChartFieldValue<M> {
    fn min_value() -> Self { ChartFieldValue(0) }
    fn max_value() -> Self { ChartFieldValue(M) }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartField<const M: u32> {
    value: ChartFieldValue<M>,
    label: String,
}

impl<const M: u32> ChartField<M> {
    pub fn get_value(&self) -> u32 {
        self.value.0
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaxChart<const M: u32> {
    fields: Vec<ChartField<M>>,
}

impl<const M: u32> MaxChart<M> {
    pub fn new(values: &[u32], labels: &[&str]) -> Result<Self> {
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

    pub fn get_max_value(&self) -> u32 {
        M
    }
}