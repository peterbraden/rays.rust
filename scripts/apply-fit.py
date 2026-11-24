#!/usr/bin/env python3
"""Copy a recorded fit into its preset, once it has been looked at and judged better.

    scripts/apply-fit.py scree [--reference 0]

The fit (written by scripts/fit-material.py --record into the preset's `references`) holds its
fitted values by parameter: a path into the preset (its parts joined with dots), a preset
variable by name, or `scale`, which becomes the preset's own scale (the size it was made at,
multiplied into whatever a scene asks for). Marks the fit applied. A later fit starts from the
applied values.
"""
import argparse, json, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
LIBRARY = os.path.join(ROOT, "src/material/library.json")


def main():
    a = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    a.add_argument("preset")
    a.add_argument("--reference", type=int, default=0)
    args = a.parse_args()
    library = json.load(open(LIBRARY))
    preset = library[args.preset]
    ref = preset["references"][args.reference]
    fit = ref.get("fit") or sys.exit("%s's reference %d hasn't been fitted" % (args.preset, args.reference))
    if fit.get("applied"):
        sys.exit("already applied")
    for key, value in fit["values"].items():
        if key == "scale":
            preset["scale"] = round(preset.get("scale", 1.) * value, 4)
        elif "." not in key:
            preset["variables"][key] = value
        else:
            parts = [int(k) if k.isdigit() else k for k in key.split(".")]
            at = preset
            for k in parts[:-1]:
                at = at[k]
            at[parts[-1]] = value
    fit["applied"] = True
    # A fit's usage asked for a scale besides; the preset now carries it.
    ref.get("usage", {}).pop("scale", None)
    json.dump(library, open(LIBRARY, "w"), indent=2)
    print("applied %s's fit (loss %s -> %s)" % (args.preset, fit["loss"]["start"], fit["loss"]["fitted"]))


if __name__ == "__main__":
    main()
