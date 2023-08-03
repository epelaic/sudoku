
use std::{env, fs, ops::Range};

use egui::{Vec2, Rect, Pos2, Rounding, Color32, Stroke, epaint::{RectShape, TextShape}, FontId, text::LayoutJob, Align, FontFamily};
use yaml_rust::{Yaml, YamlLoader};

fn main() {
    println!("Rust Sudoku Resolver !");

    let args: Vec<String> = env::args().collect();

    let grid_file_path: &String = &args[1];

    let grid_file: String = fs::read_to_string(grid_file_path).unwrap();
    let grid_file_str: &str = grid_file.as_str();
    
    let grid: Vec<Yaml> = YamlLoader::load_from_str(grid_file_str).unwrap();
    let grid_data: &Yaml = &grid[0]["grid"];

    //println!("grid_data : {:?}", grid_data);

    let mut sudoku_grid: SudokuGrid = SudokuGrid::new();

    for grid_value in grid_data.as_vec().unwrap() {

        let cell_value = grid_value["value"].as_i64().unwrap();
        let cell_i = grid_value["i"].as_i64().unwrap() as usize;
        let cell_j = grid_value["j"].as_i64().unwrap() as usize;
        //println!("value: {}, i: {}, j: {}", cell_value, cell_i, cell_j);

        sudoku_grid.init_cell_value(cell_value, cell_i, cell_j);
    }

    //sudoku_grid.solve();
  

    // Init Gui APP
    let options = &mut eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2{x: 800.0, y: 700.0});

    let _ = eframe::run_native(
        "Sudoku",
        options.to_owned(),
        Box::new(|_cc| Box::new(sudoku_grid)),
    );

    
}
#[derive(Debug, Clone, Copy)]
pub struct ValueCell {
    pub i: u8,
    pub j: u8,
    pub value: Option<i64>,
    pub initial: bool
}

impl ValueCell {

    fn init_value(&mut self, value: Option<i64>, initial: bool) {

        self.value = value;
        self.initial = initial;
    }
}

#[derive(Debug, Clone)]
pub struct SubGrid {
    pub range_i: Range<u8>,
    pub range_j: Range<u8>,
    pub data: Vec<i64>
}

impl SubGrid {

    pub fn get_missing_values(&self) -> Vec<i64> {

        // Possible values
        let possible_values: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let results: Vec<i64> = possible_values.iter()
                    .filter(|&v| !self.data.contains(&v))
                    .cloned()
                    .collect();
        
        results
    }
}

pub struct ValueCellWrapper {
    pub targer: ValueCell,
    pub constraints: i64
}

pub struct SudokuGrid {
    pub grid: [[ValueCell; 9];9]
}

impl SudokuGrid {

    const VALUE_SIZE: f32 = 60.0;

    fn new() -> Self {

        SudokuGrid {grid: init_grid()}
    }

    pub fn init_cell_value(&mut self, value: i64, i: usize, j: usize) {
        //println!("set value: {}, at i: {}, j: {}", value, i, j);
        let value_cell: &mut ValueCell = &mut self.grid[i][j];
        value_cell.init_value(Some(value), true);
    }

    pub fn solve(&mut self) {

        let value_cells_wrapper: Vec<ValueCellWrapper> = self.get_most_constraints_value_cells();

        'outer:
        for value_cell_wrapper in value_cells_wrapper.iter() {

                let grid: [[ValueCell; 9];9] = self.grid.clone();
                let i = value_cell_wrapper.targer.i;
                let j = value_cell_wrapper.targer.j;
                
                let row_cell_mut: &mut ValueCell = &mut self.grid[i as usize][j as usize];

                if row_cell_mut.value.is_none() {

                    println!("row_value_cell i: {}, j: {}", row_cell_mut.i, row_cell_mut.j);
                    // Get sub grid values
                    let sub_grid: SubGrid = SudokuGrid::get_sub_grid_for_value_cell(grid, &row_cell_mut);

                    println!("sub_grid values : {:?}", sub_grid.data.clone());

                    // Get cols values
                    let col_values: Vec<i64> = SudokuGrid::get_values_of_col(grid, j as usize);
                    
                    println!("col_values : {:?}", col_values.clone());
                    

                    // Get rows values
                    let row_values: Vec<i64> = SudokuGrid::get_values_of_row(grid, i as usize);
                    
                    println!("row_values : {:?}", row_values.clone());

                    // Check missing values restricted to subgrid values
                    let sub_grid_missing_values = sub_grid.get_missing_values();
                    
                    println!("sub_grid_missing_values : {:?}", sub_grid_missing_values.clone());

                    // Filter sub grid missing values by col values
                    let mut possible_values: Vec<i64> = sub_grid_missing_values.iter()
                                                        .filter(|&v| !col_values.contains(&v))
                                                        .cloned()
                                                        .collect();
                    
                    // Filter sub grid missing values by row values
                    possible_values = possible_values.iter()
                                                        .filter(|&v| !row_values.contains(&v))
                                                        .cloned()
                                                        .collect();
                    
                    println!("possibles_values : {:?}", possible_values.clone());

                    // Check num possibilities
                    if possible_values.len() == 1 && row_cell_mut.initial == false {
                        let val = possible_values[0];
                        
                        println!("set value : {}", val);
                        
                        row_cell_mut.value = Some(val);
                        break 'outer;
                    }
            }
        }

        // Get mos constraints cols

    }

