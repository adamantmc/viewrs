pub mod parser {
    pub use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[arg(short, long, help="Path to a file containing any-separated values")]
        pub file: String,
    
        #[arg(short, long, default_value_t=0, help="Number of rows to skip")]
        pub offset: usize,

        #[arg(short, long, default_value_t=',', help="Character which separates values of different columns")]
        pub separator: char,

        #[arg(long="whitespaces", default_value_t=4, help="Number of whitespaces used to separate columns (after padding)")]
        pub whitespace: usize,

        #[arg(short='w', long="where", default_value_t=String::from(""), help="SQL that will be appended to the WHERE clause during data reading")]
        pub where_clause: String,

        #[arg(short='c', long="columns", default_value_t=String::from(""), help="SQL Select string that will be used in the SQL query during data reading")]
        pub columns: String,

        #[arg(short='q', long="query", default_value_t=String::from(""), help="Full SQL query for data reading (table name is fixed to 'data')")]
        pub query: String,

        #[arg(long="output", default_value_t=String::from(""), help="Path to file - if provided, all output is written to this file")]
        pub output: String,

        #[arg(long="no-header", default_value_t=false, help="Flag which specifies that the input file has no header")]
        pub no_header: bool,
    }
}