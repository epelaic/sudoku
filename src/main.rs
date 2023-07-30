
use std::{env, fs};

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
    pub x: u8,
    pub y: u8,
    pub value: Option<i64>,
    pub initial: bool
}

impl ValueCell {

    fn init_value(&mut self, value: Option<i64>, initial: bool) {

        self.value = value;
        self.initial = initial;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubGrid {
    pub data: [[ValueCell; 3]; 3]
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
}

impl eframe::App for SudokuGrid {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
                
            ui.heading("Rust Sudoku Resolver");
            let mut x_offset: f32 = 0.0;
            let mut y_offset: f32 = 0.0;
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
            ValueCell{x: 0, y: 0, value: None, initial: false},
            ValueCell{x: 1, y: 0, value: None, initial: false},
            ValueCell{x: 2, y: 0, value: None, initial: false},
            ValueCell{x: 3, y: 0, value: None, initial: false},
            ValueCell{x: 4, y: 0, value: None, initial: false},
            ValueCell{x: 5, y: 0, value: None, initial: false},
            ValueCell{x: 6, y: 0, value: None, initial: false},
            ValueCell{x: 7, y: 0, value: None, initial: false},
            ValueCell{x: 8, y: 0, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 1, value: None, initial: false},
            ValueCell{x: 1, y: 1, value: None, initial: false},
            ValueCell{x: 2, y: 1, value: None, initial: false},
            ValueCell{x: 3, y: 1, value: None, initial: false},
            ValueCell{x: 4, y: 1, value: None, initial: false},
            ValueCell{x: 5, y: 1, value: None, initial: false},
            ValueCell{x: 6, y: 1, value: None, initial: false},
            ValueCell{x: 7, y: 1, value: None, initial: false},
            ValueCell{x: 8, y: 1, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 2, value: None, initial: false},
            ValueCell{x: 1, y: 2, value: None, initial: false},
            ValueCell{x: 2, y: 2, value: None, initial: false},
            ValueCell{x: 3, y: 2, value: None, initial: false},
            ValueCell{x: 4, y: 2, value: None, initial: false},
            ValueCell{x: 5, y: 2, value: None, initial: false},
            ValueCell{x: 6, y: 2, value: None, initial: false},
            ValueCell{x: 7, y: 2, value: None, initial: false},
            ValueCell{x: 8, y: 2, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 3, value: None, initial: false},
            ValueCell{x: 1, y: 3, value: None, initial: false},
            ValueCell{x: 2, y: 3, value: None, initial: false},
            ValueCell{x: 3, y: 3, value: None, initial: false},
            ValueCell{x: 4, y: 3, value: None, initial: false},
            ValueCell{x: 5, y: 3, value: None, initial: false},
            ValueCell{x: 6, y: 3, value: None, initial: false},
            ValueCell{x: 7, y: 3, value: None, initial: false},
            ValueCell{x: 8, y: 3, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 4, value: None, initial: false},
            ValueCell{x: 1, y: 4, value: None, initial: false},
            ValueCell{x: 2, y: 4, value: None, initial: false},
            ValueCell{x: 3, y: 4, value: None, initial: false},
            ValueCell{x: 4, y: 4, value: None, initial: false},
            ValueCell{x: 5, y: 4, value: None, initial: false},
            ValueCell{x: 6, y: 4, value: None, initial: false},
            ValueCell{x: 7, y: 4, value: None, initial: false},
            ValueCell{x: 8, y: 4, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 5, value: None, initial: false},
            ValueCell{x: 1, y: 5, value: None, initial: false},
            ValueCell{x: 2, y: 5, value: None, initial: false},
            ValueCell{x: 3, y: 5, value: None, initial: false},
            ValueCell{x: 4, y: 5, value: None, initial: false},
            ValueCell{x: 5, y: 5, value: None, initial: false},
            ValueCell{x: 6, y: 5, value: None, initial: false},
            ValueCell{x: 7, y: 5, value: None, initial: false},
            ValueCell{x: 8, y: 5, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 6, value: None, initial: false},
            ValueCell{x: 1, y: 6, value: None, initial: false},
            ValueCell{x: 2, y: 6, value: None, initial: false},
            ValueCell{x: 3, y: 6, value: None, initial: false},
            ValueCell{x: 4, y: 6, value: None, initial: false},
            ValueCell{x: 5, y: 6, value: None, initial: false},
            ValueCell{x: 6, y: 6, value: None, initial: false},
            ValueCell{x: 7, y: 6, value: None, initial: false},
            ValueCell{x: 8, y: 6, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 7, value: None, initial: false},
            ValueCell{x: 1, y: 7, value: None, initial: false},
            ValueCell{x: 2, y: 7, value: None, initial: false},
            ValueCell{x: 3, y: 7, value: None, initial: false},
            ValueCell{x: 4, y: 7, value: None, initial: false},
            ValueCell{x: 5, y: 7, value: None, initial: false},
            ValueCell{x: 6, y: 7, value: None, initial: false},
            ValueCell{x: 7, y: 7, value: None, initial: false},
            ValueCell{x: 8, y: 8, value: None, initial: false}
        ],
        [
            ValueCell{x: 0, y: 8, value: None, initial: false},
            ValueCell{x: 1, y: 8, value: None, initial: false},
            ValueCell{x: 2, y: 8, value: None, initial: false},
            ValueCell{x: 3, y: 8, value: None, initial: false},
            ValueCell{x: 4, y: 8, value: None, initial: false},
            ValueCell{x: 5, y: 8, value: None, initial: false},
            ValueCell{x: 6, y: 8, value: None, initial: false},
            ValueCell{x: 7, y: 8, value: None, initial: false},
            ValueCell{x: 8, y: 8, value: None, initial: false}
        ]
    ];

    grid
}