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
Pass the name of the monitor to follow as the only argument. It will then follow that monitor and output the workspaces details in JSON stdout.
```
./hyprland-workspaces eDP-1
```
You can get the names of your monitors by running:
```
hyprctl monitors -j
```

It can be used as a title widget in Eww with config similar to below.
```yuck
(deflisten workspace0 "hyprland-workspaces `hyprctl monitors -j | jq -r \".[0].name\"`")

(defwidget workspaces0 []
  (eventbox :onscroll "hyprctl dispatch workspace `echo {} | sed 's/up/+/\' | sed 's/down/-/'`1"
    (box :class "workspaces"
      (for i in workspace0
        (button
          :onclick "hyprctl dispatch workspace ${i.id}"
          :class "${i.class}"
          "${replace(i.name, ':', '')}")))))
```

The following classes are output, to provide multiple options for theming your workspaces widget.
* `workspace-button`: all workspaces will have this class
* `workspace-active`: only the active workspace will have this class. Will not be present if workspace is active, but focus is on another monitor.
* `w<WORKSTATIONID>`: Each workspace will have this class to allow for unique CSS per workspace.
* `wa<WORKSTATIONID>`: The active workspace will have this to allow for unique CSS per workspace, when it is active. Like `workspace-active`, this does not appear when the focus is on another monitor.