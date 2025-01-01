# Cubes 
**(Technically balls not very well simulated in rust)**
This is a fun project of mine, inspired by [Vercidium](https://www.youtube.com/@Vercidium) for optimizing 3d libraries as well as [Winderdev](https://www.youtube.com/@Winterdev) to build my first physics simulation with collision and gravity using raylib-rs. While the math (and programming) behind this learning project is not great and could be improved, I am somewhat proud of what I have done. 

So here are the features! :
- Key to start physics
- Gravity
- Stationary objects configurable
- Vector of objects that can be easily manipulated (customised)
- Sphere -> Sphere collision
- Sphere -> (flat) plane collision (Not good...)

All credits are given within the code for any math or code that I have taken/referenced

## Build
There is a shell.nix file that can be used with `nix-shell` and contains the dependencies and what is needed, but mainly is just rust, cmake and libGL:

```cargo run```

# Why?
Would I recommend it for creating your own engine? Probably not, there are great libraries out there that does this...

Would I recommend it for learning concepts for math and physics? 100%, was really fun. 