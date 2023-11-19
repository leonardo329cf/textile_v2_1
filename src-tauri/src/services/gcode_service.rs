use chrono::Local;

use crate::{services::file_service::{get_file_text, write_to_new_file}, models::cutting_lines::Line};

use super::file_service::FileError;


pub async fn generate_gcode_file(
    horizontal_lines: Vec<Line>, 
    vertical_lines: Vec<Line>, 
    textile_length_to_pull: Option<u32>,
    output_folder_path: &str,
    name: &str
) -> Result<String, FileError> {

    const CNC_INSTRUCTION_FOLDER_PATH: &str = "configs\\cnc_instructions";

    let start_program_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "start_program.txt";
    let end_program_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" +  "end_program.txt";
    let pick_textile_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "pick_textile.txt";
    let drop_textile_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "drop_textile.txt";
    let before_x_cut_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "before_x_cut.txt";
    let after_x_cut_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "after_x_cut.txt";
    let before_y_cut_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "before_y_cut.txt";
    let after_y_cut_file_path: String = CNC_INSTRUCTION_FOLDER_PATH.to_string() + "\\" + "after_y_cut.txt";

    let mut instructions = String::new();

    instructions.push_str(&get_title_comment(name));


    instructions.push_str(&get_start_program(&start_program_file_path).await?);
    instructions.push('\n');
    instructions.push('\n');


    if let Some(length_to_pull) = textile_length_to_pull {

        instructions.push_str(&get_pull_textile_instruction(length_to_pull, &pick_textile_file_path, &drop_textile_file_path).await?);
        instructions.push('\n');
        instructions.push('\n');
    }

    instructions.push_str(&get_vertical_lines(vertical_lines, &before_y_cut_file_path, &after_y_cut_file_path).await?);

    instructions.push('\n');
    instructions.push('\n');

    instructions.push_str(&get_horizontal_lines(horizontal_lines, &before_x_cut_file_path, &after_x_cut_file_path).await?);

    instructions.push('\n');
    instructions.push('\n');

    instructions.push_str(&get_end_program(&end_program_file_path).await?);

    let path = write_to_new_file(&format!("{}\\{}.txt", output_folder_path, name), &instructions).await?;

    Ok(format!("Arquivo criado: {}", path))
}

fn get_title_comment(name: &str) -> String {
    let mut instructions = String::new(); 

    instructions.push_str(&format!("( {} - data: {} )", name, &Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string()));
    instructions.push('\n');
    instructions.push('\n');

    instructions
}

async fn get_pull_textile_instruction(length_to_pull: u32, pick_textile_file_path: &str, drop_textile_file_path: &str) -> Result<String, FileError> {
    
    let pick_textile_instruction = get_file_text(pick_textile_file_path).await?;

    let drop_textile_instruction = get_file_text(drop_textile_file_path).await?;

    let mut instructions = String::new(); 

    instructions.push_str("( Inicio posicionar tecido )");
    instructions.push('\n');

    instructions.push_str(&pick_textile_instruction);
    instructions.push('\n');

    instructions.push_str(&format!("G1 Y{}", length_to_pull));
    instructions.push('\n');

    instructions.push_str(&drop_textile_instruction);
    instructions.push('\n');

    instructions.push_str("( Fim posicionar tecido )");
    instructions.push('\n');
    instructions.push('\n');

    Ok(instructions)
}

async fn get_start_program(start_program_file_path: &str) -> Result<String, FileError> {
    let start_program_instruction = get_file_text(start_program_file_path).await?;

    let mut instructions = String::new(); 

    instructions.push_str("( Inicio configuracoes iniciais )");
    instructions.push('\n');

    instructions.push_str(&start_program_instruction);
    instructions.push('\n');
    
    instructions.push_str("( Fim configuracoes iniciais )");
    instructions.push('\n');
    instructions.push('\n');

    Ok(instructions)
}

async fn get_horizontal_lines(horizontal_lines: Vec<Line>, before_x_cut_file_path: &str, after_x_cut_file_path: &str) -> Result<String, FileError> {
    let before_x_cut_instruction = get_file_text(before_x_cut_file_path).await?;
    let after_x_cut_instruction = get_file_text(after_x_cut_file_path).await?;

    let mut instructions = String::new();

    instructions.push_str("( Inicio cortar todas linhas horizontais )");
    instructions.push('\n');

    for line in horizontal_lines {

        instructions.push_str("( Inicio cortar linha horizontal )");
        instructions.push('\n');

        instructions.push_str(&get_move_fast_speed(line.start.pos_x, line.start.pos_y));
        instructions.push('\n');

        instructions.push_str(&before_x_cut_instruction);
        instructions.push('\n');

        instructions.push_str(&get_move_slow_speed(line.end.pos_x, line.end.pos_y));
        instructions.push('\n');

        instructions.push_str(&after_x_cut_instruction);
        instructions.push('\n');

        instructions.push_str("( Fim cortar linha horizontal )");
        instructions.push('\n');
    }

    instructions.push_str("( Fim cortar todas linhas horizontais )");
    instructions.push('\n');
    instructions.push('\n');

    Ok(instructions)
}

async fn get_vertical_lines(vertical_lines: Vec<Line>, before_y_cut_file_path: &str, after_y_cut_file_path: &str) -> Result<String, FileError> {
    let before_y_cut_instruction = get_file_text(before_y_cut_file_path).await?;
    let after_y_cut_instruction = get_file_text(after_y_cut_file_path).await?;

    let mut instructions = String::new();

    instructions.push_str("( Inicio cortar todas linhas verticais )");
    instructions.push('\n');

    for line in vertical_lines {

        instructions.push_str("( Inicio cortar linha vertical )");
        instructions.push('\n');

        instructions.push_str(&get_move_fast_speed(line.start.pos_x, line.start.pos_y));
        instructions.push('\n');

        instructions.push_str(&before_y_cut_instruction);
        instructions.push('\n');

        instructions.push_str(&get_move_slow_speed(line.end.pos_x, line.end.pos_y));
        instructions.push('\n');

        instructions.push_str(&after_y_cut_instruction);
        instructions.push('\n');

        instructions.push_str("( Fim cortar linha vertical )");
        instructions.push('\n');
    }

    instructions.push_str("( Fim cortar todas linhas verticais )");
    instructions.push('\n');
    instructions.push('\n');

    Ok(instructions)
}

async fn get_end_program(end_program_file_path: &str) -> Result<String, FileError> {
    let end_program_instruction = get_file_text(end_program_file_path).await?;

    let mut instructions = String::new(); 

    instructions.push_str("( Inicio codigo para finalizar )");
    instructions.push('\n');

    instructions.push_str(&end_program_instruction);
    instructions.push('\n');
    
    instructions.push_str("( Fim codigo para finalizar )");
    instructions.push('\n');
    instructions.push('\n');

    Ok(instructions)
}

fn get_move_slow_speed(x: i32, y: i32) -> String {
    format!("G1 X{} Y{}", x, y)
}

fn get_move_fast_speed(x: i32, y: i32) -> String {
    format!("G0 X{} Y{}", x, y)
}