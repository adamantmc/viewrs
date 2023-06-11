use polars::prelude::*;


pub fn get_max_column_lengths(df: &DataFrame) -> Vec<usize> {
    let column_names = df.get_column_names();
    let column_lengths: Vec<usize> = column_names.iter().map(|x| x.len()).collect();
    let mut max_value_lengths: Vec<usize> = Vec::new();
    let df_length = df.height();
    
    for column_name in column_names {
        if df_length > 0 {
            let all_lengths = df.column(column_name).unwrap().iter().map(|x| x.to_string().len());
            max_value_lengths.push(all_lengths.max().unwrap());
        }
        else {
            max_value_lengths.push(0 as usize);
        }
    }

    let mut max_lengths: Vec<usize> = Vec::new();

    for i in 0..column_lengths.len() {
        max_lengths.push(*[column_lengths[i], max_value_lengths[i]].iter().max().unwrap());
    }

    return max_lengths;
}


pub fn left_pad_string(s: &str, to: usize) -> String {
    return format!("{: >to$}", s, to=to);
} 


pub fn values_to_str(values: &Vec<&str>, column_lengths: &Vec<usize>, column_distance: usize, pad: bool) -> String {
    let mut s = String::from("");
    let column_whitespace = " ".repeat(column_distance as usize);

    for (index, col_length) in column_lengths.iter().enumerate() {
        let mut disp_str = *values.get(index).unwrap();

        if pad {
            let left_padded = left_pad_string(&disp_str, *col_length);
            disp_str = left_padded.as_ref();
            s.push_str(disp_str);
        }
        else {
            s.push_str(disp_str);
        }

        s.push_str(&column_whitespace);
    }

    return s; 
}


pub fn df_to_str_lines(df: &DataFrame, column_distance: usize, pad: bool) -> Vec<String> {
    let column_lengths = get_max_column_lengths(&df);
    let mut strings: Vec<String> = Vec::new();

    strings.push(values_to_str(&df.get_column_names(), &column_lengths, column_distance, pad));
    strings.push("-".repeat(strings[0].len()));

    for i in 0..df.height() {
        let v: Vec<String> = df.get(i).unwrap().iter().map(|x| x.to_string()).collect();
        let v: Vec<&str> = v.iter().map(|x| x.as_ref()).collect();

        strings.push(values_to_str(&v, &column_lengths, column_distance, pad));
    }

    return strings;
}   


pub fn format_lines(lines: &Vec<String>, width: usize, offset: usize) -> String {
    let mut processed_strings: Vec<String> = Vec::new();

    for string in lines {
        let mut new_str: String = string[offset..].to_string();

        if new_str.len() < width {
            new_str.push_str("\n");
        }
        else {
            new_str = new_str[..width].to_string();
        }

        processed_strings.push(new_str);
    }

    return processed_strings.join("");
}
