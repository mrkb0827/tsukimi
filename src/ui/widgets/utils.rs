use gtk::{
    SignalListItemFactory,
    prelude::*,
};

use super::{
    tu_list_item::{
        TuListItem,
        imp::PosterType,
    },
    tu_overview_item::{
        TuOverviewItem,
        imp::ViewGroup,
    },
};

use crate::ui::provider::tu_object::TuObject;

pub trait TuItemBuildExt {
    fn tu_item(&self, poster: PosterType) -> &Self;
    fn tu_overview_item(&self, view_group: ViewGroup) -> &Self;
}

impl TuItemBuildExt for SignalListItemFactory {
    fn tu_item(&self, poster: PosterType) -> &Self {
        self.connect_setup(move |_, item| {
            let tu_item = TuListItem::default();
            tu_item.set_poster_type(poster);

            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");
            list_item.set_child(Some(&tu_item));
            list_item
                .property_expression("item")
                .chain_property::<TuObject>("item")
                .bind(&tu_item, "item", gtk::Widget::NONE);
        });
        self
    }

    fn tu_overview_item(&self, view_group: ViewGroup) -> &Self {
        self.connect_setup(move |_, item| {
            let tu_item = TuOverviewItem::default();
            tu_item.set_view_group(view_group);
            let list_item = item
                .downcast_ref::<gtk::ListItem>()
                .expect("Needs to be ListItem");
            list_item.set_child(Some(&tu_item));
            list_item
                .property_expression("item")
                .chain_property::<TuObject>("item")
                .bind(&tu_item, "item", gtk::Widget::NONE);
        });
        self
    }
}

pub const TU_ITEM_POST_SIZE: (i32, i32) = (167, 260);
pub const TU_ITEM_VIDEO_SIZE: (i32, i32) = (250, 141);
pub const TU_ITEM_SQUARE_SIZE: (i32, i32) = (190, 190);
pub const TU_ITEM_BANNER_SIZE: (i32, i32) = (375, 70);
