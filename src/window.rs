use crate::json::Window;
use anyhow::Result;

pub struct WindowCommands;

impl WindowCommands {
    pub fn running_apps(windows: Vec<Window>) -> Vec<Window> {
        windows
            .iter()
            .filter(|w| !w.app.is_empty())
            .map(|w| w.clone())
            .collect::<Vec<_>>()
    }

    fn find_window_with_app(windows: Vec<Window>, name: &str) -> Vec<Window> {
        let apps = Self::running_apps(windows);
        let has = apps
            .iter()
            .filter(|w| w.app == name)
            .map(|f| f.clone())
            .collect::<Vec<_>>();
        has
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_find_window_with_app() {
        let window = Window {
            id: 1,
            app: "foo".to_string(),
            ..Window::default()
        };
        let windows = vec![window];
        let actual = WindowCommands::find_window_with_app(windows, "foo");
        assert_eq!(actual.len(), 1);
    }
}
