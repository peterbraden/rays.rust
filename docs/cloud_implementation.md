# Cloud Implementation for Rays.rust

This document outlines the implementation plan for adding volumetric clouds to the sky rendering in Rays.rust.

## Overview

We aim to implement realistic cloud rendering using a participating media approach. Clouds will exist as a volumetric layer in the atmosphere that scatters and absorbs light.

## Components

### 1. Noise Module
- Implement 3D Perlin/Simplex noise for cloud shape generation
- Include fractal Brownian motion (fBm) for multi-octave detail
- Support for domain warping to create more natural cloud shapes

### 2. Cloud Participating Medium
- Create `CloudLayer` struct implementing `ParticipatingMedium`
- Handle scattering and absorption of light through clouds
- Use Henyey-Greenstein phase function for anisotropic scattering
- Implement variable density based on noise and altitude

### 3. Cloud Geometry
- Define bounds for the cloud layer (altitude range and horizontal extent)
- Implement ray marching through the volume with density-based sampling
- Support for different cloud types (cumulus, stratus, cirrus)

### 4. Sky Integration
- Blend clouds with existing sky renderer
- Handle light transport between clouds and atmosphere
- Ensure clouds properly shadow and scatter light from the sun

### 5. Scene Configuration
- Add JSON configuration options for clouds
- Support presets for different weather/cloud conditions
- Allow fine-grained control of cloud parameters

## Implementation Strategy

1. First iteration: Basic noise module and simple cloud layer
2. Second iteration: Improve cloud rendering with better scattering and shapes
3. Third iteration: Add multiple cloud types and weather presets
4. Final iteration: Optimize performance and integrate with sky renderer

## JSON Configuration Example

```json
"clouds": {
  "enabled": true,
  "preset": "cumulus", 
  "base_height": 1000,
  "thickness": 500,
  "density": 0.5,
  "noise_scale": 1.0,
  "detail_octaves": 4,
  "coverage": 0.6,
  "color": [1.0, 1.0, 1.0],
  "phase_function": {
    "type": "henyey-greenstein",
    "g": 0.2
  }
}
```

## Technical Approach

### Ray Marching Algorithm

1. When a ray intersects the cloud layer bounds:
   - Determine entry and exit points of the volume
   - March along the ray with adaptive step sizes
   - At each step, evaluate noise function to get density
   - Accumulate scattering and absorption based on density

2. Light Transport:
   - For each sample point, perform light sampling towards the sun
   - Account for multiple scattering approximation
   - Handle self-shadowing within clouds

### Performance Considerations

- Use adaptive sampling to reduce steps in low-density regions
- Consider implementing acceleration structures for the noise function
- Optimize phase function calculations
- Potential for future GPU implementation

## References

- [Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite](https://media.contentapi.ea.com/content/dam/eacom/frostbite/files/s2016-pbs-frostbite-sky-clouds-new.pdf)
- [Real-time Volumetric Cloudscapes of Horizon: Zero Dawn](https://advances.realtimerendering.com/s2015/The%20Real-time%20Volumetric%20Cloudscapes%20of%20Horizon%20-%20Zero%20Dawn%20-%20ARTR.pdf)
- [Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine](https://www.guerrilla-games.com/read/nubis-authoring-real-time-volumetric-cloudscapes-with-the-decima-engine)