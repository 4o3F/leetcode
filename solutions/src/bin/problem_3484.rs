use std::collections::HashMap;

struct Spreadsheet {
    sheet: HashMap<String, i32>,
}

impl Spreadsheet {
    fn new(_: i32) -> Self {
        Self {
            sheet: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.sheet.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.set_cell(cell, 0)
    }

    fn get_value(&self, formula: String) -> i32 {
        let (a, b) = formula[1..].split_once("+").unwrap();
        let x = a.parse::<i32>().unwrap_or(*self.sheet.get(a).unwrap_or(&0));
        let y = b.parse::<i32>().unwrap_or(*self.sheet.get(b).unwrap_or(&0));
        x + y
    }
}

fn main() {
    use utils::prelude::*;
    init_logger();
    let mut spreadsheet = Spreadsheet::new(3);
    tracing::info!("{}", spreadsheet.get_value("=5+7".to_string()));
    spreadsheet.set_cell("A1".to_string(), 10);
    tracing::info!("{}", spreadsheet.get_value("=A1+6".to_string()));
    spreadsheet.set_cell("B2".to_string(), 15);
    tracing::info!("{}", spreadsheet.get_value("=A1+B2".to_string()));
    spreadsheet.reset_cell("A1".to_string());
    tracing::info!("{}", spreadsheet.get_value("=A1+B2".to_string()));
}
