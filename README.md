# hyprland-workspaces
A multi-monitor aware Hyprland workspace widget. Follows the specified monitor and outputs the currently open workspaces. Designed to be used with [Eww](https://github.com/elkowar/eww), but may function with other bars. Compatible with [hyprland-autoname-workspaces](https://github.com/cyrinux/hyprland-autoname-workspaces).

## Installation Instructions
### Dependencies
[Hyprland](https://github.com/hyprwm/Hyprland)
### Arch Linux
Arch users can install from AUR using your favourite package manager.
```
  pikaur -S hyprland-workspaces
```

### NixOS

NixOS users can use the [unstable channel](https://nixos.wiki/wiki/Nix_channels) to try or install the package.

```
nix-shell -p hyprland-workspaces
```

### Crates.io
It can be installed directly from [crates.io](https://crates.io) with cargo.
```
  cargo install hyprland-workspaces
```
### Building from source
```
git clone https://github.com/FieldofClay/hyprland-workspaces.git
cd hyprland-workspaces
cargo build --release
```

## Usage
### Basic Usage
Pass the name of the monitor to follow as the only argument. 
```
./hyprland-workspaces eDP-1
```
If you wish to get all workspaces across all monitors, pass the special argument "ALL".
```
./hyprland-workspaces ALL
```
It will then follow that monitor(s) and output the workspaces details in JSON to stdout.
```json
[{"active":false,"class":"workspace-button w1","id":1,"name":"1: ","occupied":false},{"active":false,"class":"workspace-button w2","id":2,"name":"2: ","occupied":false},{"active":true,"class":"workspace-button w4 workspace-active wa4","id":4,"name":"4: ","occupied":true}]
```
You can get the names of your monitors by running:
```
hyprctl monitors -j
```

It can be used as a workspaces widget in Eww with config similar to below.
```yuck
(deflisten workspace0 "hyprland-workspaces `hyprctl monitors -j | jq -r \".[0].name\"`")
(deflisten workspace1 "hyprland-workspaces `hyprctl monitors -j | jq -r \".[1].name\"`")

(defwidget workspaces0 []
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces"
      (for i in workspace0
        (button
          :onclick "hyprctl dispatch workspace ${i.id}"
          :class "${i.class}"
          "${i.name}")))))
(defwidget workspaces1 []
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces"
      (for i in workspace1
        (button
          :onclick "hyprctl dispatch workspace ${i.id}"
          :class "${i.class}"
          "${i.name}")))))

(defwindow bar0 []
  :monitor 0
  (box 
    (workspaces0)
    (other_widget)))
(defwindow bar1 []
  :monitor 1
  (box
    (workspaces1)
    (other_widget)))
```
### Advanced Usage
Passing the special argument '_' will output all workspaces, wrapped in an array of monitors. This allows only running a single instance of hyprland-workspaces and simplified eww configuration.
```
./hyprland-workspaces _
```
Each monitor will have a sub array, workspaces, which will be the same information output as `hyprland-workspaces MONITORNAME`.
```json
[
   {
      "name": "eDP-1",
      "workspaces": [
         {"active": false,"class": "workspace-button w6","id": 6,"name": "6 []","occupied":true}
      ]
   },
   {
      "name": "DP-3",
      "workspaces": [
         {"active": false,"class": "workspace-button w1","id": 1,"name": "1 ","occupied":false},
         {"active": true,"class": "workspace-button w3 workspace-active wa3","id": 3,"name": "3 ","occupied":true}
      ]
   },
   {
      "name": "DP-4",
      "workspaces": [
         {"active": false,"class": "workspace-button w2","id": 2,"name": "2 ","occupied":false},
         {"active": false,"class": "workspace-button w5","id": 5,"name": "5 ","occupied":false}
      ]
   }
]
```
This helps avoid repetition within your eww configuration, by using something similar to below.
```yuck
(deflisten workspaces "hyprland-workspaces _")

(defwidget workspaceWidget [monitor]
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces"
      (for i in {workspaces[monitor].workspaces}
        (button
          :onclick "hyprctl dispatch workspace ${i.id}"
          :class "${i.class}"
          "${i.name}")))))

(defwidget bar0 []
  (box
    (workspaceWidget :monitor 0)
  )
)

(defwidget bar1 []
  (box
    (workspaceWidget :monitor 1)
  )
)
```

### Occupied
`occupied` will be `true` if there are any windows on that particular workspace. You can use this to display empty and occupied workspaces in a different way.

### Classes
The following classes are output, to provide multiple options for theming your workspaces widget.
* `workspace-button`: all workspaces will have this class
* `workspace-active`: only the active workspace will have this class. Will not be present if workspace is active, but focus is on another monitor.
* `w<WORKSPACEID>`: Each workspace will have this class to allow for unique CSS per workspace.
* `wa<WORKSPACEID>`: The active workspace will have this to allow for unique CSS per workspace, when it is active. Like `workspace-active`, this does not appear when the focus is on another monitor.
