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
Procedural trees (Weber and Penn's parametric model). From left to right: palm, weeping willow
(with two Lombardy poplars mostly hidden behind it), quaking aspens, balsam firs, California
black oak and black tupelo, with shrubs in front. Each is
`{"type": "tree", "species": "...", "seed": 0, "location": [x, y, z], "bark": material, "leaves": material}`,
with optional `"params"` overriding the species' parameters (see `src/procedural/tree.rs`).
`"leaves"` may list several materials, each leaf taking one at random. The leaves here use the
translucent `leaf` material, so they glow where the sun, behind and to the right, shines
through them, and the bark is textured with noise.
![trees](./scenes/trees.png)

Evergreens, from left to right: Italian cypresses, a forked stone (umbrella) pine, Scots pines,
Norway spruces, a cedar of Lebanon and balsam firs. Trees of the same species differ only by
their seed.
![evergreens](./scenes/evergreens.png)

Autumn: each tree's `"autumn"` parameter says how far through the season it is, and its `leaf`
material gives autumn colours for shaded and sunlit leaves (`"autumn"`, `"autumn_sun"`) and a
dead colour (`"dead"`). Exposed leaves at the top and outside of a crown turn and fall first.
From left to right: a Lombardy poplar and a dead tree (`"dead_branches": 1`), a weeping willow
just turning, quaking aspens, evergreens, a California black oak with some dead limbs, a black
tupelo (scarlet in the sun, orange in shade) and, far right, an oak keeping its dead leaves
(`"leaf_fall": 0.35`); red shrubs in front.
![autumn](./scenes/autumn.png)

## Instancing
Twenty instances of one teapot mesh, each with its own transform and material.
![instances](./scenes/instances.png)

## Fireworks
![fireworks](./scenes/fireworks.png)

