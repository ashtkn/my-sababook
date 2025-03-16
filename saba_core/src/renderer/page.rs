use core::cell::RefCell;

use alloc::{
    rc::{Rc, Weak},
    string::String,
    vec::Vec,
};

use crate::{browser::Browser, display_item::DisplayItem, http::HttpResponse};

use super::{
    css::cssom::StyleSheet,
    dom::node::{ElementKind, NodeKind, Window},
    html::{parser::HtmlParser, token::HtmlTokenizer},
    layout::layout_view::LayoutView,
};

#[derive(Debug, Clone)]
pub struct Page {
    browser: Weak<RefCell<Browser>>,
    frame: Option<Rc<RefCell<Window>>>,
    style: Option<StyleSheet>,
    layout_view: Option<LayoutView>,
    display_items: Vec<DisplayItem>,
}

impl Page {
    pub fn new() -> Self {
        Self {
            browser: Weak::new(),
            frame: None,
            style: None,
            layout_view: None,
            display_items: Vec::new(),
        }
    }

    pub fn clicked(&self, position: (i64, i64)) -> Option<String> {
        let view = match &self.layout_view {
            Some(v) => v,
            None => return None,
        };
        if let Some(n) = view.find_node_by_position(position) {
            if let Some(parent) = n.borrow().parent().upgrade() {
                if let NodeKind::Element(e) = parent.borrow().node_kind() {
                    if e.kind() == ElementKind::A {
                        return e.get_attribute("href");
                    }
                }
            }
        }
        None
    }

    pub fn set_browser(&mut self, browser: Weak<RefCell<Browser>>) {
        self.browser = browser;
    }

    pub fn receive_response(&mut self, response: HttpResponse) {
        self.create_frame(response.body());
        self.set_layout_view();
        self.paint_tree();
    }

    fn create_frame(&mut self, html: String) {
        let html_tokenizer = HtmlTokenizer::new(html);
        let frame = HtmlParser::new(html_tokenizer).construct_tree();
        self.frame = Some(frame);
    }

    fn set_layout_view(&mut self) {
        let dom = match &self.frame {
            Some(frame) => frame.borrow().document(),
            None => return,
        };
        let style = match self.style.clone() {
            Some(style) => style,
            None => return,
        };
        let layout_view = LayoutView::new(dom, &style);
        self.layout_view = Some(layout_view);
    }

    fn paint_tree(&mut self) {
        if let Some(layout_view) = &self.layout_view {
            self.display_items = layout_view.paint();
        }
    }

    pub fn display_items(&self) -> Vec<DisplayItem> {
        self.display_items.clone()
    }

    pub fn clear_display_items(&mut self) {
        self.display_items = Vec::new();
    }
}
