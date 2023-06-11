use pancurses::{endwin, initscr, Window, noecho, resize_term};
use std::cmp;


pub fn init_curses() -> Window {
    let window = initscr();
    window.keypad(true);
    noecho();

    return window;
}

pub fn terminate_curses() {
    endwin();
}

pub struct DisplaySettings {
    pub offset: usize,
    pub page_size: usize,
    pub x_offset: usize,
    pub window_width: usize
}

impl DisplaySettings {
    pub fn handle_key_down_arr(&mut self, df_length: usize) {
        if self.offset != df_length - 1 {
            self.offset += 1;
        }
    }

    pub fn handle_key_up_arr(&mut self) {
        if self.offset != 0 { 
            self.offset -= 1;
        }
    }

    pub fn handle_key_left_arr(&mut self) {
        if self.x_offset != 0 { 
            self.x_offset -= 1; 
        }
    }

    pub fn handle_key_right_arr(&mut self, line_length: usize) {
        if line_length > self.window_width && self.x_offset != line_length - 1 { 
            self.x_offset += 1; 
        }
    }

    pub fn handle_key_pgdown(&mut self, df_length: usize) {
        self.offset = cmp::min(self.offset + self.page_size, df_length - 1);
    }
    
    pub fn handle_key_pgup(&mut self) {
        if self.offset > self.page_size {
            self.offset = self.offset - self.page_size; 
        } 
        else { 
            self.offset = 0; 
        }
    }

    pub fn window_resized(&mut self, window: &Window) {
        resize_term(0, 0); 
        self.page_size = window.get_max_y() as usize;
        self.window_width = window.get_max_x() as usize;
        self.x_offset = 0; 
    }
}
