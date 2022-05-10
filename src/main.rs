use crate::lib::{MyGrid};

mod lib;

fn main() {
    let mut my_grid = MyGrid::new(4, 8);
    MyGrid::print_grid(&my_grid);

    MyGrid::set_src(&mut my_grid, 3, 3);
    MyGrid::set_dst(&mut my_grid, 3, 2);

    MyGrid::print_grid(&my_grid);

    MyGrid::dfs_next(&mut my_grid);
    //
    MyGrid::print_grid(&my_grid);
}
