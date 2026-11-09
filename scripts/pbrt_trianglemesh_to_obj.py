#!/usr/bin/env python3
"""Convert the triangle meshes in a pbrt-v4 scene into one OBJ per named material.

Used to derive demo/reference/cornell-box/*.obj from Benedikt Bitterli's pbrt-v4 Cornell box
(CC0, https://benedikt-bitterli.me/resources/). Only what that scene uses is handled: each
`Shape "trianglemesh"` with "point3 P" and "integer indices", under the most recent
`NamedMaterial`. Vertex order is kept, so face winding (and so which side a one-sided light
emits from) matches pbrt.

Usage: pbrt_trianglemesh_to_obj.py scene-v4.pbrt OUTPUT_DIR
"""
import os
import re
import sys


def numbers(block, name):
    m = re.search(r'"' + re.escape(name) + r'"\s*\[([^\]]*)\]', block)
    return [float(x) for x in m.group(1).split()] if m else None


def main(path, out_dir):
    text = open(path).read()
    material = None
    meshes = {}
    # Split into directives; a mesh's parameters run until the next directive.
    for directive in re.split(r'\n(?=\s*[A-Z][A-Za-z]+\b)', text):
        d = directive.strip()
        if d.startswith('NamedMaterial'):
            material = re.search(r'"([^"]+)"', d).group(1)
        elif d.startswith('Shape "trianglemesh"'):
            p = numbers(d, 'point3 P')
            idx = [int(i) for i in numbers(d, 'integer indices')]
            meshes.setdefault(material, []).append((p, idx))
    os.makedirs(out_dir, exist_ok=True)
    for material, parts in meshes.items():
        lines = [f'# {material}, converted from {os.path.basename(path)} by {os.path.basename(__file__)}']
        offset = 1
        for p, idx in parts:
            for i in range(0, len(p), 3):
                lines.append('v %s %s %s' % tuple(repr(v) for v in p[i:i + 3]))
            for i in range(0, len(idx), 3):
                lines.append('f %d %d %d' % tuple(offset + j for j in idx[i:i + 3]))
            offset += len(p) // 3
        name = re.sub(r'(?<!^)(?=[A-Z])', '-', material).lower()
        open(os.path.join(out_dir, name + '.obj'), 'w').write('\n'.join(lines) + '\n')
        print(name + '.obj', sum(len(i) // 3 for _, i in parts), 'triangles')


if __name__ == '__main__':
    main(sys.argv[1], sys.argv[2])
