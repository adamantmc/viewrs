use pancurses::Input;
use polars_sql::SQLContext;

mod cli;
mod queries;
mod tui;
mod format;

use cli::parser::*;
use queries::*;
use tui::{init_curses, terminate_curses, DisplaySettings};
use format::*;


/// Builds and returns a vector of strings that will be printed out in the console.
/// Strings are padded to match column length (either the length of the column name, or the length of the biggest value in the viewport), 
/// and they're also padded with extra whitespaces (args.whitespace).
/// 
/// # Arguments
/// * `args` - The arguments passed to the CLI
/// * `sql_ctx` - The Polars SQLContext with the LazyFrame containing the data registered
/// * `query` - The SQL query that defines the data that will be shown
/// * `display_settings` - Struct containing settings that affect display (offsets, dimensions).
fn get_display_strings(args: &Args, sql_ctx: &mut SQLContext, query: &String, display_settings: &DisplaySettings) -> Vec<String> {
    let lf_view = query_lazyframe(sql_ctx, &query).slice(display_settings.offset as i64, display_settings.page_size as u32);
    let df_view = collect_or_error(lf_view);
    return df_to_str_lines(&df_view, args.whitespace, true)
}


/// Runs the viewer in TUI mode, which prints the file to the console using curses.
/// 
/// # Arguments
/// 
/// * `args` - The arguments passed to the CLI
/// * `sql_ctx` - The Polars SQLContext with the LazyFrame containing the data registered
/// * `query` - The SQL query that defines the data that will be shown
fn run_tui(args: &Args, sql_ctx: &mut SQLContext, query: &String) {
    let df_length: usize = get_number_of_lines(sql_ctx, &query);
    let window = init_curses();

    let mut lines: Vec<String> = Vec::new();
    let mut recompute: bool = true;
    let mut line_length: usize = 0;
    
    let mut display_settings = DisplaySettings {
        offset: args.offset,
        // Page size is number of rows minus 1 (the header)
        page_size: window.get_max_y() as usize - 2,
        x_offset: 0,
        window_width: window.get_max_x() as usize
    };

    loop {
        if recompute {
            lines.clear();
            lines.extend(get_display_strings(&args, sql_ctx, &query, &display_settings)); 
            line_length = lines[0].len();
        }
    
        window.clear();
        window.addstr(format_lines(&lines, display_settings.window_width, display_settings.x_offset));
        
        recompute = true;

        match window.getch() {
            // Some(Input::Character(c)) => { window.addch(c); },
            Some(Input::KeyDown) => display_settings.handle_key_down_arr(df_length),
            Some(Input::KeyUp) => display_settings.handle_key_up_arr(),
            Some(Input::KeyRight) => {display_settings.handle_key_right_arr(line_length); recompute = false;},
            Some(Input::KeyLeft) => {display_settings.handle_key_left_arr(); recompute = false;},
            Some(Input::KeyNPage) => display_settings.handle_key_pgdown(df_length),
            Some(Input::KeyPPage) => display_settings.handle_key_pgup(),
            Some(Input::Character('q')) => break,
            Some(Input::KeyResize) => display_settings.window_resized(&window),
            _ => (),
        }
    }

    terminate_curses();
}


/// Runs the viewer. Depending on the arguments passed, `run` will either run the TUI (where the data will be displayed)
/// or just output the data defined by the given query in the specified output file (`args.output`).
///
/// This function also initializes the SQLContext that will be used to query the data. The name of the table used in the SQL queries 
/// is fixed to `data`.
///  
/// # Arguments
/// 
/// * `args` - The arguments passed to the CLI
fn run(args: &Args) {
    let lf_name = "data";
    let query = build_query(&args, lf_name);
    let mut sql_ctx = init_sql_context(&args, lf_name);

    if args.output.len() != 0 {
        write_to_file(query_lazyframe(&mut sql_ctx, &query), &args.output, args.separator)
    }
    else {
        run_tui(&args, &mut sql_ctx, &query);
    } 
}


/// Entrypoint of the CLI.
fn main() {
    let args = Args::parse();
    run(&args);
 }
