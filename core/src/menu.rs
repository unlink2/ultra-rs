use super::font::*;
use super::math::*;
use super::render::RenderContext;

pub const MAX_ENTRIES: usize = 15;
pub const MAX_TITLE_LEN: usize = 15;

/**
 * An action function that returns either None
 * or a usize value
 */
pub type EntryFn<T> = fn(entry: &mut Entry<T>, data: T) -> Option<usize>;

#[derive(Clone, Copy)]
pub struct Entry<T>
where T: Copy + Clone {
    pub title: [char; MAX_TITLE_LEN],
    pub active: bool,
    pub update: EntryFn<T>,
    pub action: EntryFn<T>
}

pub fn no_op<T: Copy + Clone>(_entry: &mut Entry<T>, _data: T) -> Option<usize> {
    return None;
}

impl<T> Entry<T>
where T: Copy + Clone {
    pub fn empty() -> Self {
        Self {
            title: ['1'; MAX_TITLE_LEN],
            update: no_op,
            action: no_op,
            active: false
        }
    }

    pub fn new(title: &str, update: EntryFn<T>, action: EntryFn<T>) -> Self {
        let mut title_ca = ['\0'; MAX_TITLE_LEN];

        for i in 0..min(MAX_TITLE_LEN, title.len()) {
            title_ca[i] = title.chars().nth(i).unwrap_or('\0');
        }

        Self {
            title: title_ca,
            update,
            action,
            active: true
        }
    }

    pub fn checkbox(title: &str, update: EntryFn<T>, action: EntryFn<T>, value: bool) -> Self {
        let mut title_ca = ['\0'; MAX_TITLE_LEN];

        title_ca[0] = '[';
        if value { title_ca[1] = 'x' } else { title_ca[1] = ' ' };
        title_ca[2] = ']';
        for i in 3..min(MAX_TITLE_LEN, title.len()+3) {
            title_ca[i] = title.chars().nth(i-3).unwrap_or('\0');
        }

        Self {
            title: title_ca,
            update,
            action,
            active: true
        }
    }

    pub fn set_checkbox(&mut self, value: bool) {
        if value {
            self.title[1] = 'x';
        } else {
            self.title[1] = ' ';
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.title.fill(' ');
        for i in 0..min(MAX_TITLE_LEN, title.len()) {
            self.title[i] = title.chars().nth(i).unwrap_or('\0');
        }
    }

    pub fn draw(&mut self, ctxt: &mut dyn RenderContext, x: i32, y: i32) {
        ctxt.cputs(&self.title, x, y);
    }

    pub fn call_update(&mut self, data: T) {
        (self.update)(self, data);
    }

    pub fn activate(&mut self, data: T) {
        (self.action)(self, data);
    }
}

pub struct Menu<T>
where T: Copy + Clone {
    cursor: isize,
    x: i32,
    y: i32,
    pub active: bool,
    toggle_timer_max: u16,
    toggle_timer: u16,

    open_action: Entry<T>,
    close_action: Entry<T>,
    back_action: Entry<T>,
    update_action: Entry<T>,
    entries: [Entry<T>; MAX_ENTRIES],
    active_entries: usize
}

impl<T> Menu<T>
where T: Copy + Clone {
    pub fn new(x: i32, y: i32,
        open_action: Entry<T>,
        close_action: Entry<T>,
        back_action: Entry<T>,
        update_action: Entry<T>,
        entries_proto: &[Entry<T>]) -> Self {
        let mut entries = [Entry::empty(); MAX_ENTRIES];

        for i in 0..min(MAX_ENTRIES, entries_proto.len()) {
            entries[i] = entries_proto[i];
        }

        Self {
            cursor: 0,
            active: false,
            toggle_timer_max: 10,
            toggle_timer: 0,
            open_action,
            close_action,
            back_action,
            update_action,
            x, y, entries,
            active_entries: entries_proto.len()
        }
    }

    pub fn draw(&mut self, ctxt: &mut dyn RenderContext) {
        if !self.active { return; }

        let start_x: i32 = self.x;
        let mut start_y: i32 = self.y;

        let mut counter: isize = 0;
        for entry in &mut self.entries {
            if !entry.active { continue; }

            if self.cursor == counter {
                ctxt.puts(">", start_x, start_y);
            }
            entry.draw(ctxt, start_x + CHAR_W as i32 + 2, start_y);
            start_y += CHAR_H as i32 + 2;

            counter += 1;
        }
    }

    pub fn update(&mut self, data: T) {
        if self.toggle_timer > 0 {
            self.toggle_timer -= 1;
        }
        if !self.active { return; }

        self.update_action.activate(data);

        for entry in &mut self.entries {
            if !entry.active { continue; }

            entry.call_update(data);
        }
    }

    pub fn inc_cursor(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.active_entries as isize {
            self.cursor = 0;
        }
    }

    pub fn dec_cursor(&mut self) {
        self.cursor -= 1;
        if self.cursor < 0 {
            self.cursor = self.active_entries as isize - 1;
        }
    }

    pub fn activate(&mut self, data: T) {
        if !self.active { return; }

        self.entries[self.cursor as usize].activate(data);
    }

    pub fn open(&mut self, data: T) {
        self.active = true;
        self.open_action.activate(data);
    }

    pub fn close(&mut self, data: T) {
        self.active = false;
        self.close_action.activate(data);
    }

    pub fn back(&mut self, data: T) {
        self.toggle_timer = 0;
        self.back_action.activate(data);
    }

    pub fn toggle(&mut self, data: T) {
        if self.toggle_timer > 0 { return; }

        self.toggle_timer = self.toggle_timer_max;
        if self.active {
            self.close(data);
        } else {
            self.open(data);
        }
    }
}
