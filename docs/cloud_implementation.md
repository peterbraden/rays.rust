# Cloud Implementation for Rays.rust

This document outlines the implementation of volumetric clouds in the sky rendering in Rays.rust.

## Overview

We have implemented realistic cloud rendering using a participating media approach. Clouds exist as a volumetric layer in the atmosphere that scatters and absorbs light, creating a more realistic and dynamic sky.

## Components

### 1. Noise Module (`src/noise.rs`)
- Implemented 3D Perlin noise for cloud shape generation
- Added fractal Brownian motion (fBm) for multi-octave detail
- Implemented Worley noise (cellular noise) for cloud detail
- Combined noise functions for natural-looking cloud patterns
- Includes visualization tests for easy verification

### 2. Cloud Participating Medium (`src/participatingmedia.rs`)
- Created `CloudLayer` struct implementing `ParticipatingMedium`
- Handles scattering and absorption of light through clouds 
- Uses a simplified Henyey-Greenstein phase function for anisotropic scattering
- Implements variable density based on noise evaluation and altitude

### 3. Cloud Geometry
- Defines bounds for the cloud layer with parameters for base height, thickness, and horizontal extent
- Implements ray marching through the volume with density-based sampling
- Uses adaptive step sizes based on density for better performance
- Probabilistic density sampling for natural cloud edges

### 4. Sky Integration
- Clouds blend naturally with the existing sky renderer
- Light transport between clouds and atmosphere creates realistic sunrise/sunset effects
- Anisotropic scattering parameter to control cloud appearance (forward vs. isotropic scattering)

### 5. Scene Configuration
- Added JSON configuration options for creating cloud layers
- Supports full customization of all cloud parameters
- Easy to create different types of cloud formations

## Usage

To add clouds to a scene, add a "clouds" object to your scene JSON file:

```json
{
  "type": "clouds",
  "base_height": 700,
  "thickness": 300,
  "density": 0.5,
  "noise_scale": 0.002,
  "height_falloff": 0.15,
  "anisotropy": 0.3,
  "color": [1.0, 1.0, 1.0],
  "extent": 5000.0,
  "worley_density": 2.0,
  "seed": 42
}
```

## Parameters

| Parameter | Description | Default Value |
|-----------|-------------|---------------|
| `base_height` | Cloud layer bottom height | 800.0 |
| `thickness` | Cloud layer vertical thickness | 400.0 |
| `density` | Maximum cloud density factor | 0.6 |
| `noise_scale` | Scale factor for noise patterns | 0.001 |
| `height_falloff` | Controls density decrease with height | 0.1 |
| `anisotropy` | Forward scattering coefficient (higher = more directional) | 0.2 |
| `color` | Base color of clouds | [1.0, 1.0, 1.0] |
| `extent` | Horizontal extent of cloud layer | 10000.0 |
| `worley_density` | Density of cellular noise features | 1.0 |
| `seed` | Random seed for noise generation | 42 |

## Example Scenes

Two example scenes have been provided:
1. `demo/scenes/sky-clouds.json` - Earth-like cloud layer with blue sky
2. `demo/scenes/sky-sunset-clouds.json` - Golden-tinted clouds during sunset

## Technical Implementation

### Ray Marching Algorithm

The ray marching algorithm works as follows:
1. Check if ray intersects the cloud layer bounding box
2. March along the ray with adaptive step sizes
3. At each step, evaluate cloud density using combined noise patterns
4. Apply probability-based hit detection based on density
5. Return intersection point and normal for the material system

### Cloud Density Calculation

Cloud density is determined by:
1. PerlinNoise-based fBm for large-scale cloud shapes
2. WorleyNoise for detailed cellular structures
3. Height-based falloff for realistic vertical profile 
4. Combined with vertical profile curve (more dense in middle, less at edges)

### Performance Considerations

- Adaptive step sizes during ray marching (larger steps in low-density regions)
- Probabilistic sampling to reduce unnecessary calculations
- Clear bounding box for early ray termination
- Class-based approach for efficient noise calculations

## Future Improvements

1. Multiple cloud layers/types (cumulus, stratus, cirrus)
2. Self-shadowing between clouds
3. Animation support with wind direction and speed
4. Additional presets for common weather patterns
5. Optimizations for faster rendering

## References

- [Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite](https://media.contentapi.ea.com/content/dam/eacom/frostbite/files/s2016-pbs-frostbite-sky-clouds-new.pdf)
- [Real-time Volumetric Cloudscapes of Horizon: Zero Dawn](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf)
- [Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine](https://www.guerrilla-games.com/read/nubis-authoring-real-time-volumetric-cloudscapes-with-the-decima-engine)