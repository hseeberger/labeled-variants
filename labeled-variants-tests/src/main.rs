use labeled_variants::LabeledVariants;
use labeled_variants_derive::LabeledVariants;

#[derive(LabeledVariants)]
enum Foo {
    Bar,
    Baz,
    QuxQoox,
}

fn main() {
    println!("Foo::Bar = {}", Foo::Bar.variant_label());
    println!("Foo::Baz = {}", Foo::Baz.variant_label());
    println!("Foo::QuxQoox = {}", Foo::QuxQoox.variant_label());
}
