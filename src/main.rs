use hyprland::data::{Monitors, Workspaces};
use hyprland::event_listener::{EventListenerMutable as EventListener};
use hyprland::shared::{HResult, HyprData};
use std::env;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

const HELP: &str = "\
hyprland-workspaces: a multi monitor aware hyprland workspaces json widget generator for eww/waybar.

USAGE:
  hyprland-workspaces MONITOR

FLAGS:
  -h, --help            Prints help information

ARGS:
  <MONITOR>             Monitor to track windows/workspaces on
";

#[derive(Serialize)]
struct WorkspaceCustom {
    pub name: String,
    pub id: i32,
    pub active: bool,
    pub class: String,
}

#[derive(Clone)]
struct WorkspacesWidget {
    pub monitor: String,
}

impl WorkspacesWidget {
    fn new(monitor: &str) -> Self {
        Self { 
            monitor: monitor.to_owned(),       
        }
    }

    fn output(&self) {
        // get all workspaces
        let mut workspaces: Vec<_> = Workspaces::get().expect("unable to get workspaces").into_iter().collect();
        workspaces.sort_by_key(|w| w.id);
    
        //get active workspace
        let active_workspace = Monitors::get()
            .expect("unable to get monitors")
            .find(|m| m.name == self.monitor)
            .unwrap()
            .active_workspace;
        //active monitor name
        let active_monitor_name = Monitors::get()
            .expect("unable to get monitors")
            .find(|m| m.focused == true)
            .unwrap()
            .name;
    
        let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();
    
        for workspace in workspaces.iter() {
            if workspace.monitor == self.monitor {
                let mut active = false;
                let mut class = format!("workspace-button w{}",workspace.id);
                if active_workspace.name == workspace.name && active_monitor_name == self.monitor {
                    class = format!("{} workspace-active wa{}", class, workspace.id);
                    active = true;
                }

                let ws: WorkspaceCustom = WorkspaceCustom {
                    name: workspace.name.clone(),
                    id: workspace.id,
                    active: active,
                    class: class,
                };
                out_workspaces.push(ws);
    
            }
        }
        println!("{}", json!(out_workspaces).to_string());
        
    }

}




fn main() -> HResult<()> {
    let args: Vec<String> = env::args().collect();
    //check args
    if args.len() != 2 || args[1].eq("-h") || args[1].eq("--help") {
        println!("{HELP}");
        std::process::exit(0);
    }
    let monitor = args[1].to_string();
    let workspace_widget = Arc::new(WorkspacesWidget::new(&monitor));

    workspace_widget.output();
    // Create a event listener
    let mut event_listener = EventListener::new();
    
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_workspace_change_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_workspace_added_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_workspace_destroy_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_workspace_moved_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_monitor_added_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_monitor_removed_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_window_close_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_window_open_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_active_monitor_change_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_active_window_change_handler(move |_, _| {
        ww_clone.output();
    });
    let ww_clone = Arc::clone(&workspace_widget);
    event_listener.add_window_close_handler(move |_, _| {
        ww_clone.output();
    });

    event_listener.start_listener()
    
}
