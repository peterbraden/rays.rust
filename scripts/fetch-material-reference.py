#!/usr/bin/env python3
"""Fetch a scanned surface to fit a library material to (scripts/fit-material.py).

From ambientCG (`ambientcg:Gravel043`) or Poly Haven (`polyhaven:lichen_rock`), both CC0: the
albedo ("Color" / "Diffuse") map at 1K, and the scan's physical size, into
references/materials/<id>/ (not committed), with a starting spec for a fit:
references/materials/<id>/spec.json, naming the preset to fit (`--preset`) and its size, and
no parameters yet: add those to search (see fit-material.py).

    scripts/fetch-material-reference.py polyhaven:lichen_rock --preset granite_outcrop

Only the albedo map is taken (a JPEG); for ambientCG it comes in a zip, of which only that
image is extracted. A scan without a recorded size (some of ambientCG's) needs `--size`.
"""
import argparse, io, json, os, sys, urllib.request, zipfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def get(url):
    with urllib.request.urlopen(urllib.request.Request(url, headers={"User-Agent": "rays-material-fit"})) as r:
        return r.read()


def ambientcg(asset):
    info = json.loads(get("https://ambientcg.com/api/v2/full_json?id=%s&include=downloadData,dimensionsData" % asset))
    a = info["foundAssets"][0]
    url = next(d["fullDownloadPath"] for d in a["downloadFolders"]["default"]["downloadFiletypeCategories"]["zip"]["downloads"]
               if d["attribute"] == "1K-JPG")
    with zipfile.ZipFile(io.BytesIO(get(url))) as z:
        name = next(n for n in z.namelist() if n.endswith("_Color.jpg") and "/" not in n and ".." not in n)
        image = z.read(name)
    size = (a.get("dimensionX") or 0) / 100.  # centimetres
    return image, size


def polyhaven(asset):
    info = json.loads(get("https://api.polyhaven.com/assets?t=textures"))[asset]
    files = json.loads(get("https://api.polyhaven.com/files/%s" % asset))
    image = get(files["Diffuse"]["1k"]["jpg"]["url"])
    size = (info.get("dimensions") or [0])[0] / 1000.  # millimetres
    return image, size


def main():
    a = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    a.add_argument("source", help="ambientcg:<id> or polyhaven:<id>")
    a.add_argument("--preset", default="", help="the library preset to fit to it")
    a.add_argument("--size", type=float, default=0., help="the scan's width in metres, if the source doesn't say")
    args = a.parse_args()
    site, _, asset = args.source.partition(":")
    if not asset.replace("_", "").replace("-", "").isalnum():
        sys.exit("an asset's id is letters, digits, - and _")
    image, size = {"ambientcg": ambientcg, "polyhaven": polyhaven}[site](asset)
    size = args.size or size
    if size <= 0:
        sys.exit("%s doesn't record its size: give --size in metres" % args.source)
    out = os.path.join(ROOT, "references", "materials", asset)
    os.makedirs(out, exist_ok=True)
    open(os.path.join(out, "albedo.jpg"), "wb").write(image)
    spec = {"preset": args.preset, "name": asset.lower(), "reference": "albedo.jpg", "size": size,
            "source": args.source, "licence": "CC0", "params": []}
    path = os.path.join(out, "spec.json")
    if not os.path.exists(path):
        json.dump(spec, open(path, "w"), indent=1)
    print("%s: %.2f m across -> %s" % (args.source, size, out))


if __name__ == "__main__":
    main()
