use polars::prelude::*;
use polars_sql::SQLContext;
use std::process;
use std::fs::File;

use super::cli::parser::*;
use super::tui::terminate_curses;

const PARSING_ERROR_MESSAGE: &str = "Error: could not parse data - check that the separator and the query (if provided) are correct.";
const IO_ERROR_MESSAGE: &str = "Error: could not read input file. Make sure that the provided path is correct.";


pub fn build_query(args: &Args, lf_name: &str) -> String {
    let mut query = String::new();
    

    // If args.query is provided, use that as the query
    if args.query.len() != 0 {
        return args.query.clone();
    }

    // Else we build the query from the other arguments
    // Add select clause
    if args.columns.len() != 0 {
        query.push_str(format!("SELECT {}", args.columns).as_ref());
    }
    else {
        query.push_str("SELECT *");

    }

    // Add FROM clause
    query.push_str(format!(" FROM {}", lf_name).as_ref());

    // Add WHERE clause
    if args.where_clause.len() != 0 {
        query.push_str(format!(" WHERE {}", args.where_clause).as_ref());
    }

    return query;
}


pub fn read_file(args: &Args) -> LazyFrame {
    let lf_read_result = LazyCsvReader::new(args.file.clone())
        .with_delimiter(args.separator as u8)
        .has_header(!args.no_header).finish();

    let lf = match lf_read_result {
        Ok(lf) => lf,
        Err(_) => {
            eprintln!("{}", IO_ERROR_MESSAGE);
            terminate_curses();
            process::exit(-1);
        }
    };

    return lf;
}


pub fn get_number_of_lines(sql_ctx: &mut SQLContext, query: &String) -> usize {
    let lf = query_lazyframe(sql_ctx, query).select([count()]);
    let df = collect_or_error(lf);

    let v: Vec<String> = df.get(0).unwrap().iter().map(|x| x.to_string()).collect();
    let row_count = v[0].parse::<usize>().unwrap();

    return row_count;
}


pub fn query_lazyframe(sql_ctx: &mut SQLContext, query: &String) -> LazyFrame {

    let lf = match sql_ctx.execute(query.as_ref()) {
        Ok(lf) => lf,
        Err(_) => {
            eprintln!("{}", PARSING_ERROR_MESSAGE);
            terminate_curses();
            process::exit(-1);
        }
    };

    return lf;
}


pub fn collect_or_error(lf: LazyFrame) -> DataFrame {
    let df = match lf.collect() {
        Ok(df) => df,
        Err(_) => {
            eprintln!("{}", PARSING_ERROR_MESSAGE);
            terminate_curses();
            process::exit(-1);
        }
    };

    return df;
}


pub fn init_sql_context(args: &Args, lf_name: &str) -> SQLContext {
    let lf = read_file(&args);
    let mut sql_ctx = SQLContext::new();
    
    sql_ctx.register(lf_name, lf);
    
    return sql_ctx;
}


pub fn write_to_file(lf: LazyFrame, output_path: &String, separator: char) {
    let buffer = File::create(output_path).unwrap();
    CsvWriter::new(buffer).with_delimiter(separator as u8).finish(&mut collect_or_error(lf)).unwrap();
    process::exit(0);
}
