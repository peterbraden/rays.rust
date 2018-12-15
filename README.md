# rays.rust
[![Build
Status](https://travis-ci.org/peterbraden/rays.rust.svg?branch=master)](https://travis-ci.org/peterbraden/rays.rust)

## Gallery

![block terrain](demo/scenes/block-terrain.png)
![spheres](demo/scenes/spheres/spheres.png)


## Current work
![demo image](demo/demo.png)

A raytracer written in rust.

Supports Monte-Carlo global illumination, obj files.

This is the latest in a series of raytracers I've implemented to learn languages
and explore algorithms.

- [Rays (c++)](https://github.com/peterbraden/rays)
- [JS Raytracer](https://github.com/peterbraden/js-raytracer)

## Install / Run
```
cargo run --release -- -p demo/demo.json
```

## References
- [An Efficient Parametric Algorithm for Octree Traversal, J. Revelles, C. Urena, M. Lastra](http://wscg.zcu.cz/wscg2000/Papers_2000/X31.pdf)
- [Simulating Ocean Water, J. Tessendorf](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.161.9102&rep=rep1&type=pdf)
- [Physically Based Raytracing, M. Pharr, W. Jakob, G. Humphreys](http://www.pbr-book.org/)
