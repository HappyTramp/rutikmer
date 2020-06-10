use sdl2::rect::Rect;

pub struct UI {
    pub history_rect: Rect,
    pub shuffle_rect: Rect,
    pub timer_rect:   Rect,
}

impl UI {
    pub fn new(width: u32, height: u32) -> UI {
        let default = Rect::new(0, 0, 0, 0);
        let mut ret = UI {history_rect: default, shuffle_rect: default, timer_rect:default};
        ret.set_layout(width, height);
        ret
    }

    pub fn set_layout(&mut self, width: u32, height: u32) {
        self.history_rect = Rect::new(0, 0, width / 3, height);
        self.shuffle_rect = Rect::new((width / 3) as i32, 0, width - width / 3, height / 4);
        self.timer_rect   = Rect::new((width / 3) as i32, (height / 4 + (height - height / 4) / 2) as i32,
                                      100, 40);
    }
}
