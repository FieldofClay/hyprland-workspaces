use clap::Parser;
use flexi_logger::{FileSpec, Logger};
use hyprland::data::{Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, HyprDataActive, HyprError};
use hyprland::Result;
use log;
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(version = "2.1.0", about = "A multi monitor aware hyprland workspaces json widget generator for eww.", long_about = None)]
struct Args {
  // Specify the number of workspaces per monitor
  #[arg(help = "Specify the number of workspaces per monitor", short ='w', long = "workspaces", default_value = None)]
  workspace_count: Option<i8>,

  monitor: String
}

#[derive(Serialize, Clone, Debug)]
struct WorkspaceCustom {
    pub name: String,
    pub id: i32,
    pub active: bool,
    pub occupied: bool,
    pub class: String,
}

#[derive(Serialize)]
struct MonitorCustom {
    pub name: String,
    pub workspaces: Vec<WorkspaceCustom>,
}

fn fill_empty(workspaces: Vec<WorkspaceCustom>, workspaces_count: i8) -> Vec<WorkspaceCustom> {
  let mut res: Vec<WorkspaceCustom> = vec![];
  let mut workspace_iter = workspaces.iter().peekable();
  let mut current_id: i8 = 1;

  while current_id <= workspaces_count {
      if let Some(w) = workspace_iter.peek() {
          if current_id as i32 == w.id {
              res.push(workspace_iter.next().unwrap().clone());
          } else {
              res.push(WorkspaceCustom {
                  name: current_id.to_string(),
                  id: current_id.into(),
                  active: false,
                  occupied: false,
                  class: "workspace-unoccupied wU".to_string(),
              });
          }
      } else {
          res.push(WorkspaceCustom {
              name: current_id.to_string(),
              id: current_id.into(),
              active: false,
              occupied: false,
              class: "workspace-unoccupied wU".to_string(),
          });
      }
      current_id += 1;
  }
  res
}

fn get_workspace_windows(monitor: &str, workspaces_count: Option<i8>) -> Result<Vec<WorkspaceCustom>> {
    // get all workspaces
    let mut workspaces: Vec<_> = Workspaces::get()?.into_iter().collect();
    workspaces.sort_by_key(|w| w.id);


    //get active workspace
    let active_workspace_id: i32;
    if monitor == "ALL" {
        active_workspace_id = Workspace::get_active()?.id;
    } else {
        active_workspace_id = Monitors::get()?
            .into_iter()
            .find(|m| m.name == monitor)
            .ok_or_else(|| {
                log::error!("No monitor found with name: {}", monitor);
                HyprError::NotOkDispatch("No monitor found".to_string())
            })?
            .active_workspace
            .id;
    }
    //active monitor name
    let active_monitor_name: String = Monitors::get()?
        .into_iter()
        .find(|m| m.focused == true)
        .ok_or_else(|| {
            log::error!("No active monitor found.");
            HyprError::NotOkDispatch("No active monitor found".to_string())
        })?
        .name;
    let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();
      
    for workspace in workspaces
    .iter()
    .filter(|m| m.monitor == monitor || monitor == "ALL" || monitor == "_")
    {
        let mut active = false;
        let mut class = format!("workspace-button w{}", workspace.id);
        if active_workspace_id == workspace.id
            && (active_monitor_name == monitor || monitor == "ALL")
        {
            class = format!("{} workspace-active wa{}", class, workspace.id);
            active = true;
        }

        let ws: WorkspaceCustom = WorkspaceCustom {
            name: workspace.name.clone(),
            id: workspace.id,
            occupied: true,
            active,
            class,
        };
        out_workspaces.push(ws);
    }

    out_workspaces.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());

    if let Some(wc) = workspaces_count {
        out_workspaces = fill_empty(out_workspaces, wc * Monitors::get()?.into_iter().len() as i8);

        let monitors: Monitors = Monitors::get()?;    
        let mon_index = monitors.iter().rev().position(|m| m.name == monitor).unwrap_or(9999); // Don't know why the monitor ids are reversed vs `hyprctl monitors` but this is the only way this will work
        let mon_count = monitors.iter().len();


        let sec_size = out_workspaces.len() / mon_count;
        let start = mon_index * sec_size;
        let end = if mon_index == mon_count - 1 {
            out_workspaces.len() // Remaining workspaces
        } else {
            start + sec_size
        };

        out_workspaces = out_workspaces[start..end].to_vec();
      
    }

    return Ok(out_workspaces)
}

fn get_all_advanced(workspaces_count: Option<i8>) -> Result<Vec<MonitorCustom>> {
    let monitors = Monitors::get()?;
    let mut out_monitors: Vec<MonitorCustom> = Vec::new();
    for m in monitors {
        let workspaces = get_workspace_windows(&m.name, workspaces_count)?;
        let mc: MonitorCustom = MonitorCustom {
            name: m.name,
            workspaces: workspaces,
        };
        out_monitors.push(mc);
    }
    Ok(out_monitors)
}

fn output(monitor: &str, workspaces_count: Option<i8>) {
    if monitor == "_" {
        println!(
            "{}",
            json!(get_all_advanced(workspaces_count).unwrap_or_else(|err| {
                log::error!("Advanced get failed: {}", err);
                Vec::new()
            }))
            .to_string()
        );
    } else {
        println!(
            "{}",
            json!(get_workspace_windows(monitor, workspaces_count).unwrap_or_else(|err| {
                log::error!("Basic get failed: {}", err);
                Vec::new()
            }))
            .to_string()
        );
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let _logger = match Logger::try_with_str("info") {
        Ok(logger) => {
            match logger
                .log_to_file(
                    FileSpec::default()
                        .directory("/tmp")
                        .basename("hyprland-workspaces"),
                )
                .start()
            {
                Ok(logger) => logger,
                Err(e) => {
                    println!("Unable to start logger: {}", e);
                    std::process::exit(1)
                }
            };
        }
        Err(e) => {
            println!("Unable to initialise logger: {}", e);
            std::process::exit(1)
        }
    };
    let mon = Arc::new(String::from(args.monitor));
    let workspaces_count = Arc::new(args.workspace_count);
    if let None = Monitors::get()
        .unwrap_or_else(|err| {
            log::error!("Unable to get monitors: {}", err);
            std::process::exit(1)
        })
        .into_iter()
        .find(|m| m.name == mon.to_string() || mon.to_string() == "ALL" || mon.to_string() == "_")
    {
        log::error!("Unable to find monitor {mon}");
        std::process::exit(0);
    }

    log::info!("Started with arg {}", mon);
    output(&mon, *workspaces_count);
    // Create a event listener
    let mut event_listener = EventListener::new();
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_workspace_change_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_workspace_added_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_workspace_destroy_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_workspace_moved_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_monitor_added_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_monitor_removed_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_window_close_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_window_open_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_active_monitor_change_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_active_window_change_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_window_close_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_fullscreen_state_change_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_window_moved_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_layer_open_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_layer_closed_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let mon_clone = Arc::clone(&mon);
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    event_listener.add_urgent_state_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });
    let workspaces_count_clone = Arc::clone(&workspaces_count);
    let mon_clone = Arc::clone(&mon);
    event_listener.add_window_title_change_handler(move |_| {
        output(&mon_clone, *workspaces_count_clone);
    });

    event_listener.start_listener()
}
