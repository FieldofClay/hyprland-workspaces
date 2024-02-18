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
                        Use the special keywords ALL to track all monitors or _ to track all monitors and output monitor name information as well
";

#[derive(Serialize)]
struct WorkspaceCustom {
    pub name: String,
    pub id: i32,
    pub active: bool,
    pub class: String,
}

#[derive(Serialize)]
struct MonitorCustom {
    pub name: String,
    pub workspaces: Vec<WorkspaceCustom>
}

fn get_workspace_windows(monitor: &str)-> Vec<WorkspaceCustom>  {
    // get all workspaces
    let mut workspaces: Vec<_> = Workspaces::get().expect("unable to get workspaces").into_iter().collect();
    workspaces.sort_by_key(|w| w.id);

    //get active workspace
    let active_workspace_id: i32;
    if monitor == "ALL" {
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

    for workspace in workspaces.iter().filter(|m| m.monitor == monitor || monitor == "ALL" || monitor == "_") {
            let mut active = false;
            let mut class = format!("workspace-button w{}",workspace.id);
            if active_workspace_id == workspace.id && (active_monitor_name == monitor || monitor == "ALL") {
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
    return out_workspaces;
}

fn output(monitor: &str) {
    if monitor == "_" {
        let monitors = Monitors::get().expect("unable to get monitors");
        let mut out_monitors: Vec<MonitorCustom> = Vec::new();
        for m in monitors {
            let workspaces = get_workspace_windows(&m.name);
            let mc: MonitorCustom = MonitorCustom {
                name: m.name,
                workspaces: workspaces,
            };
            out_monitors.push(mc);
        }
        println!("{}", json!(out_monitors).to_string());
    } else {
        println!("{}", json!(get_workspace_windows(monitor)).to_string());
    }
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
        .find(|m| m.name == mon || mon == "ALL" || mon == "_") {
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
    event_listener.add_layer_open_handler(|_, _| {
        output!();
     });
    event_listener.add_layer_closed_handler(|_, _| {
        output!();
     });
    event_listener.add_urgent_state_handler(|_, _| {
        output!();
    });
    event_listener.add_window_title_change_handler(|_, _| {
        output!();
    });

    event_listener.start_listener()
    
}
