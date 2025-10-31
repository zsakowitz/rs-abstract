use std::fmt::{self, Display};

pub trait ToLatex {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    fn print_latex(&self)
    where
        Self: Sized,
    {
        struct DisplayAsLatex<'a, T>(&'a T);

        impl<'a, T: ToLatex> Display for DisplayAsLatex<'a, T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        println!("{}", DisplayAsLatex(self));
    }
}
