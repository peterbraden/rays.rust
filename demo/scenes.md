# Test Scenes

## Spheres
Spheres demonstrating 3 different materials.
![Spheres](./scenes/spheres.png)

## Ocean scenes
Demonstrating the ocean generation.
![Ocean 1](./scenes/ocean-1.png)
![Ocean 2](./scenes/ocean-2.png)
![Ocean 3](./scenes/spheres-on-water.png)
![Ocean 4](./scenes/ocean-boat.png)

## Block Terrain
Procedural art with a block city skyline at sunset.
![block terrain](./scenes/block-terrain.png)

## Trees
Procedural trees of every built-in species (Weber and Penn's parametric model): palm, weeping
willow, quaking aspen, black tupelo, balsam fir, Lombardy poplar, California black oak and
shrubs. Each is
`{"type": "tree", "species": "...", "seed": 0, "location": [x, y, z], "bark": material, "leaves": material}`,
with optional `"params"` overriding the species' parameters (see `src/procedural/tree.rs`).
![trees](./scenes/trees.png)

## Fireworks
![fireworks](./scenes/fireworks.png)

