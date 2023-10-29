# Graphmage Rendering

This crate provides necessary machinery requred to render a graphmage graph into
a buffer or on a surface.

## Goals
- [x] Support OpenGL backend
- [ ] Support Vulkan backend
- [ ] Support WebGPU backend
- [ ] C bindings

### Non-goals

These might change down the line, though they are not on the roadmap until most
Graphviz features are implemented.

- Support Metal backend - supported via MoltenVK
- Support DirectX backend - Windows supports OpenGL and Vulkan
