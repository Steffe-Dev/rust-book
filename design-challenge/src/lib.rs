use std::fmt::{Debug, Display};

// Option 1: children must be Self
pub trait Widget1: Sized {
    fn render(&self) -> Vec<Self>;
}

// Option 2: children are a trait parameter
pub trait Widget2<Children> {
    fn render(&self) -> Vec<Children>;
}

// Option 3: children are an associated type
pub trait Widget3 {
    type Children: Widget3;
    fn render(&self) -> Vec<Self::Children>;
}

// Option 4: children are a reference trait object
pub trait Widget4 {
    fn render(&self) -> Vec<&dyn Widget4>;
}

// Option 5: children are a boxed trait object
pub trait Widget5 {
    fn render(&self) -> Vec<Box<dyn Widget5>>;
}

// Question 1a
#[derive(PartialEq)]
enum WidgetImpl {
    Button,
    Textbox,
    Hamburger,
}

impl Widget1 for WidgetImpl {
    fn render(&self) -> Vec<Self> {
        vec![WidgetImpl::Button]
    }
}

// impl<Children> Widget2<Children> for WidgetImpl {
//     fn render(&self) -> Vec<Children> {
//         println!("Doing Something...");
//         vec![]
//     }
// }

// impl Widget3 for WidgetImpl {
//     type Children = WidgetImpl;
//     fn render(&self) -> Vec<Self::Children> {
//         println!("Doing Something...");
//         vec![WidgetImpl::Button]
//     }
// }

// impl Widget4 for WidgetImpl {
//     fn render(&self) -> Vec<&dyn Widget4> {
//         println!("Doing Something...");
//         vec![WidgetImpl::Button]
//     }
// }

// Question 1b
// struct Button;
// struct Textbox;
// struct Hamburger;

// impl Widget1 for Button {
//     fn render(&self) -> Vec<Self> {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let widget = WidgetImpl::Button;
        assert!(widget.render().contains(&WidgetImpl::Button));
    }
}
