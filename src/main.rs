use hyprland::data::{Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::HyprData;
use hyprland::shared::HyprDataActive;
use hyprland::Result;
use std::env;
use serde::Serialize;
use serde_json::json;

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

fn output(monitor: &str) {
    // get all workspaces
    let mut workspaces: Vec<_> = Workspaces::get().expect("unable to get workspaces").into_iter().collect();
    workspaces.sort_by_key(|w| w.id);

    //get active workspace
    let mut active_workspace_id = -499;
    if monitor == "_" {
        active_workspace_id = Workspace::get_active().expect("unable to get active workspace").id;
    } else {
        active_workspace_id = Monitors::get()
            .expect("unable to get monitors")
            .find(|m| m.name == monitor)
            .unwrap()
            .active_workspace
            .id;
    }
    //active monitor name
    let active_monitor_name = Monitors::get()
        .expect("unable to get monitors")
        .find(|m| m.focused == true)
        .unwrap()
        .name;

    let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();

    for workspace in workspaces.iter().filter(|m| m.monitor == monitor || monitor == "_") {
            let mut active = false;
            let mut class = format!("workspace-button w{}",workspace.id);
            if active_workspace_id == workspace.id && active_monitor_name == monitor {
                class = format!("{} workspace-active wa{}", class, workspace.id);
                active = true;
            }

            let ws: WorkspaceCustom = WorkspaceCustom {
                name: workspace.name.clone(),
                id: workspace.id,
                active,
                class,
            };
            out_workspaces.push(ws);
    }
    println!("{}", json!(out_workspaces).to_string());
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    //check args
    if args.len() != 2 || args[1].eq("-h") || args[1].eq("--help") {
        println!("{HELP}");
        std::process::exit(0);
    }
    let mon = env::args().nth(1).unwrap();
    if let None = Monitors::get()
        .expect("unable to get monitors")
        .find(|m| m.name == mon || mon == "_") {
            println!("Unable to find monitor {mon}");
            std::process::exit(0);
    }

    macro_rules! output {
        () => {
            output(&env::args().nth(1).unwrap());
        };
    }
    output!();
    // Create a event listener
    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_added_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_destroy_handler(|_, _| {
        output!();
    });
    event_listener.add_workspace_moved_handler(|_, _| {
        output!();
    });
    event_listener.add_monitor_added_handler(|_, _| {
        output!();
    });
    event_listener.add_monitor_removed_handler(|_, _| {
        output!();
    });
    event_listener.add_window_close_handler(|_, _| {
        output!();
    });
    event_listener.add_window_open_handler(|_, _| {
        output!();
    });
    event_listener.add_active_monitor_change_handler(|_, _| {
        output!();
    });
    event_listener.add_active_window_change_handler(|_, _| {
        output!();
    });
    event_listener.add_window_close_handler(|_, _| {
        output!();
    });
    event_listener.add_fullscreen_state_change_handler(|_, _| {
        output!();
    });
    event_listener.add_window_moved_handler(|_, _| {
        output!();
    });

    event_listener.start_listener()
    
}
