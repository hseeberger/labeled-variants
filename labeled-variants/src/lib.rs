pub trait LabeledVariants {
    fn variant_label(&self) -> &'static str;
}
