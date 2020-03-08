# Changelog
All notable changes to Photon will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Released]

## [0.1.1] - 2020-03-03
### Added
- Contrast function
- Conversion of `PhotonImage`s to base64
- Solarize function which does not alter a mutable reference, but returns a `PhotonImage` (for waSCC research)
- Invert function
- Mix with color function
- WASM constructor for RGB struct

### Changed
- Image resizing algorithm can now be decided upon. Developers can choose from Nearest Neighbour, Lanczos3, Gaussian, 
CatmullRom, or Triangle.
