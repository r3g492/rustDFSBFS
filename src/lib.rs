pub struct Hello {
    pub var1 : usize,
    pub var2 : usize,
}

pub struct MyGrid {
    pub grid : Vec<Vec<usize>>,
    width : usize,
    height : usize,
    pub src : (usize, usize),
    pub dst : (usize, usize)
}
impl MyGrid {
    pub fn new(width: usize, height: usize) -> MyGrid {
        let mut column = vec![];
        for i in 1..=height {
            let mut row = vec![0 ; width];
            column.push(row);
        }
        let mut my_grid = MyGrid {
            grid: column,
            width,
            height,
            src : (0 ,0),
            dst : (height, width),
        };
        my_grid.grid[0][0] = 2;
        my_grid.grid[height - 1][width - 1] = 3;
        return my_grid;
    }
    pub fn print_grid(&self) {
        for i in self.grid.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!();
        }
    }

    pub fn bfs(&self) {

    }
}