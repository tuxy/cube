![](./showcase.gif)
*please excuse the video quality, gnome's screen recorder is pretty bad...*

# Cubes 
**(Technically balls not very well simulated in rust)**
This is a fun project of mine, inspired by [Vercidium](https://www.youtube.com/@Vercidium) for optimizing 3d libraries as well as [Winderdev](https://www.youtube.com/@Winterdev) to build my first physics simulation with collision and gravity using raylib-rs. While the math (and programming) behind this learning project is not great and could be improved, I am somewhat proud of what I have done. 

I am planning to use GLFW and a real, optimized collision library to build me next project...

So here are the features! :
- Key to start physics (z by default)
- Gravity
- Stationary objects configurable
- Vector of objects that can be easily manipulated (customised)
- Sphere -> Sphere collision
- Sphere -> (flat) plane collision

All credits are given within the code for any math or code that I have taken/referenced

## Issues
While I am decently proud of what I have done (considering I have never done this before), there are still a few issues to point out:
 - When rested (No velocity), spheres just oscillate up and down. I have tried smoothing it out but there is still some left
 - Sinking may still be an issue, as a vec3 velocity of zero still results in some movement
 - For every object, every other object is checked for collision, even if they are very far apart. I might plan to implement bounding boxes for faster AABB collision.
 - Rotation is NOT accounted for, and so the spheres do not rotate. No angular momentum

## Build
There is a shell.nix file that can be used with `nix-shell` and contains the dependencies and what is needed, but mainly is just rust, cmake and libGL:

```cargo run```

# Why?
Would I recommend it for creating your own engine? Probably not, there are great libraries out there that does this...

Would I recommend it for learning concepts for math and physics? 100%, was really fun. 