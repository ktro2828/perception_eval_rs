use crate::label::Label;

#[derive(Debug, Clone)]
pub struct LabelThreshold<'a> {
    label: &'a Label,
    target_labels: &'a Vec<Label>,
}

impl<'a> LabelThreshold<'a> {
    pub fn new(label: &'a Label, target_labels: &'a Vec<Label>) -> Self {
        Self {
            label: label,
            target_labels: target_labels,
        }
    }

    pub fn get_threshold<T>(&self, thresholds: &Vec<T>) -> Option<T>
    where
        T: Copy,
    {
        get_label_threshold(self.label, self.target_labels, thresholds)
    }
}

pub fn get_label_threshold<T>(
    label: &Label,
    target_labels: &Vec<Label>,
    thresholds: &Vec<T>,
) -> Option<T>
where
    T: Copy,
{
    if target_labels.contains(label) {
        let index = target_labels.iter().position(|v| v == label).unwrap();
        let value = &thresholds[index];
        Some(*value)
    } else {
        None
    }
}
