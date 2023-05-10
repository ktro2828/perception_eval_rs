use crate::label::Label;

/// A struct to extract corresponding threshold value from list of thresholds.
///
/// * `label`           - Target label.
/// * `target_labels`   - List of labels.
#[derive(Debug, Clone)]
pub struct LabelThreshold<'a> {
    label: &'a Label,
    target_labels: &'a Vec<Label>,
}

impl<'a> LabelThreshold<'a> {
    /// Generate instance of `LabelThreshold`
    ///
    /// * `label`           - Target label
    /// * `target_labels`   - List of labels.
    ///
    /// # Example
    /// ```
    /// use perception_eval::{label::Label, threshold::LabelThreshold};
    ///
    /// let label = Label::Car;
    /// let target_labels = vec![Label::Car, Label::Bus, Label::Pedestrian];
    ///
    /// let label_threshold = LabelThreshold::new(&label, &target_labels);
    /// ```
    pub fn new(label: &'a Label, target_labels: &'a Vec<Label>) -> Self {
        Self {
            label: label,
            target_labels: target_labels,
        }
    }

    /// Returns corresponding threshold from list of thresholds.
    /// The index is same with target label's one.
    ///
    /// * `thresholds`      - List of thresholds.
    ///
    /// # Example
    /// ```
    /// use perception_eval::{label::Label, threshold::LabelThreshold};
    ///
    /// let label = Label::Car;
    /// let target_labels = vec![Label::Car, Label::Bus, Label::Pedestrian];
    ///
    /// let label_threshold = LabelThreshold::new(&label, &target_labels);
    ///
    /// let thresholds = vec![1.0, 2.0, 3.0];
    ///
    /// let threshold = label_threshold.get_threshold(&thresholds).unwrap();
    /// assert_eq!(threshold, 1.0);
    /// ```
    pub fn get_threshold<T>(&self, thresholds: &Vec<T>) -> Option<T>
    where
        T: Copy,
    {
        get_label_threshold(self.label, self.target_labels, thresholds)
    }
}

/// Returns corresponding threshold from list of thresholds.
/// The index is same with target label's one.
///
/// * `label`           - Target label.
/// * `target_labels`   - List of labels.
/// * `thresholds`      - List of thresholds.
///
/// # Example
/// ```
/// use perception_eval::{label::Label, threshold::get_label_threshold};
///
/// let label = Label::Car;
/// let target_labels = vec![Label::Car, Label::Bus, Label::Pedestrian];
/// let thresholds = vec![1.0, 2.0, 3.0];
///
/// let threshold = get_label_threshold(&label, &target_labels, &thresholds).unwrap();
/// assert_eq!(threshold, 1.0);
/// ```
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
