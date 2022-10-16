use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Display {
    pub id: u32,
    pub uuid: String,
    pub index: u32,
    pub frame: Frame,
    pub spaces: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Space {
    pub id: usize,
    pub uuid: String,
    pub index: usize,
    pub label: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub display: usize,
    pub windows: Vec<usize>,
    #[serde(alias = "first-window")]
    pub first_window: usize,
    #[serde(alias = "last-window")]
    pub last_window: usize,
    #[serde(alias = "has-focus")]
    pub has_focus: bool,
    #[serde(alias = "is-visible")]
    pub is_visible: bool,
    #[serde(alias = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Window {
    pub id: usize,
    pub pid: usize,
    pub app: String,
    pub title: String,
    pub frame: Frame,
    pub role: String,
    pub subrole: String,
    pub tags: String,
    pub display: usize,
    pub space: usize,
    pub level: usize,
    pub opacity: f64,
    #[serde(alias = "split-type")]
    pub split_type: String,
    #[serde(alias = "split-child")]
    pub split_child: String,
    #[serde(alias = "stack-index")]
    pub stack_index: usize,
    #[serde(alias = "can-move")]
    pub can_move: bool,
    #[serde(alias = "can-resize")]
    pub can_resize: bool,
    #[serde(alias = "has-focus")]
    pub has_focus: bool,
    #[serde(alias = "has-shadow")]
    pub has_shadow: bool,
    #[serde(alias = "has-border")]
    pub has_border: bool,
    #[serde(alias = "has-parent-zoom")]
    pub has_parent_zoom: bool,
    #[serde(alias = "has-fullscreen-zoom")]
    pub has_fullscreen_zoom: bool,
    #[serde(alias = "is-native-fullscreen")]
    pub is_native_fullscreen: bool,
    #[serde(alias = "is-visible")]
    pub is_visible: bool,
    #[serde(alias = "is-minimized")]
    pub is_minimized: bool,
    #[serde(alias = "is-hidden")]
    pub is_hidden: bool,
    #[serde(alias = "is-floating")]
    pub is_floating: bool,
    #[serde(alias = "is-sticky")]
    pub is_sticky: bool,
    #[serde(alias = "is-topmost")]
    pub is_topmost: bool,
    #[serde(alias = "is-grabbed")]
    pub is_grabbed: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
