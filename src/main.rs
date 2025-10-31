use crate::{dihedral::D, latex::ToLatex, table::Table};

pub mod coset;
pub mod dihedral;
pub mod group;
pub mod inv;
pub mod latex;
pub mod supsub;
pub mod table;

fn main() {
    let a = [
        D::<4>::rot(0),
        D::rot(2),
        D::flip(2),
        D::flip(4),
        D::rot(1),
        D::rot(3),
        D::flip(3),
        D::flip(1),
    ];
    let mut table = Table::by_product(a, a);
    table.add_row_divider_after(4);
    table.add_col_divider_after(4);
    table.print_latex();
}
