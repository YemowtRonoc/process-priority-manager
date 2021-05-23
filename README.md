# Process Priority Manager
This software will set the priority for certain processes on your system automatically. The current version will read the process list and priorities from a configuration file. In this case, the configuration file is called ```process_config.json```. 

A sample content for this file is below: 
```
{
    "processes": [
        {
            "process_name": "OVRServer_x64.exe",
            "process_priority": "high"
        },
        {
            "process_name": "explorer.exe",
            "process_priority": "AboVe NormAL"
        }
    ]
}
```

In this case, the software will set the priority for the two processes to the given priorities. The priority level is case-insensitive, and it is based on the priority list from [this Windows API](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setpriorityclass).

## Priority Levels
The priority levels for processes, according to the MSDN documentation, are as follows:

| process_priority                                                                                       | Priority Meaning                                                                                                                               |
| ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| REALTIME [(WARNING: Not Recommended)](https://devblogs.microsoft.com/oldnewthing/20100610-00/?p=13753) | Highest possible process priority. This will cause the process to preempt all other processes on the system.                                   |
| HIGH                                                                                                   | Process must preempt all other processes with normal/above normal priorities. This may use nearly all available CPU time.                      |
| ABOVE NORMAL                                                                                           | Above Normal but below high priority.                                                                                                          |
| NORMAL                                                                                                 | Normal process with no special scheduling needs.                                                                                               |
| BELOW NORMAL                                                                                           | Below Normal priority, but above idle priority.                                                                                                |
| IDLE                                                                                                   | Only run when the system is idle.                                                                                                              |
| BACKGROUND BEGIN                                                                                       | The system will lower the scheduling priorities of the process so it can perform background work without effecting anything in the foreground. |
| BACKGROUND END                                                                                         | System will restore the processing priority to before the background begin setting.                                                            |

The values on the left in the above table will be the possible values for use as ```process_priority``` in the ```process_config.json``` configuration file.

## Build from source
1. Install Rust for windows.
2. clone and unzip. 
3. cd to the project directory (the one that contains `Cargo.tml`)
4. run `cargo build`

## Automation
To run this program automatically, it is recommended to use the in-built Windows Task Scheduler. To open this software, simply search in your start menu for ```Task Scheduler``` and it should appear. You then want to ```Create a Basic Task```. Give it a ```Name``` that you will remember, and a ```Description``` that accurately describes what you want it to do. Choose how frequently you want to run the task (I choose ```When I log on```). We want to ```Start a Program``` on the next dialog. On the next page, you will want to choose the path to the executable for the process manager. Finally, click ```Finish```.

This should now run automatically everytime you log in, or however frequently you have chosen to run the software. If you want to run it once every hour, for example, the option should be available in the task properties. 

## Original Purpose
This software was originally created to change the priority of OVRServer_x64 process on Windows. For some systems, including my own, this change made Oculus Link more stable in use.

The need to manually update the process priority everytime I used the headset became a nuisance, and I wanted an excuse to learn a new language. So, I decided to automate the process priority setting, and try out Rust at the same time. 

## Disclaimer
This software has not been thoroughly tested. I just made it good enough for my own personal use. However, if anyone does have any issues with it, they can let me know, and I will try to get to them on my own time.

If you try to change the priority of system processes, you must run this software as an administrator. Otherwise, the software will fail to change the process priority as it has insufficient permissions.
