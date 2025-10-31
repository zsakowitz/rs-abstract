use std::fmt::{Display, Write};

pub struct SubS<T>(pub T);
pub struct SupS<T>(pub T);

impl<T: Display> Display for SupS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in format!("{}", self.0).chars() {
            let c = match char {
                '0' => '⁰',
                '1' => '¹',
                '2' => '²',
                '3' => '³',
                '4' => '⁴',
                '5' => '⁵',
                '6' => '⁶',
                '7' => '⁷',
                '8' => '⁸',
                '9' => '⁹',
                x => x,
            };
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl<T: Display> Display for SubS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for char in format!("{}", self.0).chars() {
            let c = match char {
                '0' => '₀',
                '1' => '₁',
                '2' => '₂',
                '3' => '₃',
                '4' => '₄',
                '5' => '₅',
                '6' => '₆',
                '7' => '₇',
                '8' => '₈',
                '9' => '₉',
                x => x,
            };
            f.write_char(c)?;
        }
        Ok(())
    }
}
