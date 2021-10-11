use labeled_variants::LabeledVariants;
use labeled_variants_derive::LabeledVariants;

#[derive(LabeledVariants)]
enum Foo {
    Bar,
    Baz,
    QuxQoox,
}

#[test]
fn tests() {
    assert_eq!(Foo::Bar.variant_label(), "bar");
    assert_eq!(Foo::Baz.variant_label(), "baz");
    assert_eq!(Foo::QuxQoox.variant_label(), "qux_qoox");
}
