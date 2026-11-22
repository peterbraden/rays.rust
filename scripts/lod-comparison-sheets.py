import sys, importlib.util, numpy as np
from PIL import Image, ImageDraw, ImageFont
spec=importlib.util.spec_from_file_location('c','/home/user/rays.rust/scripts/compare-linear.py'); c=importlib.util.module_from_spec(spec); spec.loader.exec_module(c)
R='/tmp/claude-0/-home-user-rays-rust/68517cbd-7562-597b-b10a-4dd73f225831/scratchpad/renders/'
OUT='/home/user/rays.rust/docs/research/learned-lod/images/'
def srgb(x):
    x=np.clip(x,0,1); return np.where(x<=0.0031308,12.92*x,1.055*x**(1/2.4)-0.055)
def load(n): return c.read(R+n+'.exr')[...,:3]
try: font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',15)
except: font=ImageFont.load_default()
def label(img,text):
    d=ImageDraw.Draw(img); d.rectangle([0,0,img.width,22],fill=(0,0,0)); d.text((6,3),text,fill=(255,255,255),font=font); return img
def tile(rgb,scale):  # display with exposure scale
    return Image.fromarray((srgb(rgb*scale)*255).astype(np.uint8))
def ratio_map(t,r,mask):
    lt,lr=c.luminance(t),c.luminance(r)
    # blur 5x5 box to show bias rather than noise
    def blur(a,k=6):
        from numpy.lib.stride_tricks import sliding_window_view
        p=np.pad(a,k,mode='edge'); w=sliding_window_view(p,(2*k+1,2*k+1)); return w.mean(axis=(-1,-2))
    top=np.quantile(lr,0.999); lt,lr=np.minimum(lt,top),np.minimum(lr,top)
    q=blur(lt)/np.maximum(blur(lr),1e-6)-1
    q=np.clip(q/0.15,-1,1)  # ±15%
    rgbm=np.zeros(q.shape+(3,))
    rgbm[...,0]=np.where(q>0,1,1+q); rgbm[...,2]=np.where(q<0,1,1-q); rgbm[...,1]=1-np.abs(q)
    rgbm[~mask]=0.5
    return Image.fromarray((rgbm*255).astype(np.uint8))
def sheet(name, ref, rows, depth, scale, crop=None):
    r=load(ref); d=c.read(R+depth+'.exr'); d=d[...,0] if d.ndim==3 else d; mask=np.isfinite(d)&(d<1e6)
    tiles=[]
    for n,lab in rows:
        t=load(n)
        a=tile(t,scale); b=ratio_map(t,r,mask) if n!=ref else Image.new('RGB',a.size,(128,128,128))
        if crop: a=a.crop(crop).resize((a.width, int(a.width*(crop[3]-crop[1])/(crop[2]-crop[0]))),Image.NEAREST); b=b.crop(crop).resize(a.size,Image.NEAREST)
        tiles.append((label(a,lab),label(b,'brightness vs full detail (red brighter, blue darker, ±15%)' if n!=ref else 'reference')))
    W,H=tiles[0][0].size
    out=Image.new('RGB',(2*W+6,len(tiles)*(H+6)),(255,255,255))
    for i,(a,b) in enumerate(tiles):
        out.paste(a,(0,i*(H+6))); out.paste(b,(W+6,i*(H+6)))
    out.save(OUT+name); print(OUT+name, out.size)
sheet('aerial.png','full64',[('full64','Full detail (24.7 G triangles placed)'),('lod64-old','Old cards: FLIP 0.0585, +4.6%'),('lod64-new','This PR: FLIP 0.0543, -1.2%'),('lod64-vs600','Volumes from L5 (prototype): FLIP 0.0542, -2.5%')],'full-depth',2.2)
sheet('aerial-crop.png','full64',[('full64','Full detail'),('lod64-old','Old cards'),('lod64-new','This PR'),('lod64-vs600','Volumes from L5')],'full-depth',2.2,crop=(150,180,450,330))
sheet('low.png','low-full',[('low-full','Full detail, from 90 m'),('low-old','Old cards: FLIP 0.0313'),('low-new','This PR: FLIP 0.0296'),('low-vs100','Volumes from L4 (prototype): FLIP 0.0282')],'low-depth',1.6)
sheet('crossed.png','full64',[('full64','Full detail'),('lod64-new','This PR (one card): FLIP 0.0543'),('lod64-cross','Crossed cards only: FLIP 0.1001, -13.5%'),('lod64-tc','Crossed cards + measured self-shadowing: FLIP 0.0454')],'full-depth',2.2)