    fn get_most_constraints_value_cells(&self) -> Vec<ValueCellWrapper>{

        let mut results: Vec<ValueCellWrapper> = Vec::new();

        for i in self.grid.iter() {

            for j in i.iter() {
                
                let value_cell = j.clone();

                if value_cell.value.is_none() {

                    let col_const: Vec<i64> = SudokuGrid::get_values_of_col(self.grid, value_cell.j as usize);
                    let row_const: Vec<i64> = SudokuGrid::get_values_of_row(self.grid, value_cell.i as usize);

                    results.push(ValueCellWrapper { targer: value_cell, constraints: (col_const.len() + row_const.len()) as i64 });
                }
            }
        }

        results.sort_by(|a,b| b.constraints.cmp(&a.constraints));

        results
    }

    fn get_values_of_col(grid: [[ValueCell; 9];9], col_index: usize) -> Vec<i64> {

        let result: &mut Vec<i64> = &mut Vec::new();

        for i in 0..grid.len() {

            let value_cell: &ValueCell = &grid[i][col_index];

            if value_cell.value.is_some() {
                let col_value: i64 = grid[i][col_index].value.unwrap();
                result.push(col_value);
            }
        }

        result.to_vec()
    }

    fn get_values_of_row(grid: [[ValueCell; 9];9], row_index: usize) -> Vec<i64> {

        let result: &mut Vec<i64> = &mut Vec::new();

        for j in 0..grid.len() {

            let value_cell: &ValueCell = &grid[row_index][j];

            if value_cell.value.is_some() {
                let row_value: i64 = grid[row_index][j].value.unwrap();
                result.push(row_value);
            }
        }

        result.to_vec()
    }

    fn get_sub_grid_for_value_cell(grid: [[ValueCell; 9];9], value_cell: &ValueCell) -> SubGrid {

        let i: u8 = value_cell.i;
        let j: u8 = value_cell.j;

        let resolved_range_i: Range<u8> = SudokuGrid::resolve_sub_grid(i);
        let resolved_range_j: Range<u8> = SudokuGrid::resolve_sub_grid(j);
        
        let mut data:  Vec<i64> = Vec::new();

        println!("sub_grid range i1: {}, i2: {}, j1: {}, j2: {}", 
            resolved_range_i.start, resolved_range_i.end,
           resolved_range_j.start, resolved_range_j.end);

        let mut i: usize = resolved_range_i.start.into();
        let end_i: usize = resolved_range_i.end.into();

        'outer: loop {

            let mut j: usize = resolved_range_j.start.into();
            let end_j: usize = resolved_range_j.end.into() ;

            'inner: loop {
                
                //println!("loop i: {}, j: {}", i, j);

                let current_sub_grid_value_cell: &ValueCell = &grid[i][j];

                if current_sub_grid_value_cell.value.is_some() {
                    let cell_value = current_sub_grid_value_cell.value.unwrap();
                    
                    println!("cell_value: {}", cell_value);
                    
                    data.push(cell_value);
                }

                j += 1;

                if j >= end_j + 1 {
                    break 'inner;
                }
            }

            i += 1;

            if i >= end_i + 1 {
                break 'outer;
            }
        }

        SubGrid {range_i: resolved_range_i, range_j: resolved_range_j, data: data }
    }

    fn resolve_sub_grid(index: u8) -> Range<u8> {

        let first: Range<u8> = 0..2;
        let second: Range<u8> = 3..5;
        let third: Range<u8> = 6..8;
        
        let mut resolved_range: Range<u8> = first.clone();

        if index >= 0 && index <= 2 {
            resolved_range = first;
        } else if index >= 3 && index <= 5 {
            resolved_range = second.clone();
        } else if index >= 6 && index <= 8 {
            resolved_range = third.clone();
        }

        resolved_range
    }

}

