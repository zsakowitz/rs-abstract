use std::{fmt, ops::Mul};

use crate::latex::ToLatex;

#[derive(Debug)]
pub struct Table<T> {
    cols: usize,
    rows: usize,
    data: Vec<T>,
    row_dividers: Vec<usize>, // a divider will be inserted after the given row index
    col_dividers: Vec<usize>, // a divider will be inserted after the given column index
    label: &'static str,
}

impl<T> Table<T> {
    pub fn add_row_divider_after(&mut self, i: usize) {
        self.row_dividers.push(i);
    }

    pub fn add_col_divider_after(&mut self, i: usize) {
        self.col_dividers.push(i);
    }
}

impl<T: Clone + Default + Mul<Output = T>> Table<T> {
    pub fn by_product(
        rows: impl IntoIterator<Item = T>,
        cols: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut rows: Vec<_> = rows.into_iter().collect();
        rows.insert(0, Default::default());
        let mut cols: Vec<_> = cols.into_iter().collect();
        cols.insert(0, Default::default());

        Table {
            cols: cols.len(),
            rows: rows.len(),
            data: rows
                .into_iter()
                .flat_map(|row| cols.iter().map(move |col| row.clone() * col.clone()))
                .collect(),
            row_dividers: vec![0],
            col_dividers: vec![0],
            label: "\\text{left}*\\text{top}",
        }
    }
}

impl<T: ToLatex> ToLatex for Table<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "$$\\begin{{array}}{{")?;
        for i in 0..self.cols {
            write!(fmt, "c")?;
            if self.col_dividers.contains(&i) {
                write!(fmt, "|")?;
            }
        }
        write!(fmt, "}}")?;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if i == 0 && j == 0 {
                    write!(fmt, "{}", self.label)?;
                } else {
                    if j != 0 {
                        write!(fmt, "&")?;
                    }
                    let cell_data = &self.data[i * self.cols + j];
                    cell_data.fmt(fmt)?;
                }
            }
            write!(fmt, "\\\\")?;
            if self.row_dividers.contains(&i) {
                write!(fmt, "\\hline ")?;
            }
        }

        write!(fmt, "\\end{{array}}$$")?;
        Ok(())
    }
}
