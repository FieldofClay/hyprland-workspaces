use flexi_logger::{FileSpec, Logger};
use log;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const REFRESH_EVENTS: &[&str] = &[
    "workspace",
    "workspacev2",
    "createworkspace",
    "createworkspacev2",
    "destroyworkspace",
    "destroyworkspacev2",
    "moveworkspace",
    "moveworkspacev2",
    "renameworkspace",
    "monitoradded",
    "monitoraddedv2",
    "monitorremoved",
    "monitorremovedv2",
    "focusedmon",
    "focusedmonv2",
    "openwindow",
    "closewindow",
    "movewindow",
    "movewindowv2",
    "activewindow",
    "activewindowv2",
    "fullscreen",
    "openlayer",
    "closelayer",
    "urgent",
    "windowtitle",
    "windowtitlev2",
];

const HELP: &str = "\
hyprland-workspaces: a multi monitor aware hyprland workspaces json widget generator for eww.

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
    pub on_screen: bool,
    pub class: String,
}

#[derive(Serialize)]
struct MonitorCustom {
    pub name: String,
    pub workspaces: Vec<WorkspaceCustom>,
}

#[derive(Deserialize)]
struct Workspace {
    id: i32,
    name: String,
    monitor: String,
}

#[derive(Deserialize)]
struct WorkspaceRef {
    id: i32,
}

#[derive(Deserialize)]
struct Monitor {
    name: String,
    focused: bool,
    #[serde(rename = "activeWorkspace")]
    active_workspace: WorkspaceRef,
}

fn socket_path(socket: &str) -> Result<PathBuf> {
    let sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not set, is Hyprland running?")?;
    if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(runtime_dir).join("hypr").join(&sig).join(socket);
        if path.exists() {
            return Ok(path);
        }
    }
    Ok(PathBuf::from("/tmp/hypr").join(sig).join(socket))
}

fn query<T: DeserializeOwned>(cmd: &str) -> Result<T> {
    let mut stream = UnixStream::connect(socket_path(".socket.sock")?)?;
    stream.write_all(format!("j/{cmd}").as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(serde_json::from_str(&response)?)
}

fn get_workspace_windows(monitor: &str) -> Result<Vec<WorkspaceCustom>> {
    // get all workspaces
    let mut workspaces: Vec<Workspace> = query("workspaces")?;
    workspaces.sort_by_key(|w| w.id);
    let monitors: Vec<Monitor> = query("monitors")?;

    //get active workspace
    let active_workspace_id: i32;
    if monitor == "ALL" {
        active_workspace_id = query::<WorkspaceRef>("activeworkspace")?.id;
    } else {
        active_workspace_id = monitors
            .iter()
            .find(|m| m.name == monitor)
            .ok_or_else(|| {
                log::error!("No monitor found with name: {}", monitor);
                "No monitor found"
            })?
            .active_workspace
            .id;
    }
    //active monitor name
    let active_monitor_name = &monitors
        .iter()
        .find(|m| m.focused)
        .ok_or_else(|| {
            log::error!("No active monitor found.");
            "No active monitor found"
        })?
        .name;
    let on_screen_workspaces: Vec<i32> = monitors
        .iter()
        .map(|m| m.active_workspace.id).collect();
    let mut out_workspaces: Vec<WorkspaceCustom> = Vec::new();

    for workspace in workspaces
        .iter()
        .filter(|m| m.monitor == monitor || monitor == "ALL" || monitor == "_")
    {
        let mut active = false;
        let on_screen = on_screen_workspaces.contains(&workspace.id);
        let mut class = format!("workspace-button w{}{}", workspace.id, if on_screen {" workspace-on-screen"} else {""});
        if (active_workspace_id == workspace.id)
            && (active_monitor_name == monitor || monitor == "ALL")
        {
            class = format!("{} workspace-active wa{}", class, workspace.id);
            active = true;
        }

        let ws: WorkspaceCustom = WorkspaceCustom {
            name: workspace.name.clone(),
            id: workspace.id,
            active,
            on_screen,
            class,
        };
        out_workspaces.push(ws);
    }
    return Ok(out_workspaces);
}

fn get_all_advanced() -> Result<Vec<MonitorCustom>> {
    let monitors: Vec<Monitor> = query("monitors")?;
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

fn output(monitor: &str, last: &mut String) {
    let current = if monitor == "_" {
        json!(get_all_advanced().unwrap_or_else(|err| {
            log::error!("Advanced get failed: {}", err);
            Vec::new()
        }))
        .to_string()
    } else {
        json!(get_workspace_windows(monitor).unwrap_or_else(|err| {
            log::error!("Basic get failed: {}", err);
            Vec::new()
        }))
        .to_string()
    };
    if current != *last {
        println!("{current}");
        *last = current;
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    //check args
    if args.len() != 2 || args[1].eq("-h") || args[1].eq("--help") {
        println!("{HELP}");
        std::process::exit(0);
    }
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
    let mon = args[1].to_string();
    let monitors: Vec<Monitor> = query("monitors").unwrap_or_else(|err| {
        log::error!("Unable to get monitors: {}", err);
        std::process::exit(1)
    });
    if let None = monitors
        .iter()
        .find(|m| m.name == mon || mon == "ALL" || mon == "_")
    {
        log::error!("Unable to find monitor {mon}");
        std::process::exit(0);
    }

    log::info!("Started with arg {}", mon);
    let mut last = String::new();
    output(&mon, &mut last);
    // Create a event listener
    let events = BufReader::new(UnixStream::connect(socket_path(".socket2.sock")?)?);
    for line in events.lines() {
        let line = line?;
        let event = line.split_once(">>").map_or(line.as_str(), |(name, _)| name);
        if REFRESH_EVENTS.contains(&event) {
            output(&mon, &mut last);
        }
    }
    Err("Hyprland event socket closed".into())
}
