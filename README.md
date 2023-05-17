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
### Building from source
```
git clone https://github.com/FieldofClay/hyprland-workspaces.git
cd hyprland-workspaces
cargo build --release
```

## Usage
Pass the name of the monitor to follow as the only argument. 
```
./hyprland-workspaces eDP-1
```
If you wish to get all workspaces across all monitors, pass the special argument "_".
```
./hyprland-workspaces _
```
It will then follow that monitor(s) and output the workspaces details in JSON to stdout.
```json
[{"active":false,"class":"workspace-button w1","id":1,"name":"1: "},{"active":false,"class":"workspace-button w2","id":2,"name":"2: "},{"active":true,"class":"workspace-button w4 workspace-active wa4","id":4,"name":"4: "}]
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

The following classes are output, to provide multiple options for theming your workspaces widget.
* `workspace-button`: all workspaces will have this class
* `workspace-active`: only the active workspace will have this class. Will not be present if workspace is active, but focus is on another monitor.
* `w<WORKSPACEID>`: Each workspace will have this class to allow for unique CSS per workspace.
* `wa<WORKSPACEID>`: The active workspace will have this to allow for unique CSS per workspace, when it is active. Like `workspace-active`, this does not appear when the focus is on another monitor.