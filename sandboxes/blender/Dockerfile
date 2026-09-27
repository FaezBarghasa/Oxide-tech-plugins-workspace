FROM python:3.10-slim

# Install system dependencies required by Blender and NVIDIA Warp
RUN apt-get update && apt-get install -y --no-install-recommends \
    libglu1-mesa \
    libxi6 \
    libxrender1 \
    libxxf86vm1 \
    libxkbcommon0 \
    libsm6 \
    && rm -rf /var/lib/apt/lists/*

RUN pip install --no-cache-dir bpy warp-lang requests numpy

WORKDIR /workspace
