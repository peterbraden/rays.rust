"""Render the Stanford dragon reference image with Mitsuba 3.

The scene matches demo/reference/stanford-dragon.json: the same PLY and ground OBJ, transform,
camera, materials and constant white environment. Flat shading (face_normals) and two-sided
diffuse match our Mesh and Lambertian; a box filter matches our default.

    pip install mitsuba==3.9.1
    python scripts/mitsuba_dragon_reference.py references/stanford-dragon/dragon_vrip.ply \
        demo/reference/stanford-dragon/ground.obj out.exr [samples per pixel]
"""
import sys

import mitsuba as mi

mi.set_variant("scalar_rgb")
T = mi.ScalarTransform4f

dragon, ground, out = sys.argv[1:4]
spp = int(sys.argv[4]) if len(sys.argv) > 4 else 1024


def diffuse(albedo):
    return {"type": "twosided", "bsdf": {"type": "diffuse", "reflectance": {"type": "rgb", "value": albedo}}}


scene = mi.load_dict({
    "type": "scene",
    # Our max_depth of 64 bounces is 65 path segments in Mitsuba's counting.
    "integrator": {"type": "path", "max_depth": 65, "rr_depth": 5},
    "sensor": {
        "type": "perspective",
        # Our angle 0.491952 = atan(2 tan(15 degrees)): a 30 degree vertical field of view.
        "fov": 30, "fov_axis": "y",
        "to_world": T().look_at(origin=[0, 6, 24], target=[0, 3.2, 0], up=[0, 1, 0]),
        "film": {"type": "hdrfilm", "width": 640, "height": 480, "rfilter": {"type": "box"}},
        "sampler": {"type": "independent", "sample_count": spp},
    },
    "sky": {"type": "constant", "radiance": {"type": "rgb", "value": [1, 1, 1]}},
    "dragon": {
        "type": "ply", "filename": dragon, "face_normals": True,
        "to_world": T().translate([0.29, -2.636, 0.23]) @ T().scale(50),
        "bsdf": diffuse([0.8, 0.8, 0.8]),
    },
    "ground": {"type": "obj", "filename": ground, "face_normals": True, "bsdf": diffuse([0.5, 0.5, 0.5])},
})
mi.Bitmap(mi.render(scene)).write(out)
