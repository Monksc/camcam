# CamCam
Given straight lines that form shapes.  The library will create gcode that will cut out those shapes.
This is a cam software to generate gcode for a cnc router.

# Usage

Currenlty its just a libary just add in this line to your Cargo.toml. You can remove the rev if you want a newer version.
```
[dependencies]
camcam = {  git = "https://github.com/Monksc/camcam", rev = "f01246ec0e166fe6178d1535984c427547c27d22"}
```

We have an example of how to use the libary in main.rs. 

Eventually if somone wants a command line interface and have it accept a certain file like a .dxf file then we may.
Just ask with an Issue.
To get all the tool path settings we are looking at using a .toml file.


# Results

You can view some of the examples produced with this on 
[cncsim](https://github.com/Monksc/cncsim) .

# Software to view the gcode

You can use https://ncviewer.com/ CNCSimPro and LinuxCNC-sim for testing

Another great option to view the end product is 
[cncsim](https://github.com/Monksc/cncsim) .

# Contributing

We would love to have people contribute. If you want to send a pull request then I will view it.
A less effort way is putting in an issue with a solution and Ill add it later.
