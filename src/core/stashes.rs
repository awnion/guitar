#[rustfmt::skip]
use std::{
    cell::{
        RefCell
    },
    rc::Rc,
    collections::{
        HashMap
    },
};
#[rustfmt::skip]
use ratatui::{
    style::{
        Color
    }
};
#[rustfmt::skip]
use crate::{
    helpers::{
        palette::{
            Theme
        },
        colors::{
            ColorPicker
        }
    }
};

pub struct Stashes {
    pub colors: HashMap<u32, Color>,
}

impl Default for Stashes {

    fn default() -> Self {
        Self {
            colors: Default::default(),
        }
    }
}

impl Stashes {

    pub fn feed(
        &mut self,
        color: &Rc<RefCell<ColorPicker>>,
        stashes_lanes: &HashMap<u32, usize>,
    ) {

        // Initialize
        self.colors = HashMap::new();

        // Set tag colors
        for (oidi, &lane_idx) in stashes_lanes.iter() {
            self.colors.insert(*oidi, color.borrow().get_lane(lane_idx));
        }
    }

    pub fn get_color(&self, theme: &Theme, stash_alias: &u32) -> Color {
        *self.colors.get(stash_alias).unwrap_or(&theme.COLOR_TEXT)
    }
}
