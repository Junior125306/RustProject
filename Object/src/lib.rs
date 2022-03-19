pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub compoents: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.compoents.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        //绘制图形
    }
}
