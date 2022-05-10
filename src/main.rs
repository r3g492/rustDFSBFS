use crate::lib::{Hello, MyGrid};

mod lib;

fn main() {
    let my_grid = MyGrid::new(16, 8);
    MyGrid::print_grid(&my_grid);
}