impl eframe::App for SudokuGrid {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
                
            ui.heading("Rust Sudoku Resolver");
            
            if ui.add(egui::Button::new("Next iteration")).clicked() {
                self.solve();
            }

            let mut x_offset: f32 = 00.0;
            let mut y_offset: f32 = 50.0;
            let border: [usize; 2] = [3,6];

            for i in 0..self.grid.len() {

                let row = self.grid[i];

                for j in 0..row.len() {

                    let value_cell = row[j];
                    //println!("value_cell : {:?}", value_cell);

                    let mut min_x: f32 = x_offset;
                    let mut min_y: f32 = y_offset;

                    if border.contains(&i) {
                        min_y += 1.0;
                    }

                    if border.contains(&j) {
                        min_x += 1.0;
                    }

                    let rect: Rect = Rect{
                        min: Pos2{x: min_x, y: min_y},
                        max: Pos2{x: min_x + SudokuGrid::VALUE_SIZE, y: min_y + SudokuGrid::VALUE_SIZE}
                    };

                    let mut bg_ground_color: Color32 = Color32::WHITE;

                    if value_cell.initial == true {
                        bg_ground_color = Color32::GRAY;
                    }

                    let box_rect: RectShape = RectShape { 
                        rect: rect, 
                        rounding: Rounding::none(), 
                        fill: bg_ground_color, 
                        stroke: Stroke { width: 2.0, color: Color32::BLACK } 
                    };

                    ui.painter().add(box_rect);

                    x_offset += SudokuGrid::VALUE_SIZE;

                    //Draw text value
                    if value_cell.value.is_some() {

                        let value_str = value_cell.value.unwrap().to_owned().to_string();
                        //println!("Current value : {}", value_str);
                        let font_id: FontId = FontId::new(30.0, FontFamily::Monospace);
                        let mut layout_job: LayoutJob = LayoutJob::simple_singleline(value_str, font_id, Color32::BLACK);
                        layout_job.halign = Align::Center;

                        let galley = ctx.fonts(|f| {

                            f.layout_job(layout_job)
                        });

                        let pos = Pos2 { x: min_x + 30.0, y: min_y + 15.0 };

                        let text_shape: TextShape = TextShape { pos, galley, underline: Stroke::NONE, override_text_color: None, angle: 0.0 };
        
                        ui.painter().add(text_shape);
                    }

                }

                x_offset = 0.0;
                y_offset += SudokuGrid::VALUE_SIZE;
            }
            
        });
    
    }

}

