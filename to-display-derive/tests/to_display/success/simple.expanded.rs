struct Foo {}
impl ::to_display::ToDisplay for Foo {
    type Displayer<'a> = &'a Foo where Self: 'a;
    fn display_with_context(
        &self,
        context: ::to_display::Context,
    ) -> Self::Displayer<'_> {
        self
    }
}
