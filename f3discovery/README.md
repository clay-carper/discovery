# Getting Started

Throughout this process, I will be using Ubuntu, both natively and on WSL2.

## WSL2
Use [this](https://www.xda-developers.com/wsl-connect-usb-devices-windows-11/) guide to initially set up.
Before trying to interface with the STM32F3 discovery kit, it's necessary to pass the device through WSL. This can be done by doing the following in an Administrator Command Prompt.
```
usbipd wsl list
usbipd wsl attach --busid `id`
```
Afterward, the user should be prompted for the WSL distro's password. To check if the passthrough worked, run `lsusb` in the distro's shell.  
If this fails, run `wsl --shutdown`. If issues persist, try updating WSL with `wsl update`, restart the entire machine, and google like mad.  
If you get a `the target distro is not running` error, set your default distro to the desired on with `wsl --setdefault distro_name`  
If `udevadm control --reload-rules` throws an error, try running `/lib/systemd/systemd-udevd --daemon`.  
If the permissions are wrong for the STM32F3 after refreshing the udev rules, disconnect and reconnect the controller.  
The command for gdb on my WSL2-Ubuntu machine is `gdb-multiarch -q -ex "target remote :3333" target/thumbv7em-none-eabihf/debug/led-roulette`