fn init_grid() -> [[ValueCell; 9]; 9] {
    
    let grid: [[ValueCell; 9];9] = [
        [
            ValueCell{i: 0, j: 0, value: None, initial: false},
            ValueCell{i: 0, j: 1, value: None, initial: false},
            ValueCell{i: 0, j: 2, value: None, initial: false},
            ValueCell{i: 0, j: 3, value: None, initial: false},
            ValueCell{i: 0, j: 4, value: None, initial: false},
            ValueCell{i: 0, j: 5, value: None, initial: false},
            ValueCell{i: 0, j: 6, value: None, initial: false},
            ValueCell{i: 0, j: 7, value: None, initial: false},
            ValueCell{i: 0, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 1, j: 0, value: None, initial: false},
            ValueCell{i: 1, j: 1, value: None, initial: false},
            ValueCell{i: 1, j: 2, value: None, initial: false},
            ValueCell{i: 1, j: 3, value: None, initial: false},
            ValueCell{i: 1, j: 4, value: None, initial: false},
            ValueCell{i: 1, j: 5, value: None, initial: false},
            ValueCell{i: 1, j: 6, value: None, initial: false},
            ValueCell{i: 1, j: 7, value: None, initial: false},
            ValueCell{i: 1, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 2, j: 0, value: None, initial: false},
            ValueCell{i: 2, j: 1, value: None, initial: false},
            ValueCell{i: 2, j: 2, value: None, initial: false},
            ValueCell{i: 2, j: 3, value: None, initial: false},
            ValueCell{i: 2, j: 4, value: None, initial: false},
            ValueCell{i: 2, j: 5, value: None, initial: false},
            ValueCell{i: 2, j: 6, value: None, initial: false},
            ValueCell{i: 2, j: 7, value: None, initial: false},
            ValueCell{i: 2, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 3, j: 0, value: None, initial: false},
            ValueCell{i: 3, j: 1, value: None, initial: false},
            ValueCell{i: 3, j: 2, value: None, initial: false},
            ValueCell{i: 3, j: 3, value: None, initial: false},
            ValueCell{i: 3, j: 4, value: None, initial: false},
            ValueCell{i: 3, j: 5, value: None, initial: false},
            ValueCell{i: 3, j: 6, value: None, initial: false},
            ValueCell{i: 3, j: 7, value: None, initial: false},
            ValueCell{i: 3, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 4, j: 0, value: None, initial: false},
            ValueCell{i: 4, j: 1, value: None, initial: false},
            ValueCell{i: 4, j: 2, value: None, initial: false},
            ValueCell{i: 4, j: 3, value: None, initial: false},
            ValueCell{i: 4, j: 4, value: None, initial: false},
            ValueCell{i: 4, j: 5, value: None, initial: false},
            ValueCell{i: 4, j: 6, value: None, initial: false},
            ValueCell{i: 4, j: 7, value: None, initial: false},
            ValueCell{i: 4, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 5, j: 0, value: None, initial: false},
            ValueCell{i: 5, j: 1, value: None, initial: false},
            ValueCell{i: 5, j: 2, value: None, initial: false},
            ValueCell{i: 5, j: 3, value: None, initial: false},
            ValueCell{i: 5, j: 4, value: None, initial: false},
            ValueCell{i: 5, j: 5, value: None, initial: false},
            ValueCell{i: 5, j: 6, value: None, initial: false},
            ValueCell{i: 5, j: 7, value: None, initial: false},
            ValueCell{i: 5, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 6, j: 0, value: None, initial: false},
            ValueCell{i: 6, j: 1, value: None, initial: false},
            ValueCell{i: 6, j: 2, value: None, initial: false},
            ValueCell{i: 6, j: 3, value: None, initial: false},
            ValueCell{i: 6, j: 4, value: None, initial: false},
            ValueCell{i: 6, j: 5, value: None, initial: false},
            ValueCell{i: 6, j: 6, value: None, initial: false},
            ValueCell{i: 6, j: 7, value: None, initial: false},
            ValueCell{i: 6, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 7, j: 0, value: None, initial: false},
            ValueCell{i: 7, j: 1, value: None, initial: false},
            ValueCell{i: 7, j: 2, value: None, initial: false},
            ValueCell{i: 7, j: 3, value: None, initial: false},
            ValueCell{i: 7, j: 4, value: None, initial: false},
            ValueCell{i: 7, j: 5, value: None, initial: false},
            ValueCell{i: 7, j: 6, value: None, initial: false},
            ValueCell{i: 7, j: 7, value: None, initial: false},
            ValueCell{i: 7, j: 8, value: None, initial: false}
        ],
        [
            ValueCell{i: 8, j: 0, value: None, initial: false},
            ValueCell{i: 8, j: 1, value: None, initial: false},
            ValueCell{i: 8, j: 2, value: None, initial: false},
            ValueCell{i: 8, j: 3, value: None, initial: false},
            ValueCell{i: 8, j: 4, value: None, initial: false},
            ValueCell{i: 8, j: 5, value: None, initial: false},
            ValueCell{i: 8, j: 6, value: None, initial: false},
            ValueCell{i: 8, j: 7, value: None, initial: false},
            ValueCell{i: 8, j: 8, value: None, initial: false}
        ]
    ];

    grid
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_resolve_sub_grid_0() {
        assert_eq!(SudokuGrid::resolve_sub_grid(0), 0..2);
    }

    #[test]
    fn test_resolve_sub_grid_1() {
        assert_eq!(SudokuGrid::resolve_sub_grid(1), 0..2);
    }

    #[test]
    fn test_resolve_sub_grid_2() {
        assert_eq!(SudokuGrid::resolve_sub_grid(2), 0..2);
    }

    #[test]
    fn test_resolve_sub_grid_3() {
        assert_eq!(SudokuGrid::resolve_sub_grid(3), 3..5);
    }

    #[test]
    fn test_resolve_sub_grid_4() {
        assert_eq!(SudokuGrid::resolve_sub_grid(4), 3..5);
    }

    #[test]
    fn test_resolve_sub_grid_5() {
        assert_eq!(SudokuGrid::resolve_sub_grid(5), 3..5);
    }

    #[test]
    fn test_resolve_sub_grid_6() {
        assert_eq!(SudokuGrid::resolve_sub_grid(6), 6..8);
    }

    #[test]
    fn test_resolve_sub_grid_7() {
        assert_eq!(SudokuGrid::resolve_sub_grid(7), 6..8);
    }

    #[test]
    fn test_resolve_sub_grid_8() {
        assert_eq!(SudokuGrid::resolve_sub_grid(8), 6..8);
    }

}