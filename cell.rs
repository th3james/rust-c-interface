pub struct Cell {
    value: i32
}

#[no_mangle]
pub extern "C" fn get_cell_value() -> i32 {
    return 2
}

fn main() {
    let cell = Cell{
        //value: "Hello World!".to_string()
        value: 31
    };
    println!("cell value: {}", cell.value);
    println!("get_cell_value {}", get_cell_value());
}
