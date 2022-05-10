use std::collections::LinkedList;

pub struct MyGrid {
    pub grid : Vec<Vec<usize>>,
    width : usize,
    height : usize,
    pub cur: (usize, usize),
    pub dst : (usize, usize),
    pub route : LinkedList<(usize, usize)>,
}
impl MyGrid {
    pub fn new(height: usize, width: usize) -> MyGrid {
        let mut column = vec![];
        for i in 1..=height {
            let mut row = vec![0 ; width];
            column.push(row);
        }
        let list: LinkedList<(usize, usize)> = LinkedList::new();
        let mut my_grid = MyGrid {
            grid: column,
            width,
            height,
            cur: (0, 0),
            dst : (height - 1, width - 1),
            route : list,
        };
        my_grid.grid[0][0] = 2;
        my_grid.grid[height - 1][width - 1] = 3;
        return my_grid;
    }
    pub fn clear(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.grid[i][j] = 0;
            }
        }
        self.cur.0 = 0;
        self.cur.1 = 0;
        self.grid[self.cur.0][self.cur.1] = 2;

        self.dst.0 = self.height - 1;
        self.dst.1 = self.width - 1;
        self.grid[self.dst.0][self.dst.1] = 3;

        self.route.clear();
    }
    pub fn print_grid(&self) {
        println!();
        for i in self.grid.iter() {
            for j in i.iter() {
                print!("{} ", j);
            }
            println!();
        }
        println!();
    }
    pub fn set_src(&mut self, new_src_x:usize, new_src_y:usize) {
        if self.grid[new_src_x][new_src_y] == 4 {
            return;
        }
        self.grid[self.cur.0][self.cur.1] = 0;
        self.grid[new_src_x][new_src_y] = 2;
        self.cur.0 = new_src_x;
        self.cur.1 = new_src_y;
    }
    pub fn set_dst(&mut self, new_dst_x:usize, new_dst_y:usize) {
        if self.grid[new_dst_x][new_dst_y] == 3 {
            return;
        }
        self.grid[self.dst.0][self.dst.1] = 0;
        self.grid[new_dst_x][new_dst_y] = 3;
        self.dst.0 = new_dst_x;
        self.dst.1 = new_dst_y;
    }
    pub fn dfs(&mut self) {
        let mut linked_list: LinkedList<(usize, usize)> = LinkedList::new();
        linked_list.push_front(self.cur);

        while !linked_list.is_empty() {
            let pop_point : (usize, usize) = linked_list.pop_front().unwrap();
            println!("{} {}", pop_point.0, pop_point.1);
            self.route.push_front((pop_point.0, pop_point.1));
            if pop_point.0 == self.dst.0 && pop_point.1 == self.dst.1 {
                return;
            }
            self.grid[pop_point.0][pop_point.1] = 4;

            let next_x = pop_point.0 + 1;
            let next_y = pop_point.1;
            if self.is_address_valid(next_x, next_y) {
                linked_list.push_front((next_x, next_y));
            }

            let next_x = pop_point.0;
            let next_y = pop_point.1 + 1;
            if self.is_address_valid(next_x, next_y) {
                linked_list.push_front((next_x, next_y));
            }
            if pop_point.0 >= 1 {
                let next_x = pop_point.0 - 1;
                let next_y = pop_point.1;
                if self.is_address_valid(next_x, next_y) {
                    linked_list.push_front((next_x, next_y));
                }
            }

            if pop_point.1 >= 1 {
                let next_x = pop_point.0;
                let next_y = pop_point.1 - 1;
                if self.is_address_valid(next_x, next_y) {
                    linked_list.push_front((next_x, next_y));
                }
            }
        }
        return;
    }
    pub fn bfs(&mut self) {
        let mut linked_list: LinkedList<(usize, usize)> = LinkedList::new();
        linked_list.push_front(self.cur);

        while !linked_list.is_empty() {
            let pop_point : (usize, usize) = linked_list.pop_back().unwrap();
            println!("{} {}", pop_point.0, pop_point.1);
            self.route.push_front((pop_point.0, pop_point.1));
            if pop_point.0 == self.dst.0 && pop_point.1 == self.dst.1 {
                return;
            }
            self.grid[pop_point.0][pop_point.1] = 4;

            let next_x = pop_point.0 + 1;
            let next_y = pop_point.1;
            if self.is_address_valid(next_x, next_y) {
                linked_list.push_front((next_x, next_y));
            }

            let next_x = pop_point.0;
            let next_y = pop_point.1 + 1;
            if self.is_address_valid(next_x, next_y) {
                linked_list.push_front((next_x, next_y));
            }
            if pop_point.0 >= 1 {
                let next_x = pop_point.0 - 1;
                let next_y = pop_point.1;
                if self.is_address_valid(next_x, next_y) {
                    linked_list.push_front((next_x, next_y));
                }
            }

            if pop_point.1 >= 1 {
                let next_x = pop_point.0;
                let next_y = pop_point.1 - 1;
                if self.is_address_valid(next_x, next_y) {
                    linked_list.push_front((next_x, next_y));
                }
            }
        }
        return;
    }
    pub fn is_address_valid(&self, x : usize, y : usize) -> bool {
        if x >= 0 && x < self.height && y >= 0 && y < self.width {
            if self.grid[x][y] == 3 {
                return true;
            }
            if self.grid[x][y] == 0 {
                return true;
            }
        }
        return false;
    }

    pub fn pop_route(&mut self) -> (usize, usize){
        if !self.route.is_empty() {
            let answer : (usize, usize) = self.route.pop_front().unwrap();
            return answer;
        }
        return (999, 999);
    }
}