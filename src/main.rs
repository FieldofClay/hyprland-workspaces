use hyprland::data::{Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListenerMutable as EventListener;
use hyprland::shared::{HyprData,HyprError};
use hyprland::shared::HyprDataActive;
use hyprland::Result;
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

fn get_workspace_windows(monitor: &str)-> Result<Vec<WorkspaceCustom>>  {
    // get all workspaces
    let mut workspaces: Vec<_> = Workspaces::get()?
        .into_iter()
        .collect();
    workspaces.sort_by_key(|w| w.id);

    //get active workspace
    let active_workspace_id: i32;
    if monitor == "ALL" {
        active_workspace_id = Workspace::get_active()?.id;
    } else {
        active_workspace_id = Monitors::get()?
            .find(|m| m.name == monitor)
            .ok_or_else(|| {
                HyprError::NotOkDispatch("No monitor found".to_string())
            })?
            .active_workspace
            .id;
    }
    //active monitor name
    let active_monitor_name = Monitors::get()?
        .find(|m| m.focused == true)
        .ok_or_else(|| {
            HyprError::NotOkDispatch("No active monitor found".to_string())
        })?
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
    return Ok(out_workspaces);
}

fn get_all_advanced()-> Result<Vec<MonitorCustom>>  {
    let monitors = Monitors::get()?;
    let mut out_monitors: Vec<MonitorCustom> = Vec::new();
    for m in monitors {
        let workspaces = get_workspace_windows(&m.name)?;
        let mc: MonitorCustom = MonitorCustom {
            name: m.name,
            workspaces: workspaces,
        };
        out_monitors.push(mc);
    }
    Ok(out_monitors)
}


fn output(monitor: &str) {
    if monitor == "_" {
        println!("{}", json!(get_all_advanced().unwrap_or_default()).to_string());
    } else {
        println!("{}", json!(get_workspace_windows(monitor).unwrap_or_default()).to_string());
    }
}


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    //check args
    if args.len() != 2 || args[1].eq("-h") || args[1].eq("--help") {
        println!("{HELP}");
        std::process::exit(0);
    }
    let mon = Arc::new(String::from(args[1].to_string()));
    if let None = Monitors::get()?
        .find(|m| m.name == mon.to_string() || mon.to_string() == "ALL" || mon.to_string() == "_") {
            println!("Unable to find monitor {mon}");
            std::process::exit(0);
    }

    output(&mon);
    // Create a event listener
    let mut event_listener = EventListener::new();
    let mon_clone = Arc::clone(&mon);
    event_listener.add_workspace_change_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_workspace_added_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_workspace_destroy_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_workspace_moved_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_monitor_added_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_monitor_removed_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_close_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_open_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_active_monitor_change_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_active_window_change_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_close_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_fullscreen_state_change_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_moved_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_layer_open_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_layer_closed_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_urgent_state_handler(move |_, _| {
        output(&mon_clone);
    });
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_title_change_handler(move |_, _| {
        output(&mon_clone);
    });

    event_listener.start_listener()
    
}
