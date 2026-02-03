# game_engine

An **opinionated game engine** focused on **3D strategy games** with a **pixel-art rendering style** and a **highly specialized camera system**.

![Status](https://img.shields.io/badge/status-experimental-orange)

## Overview

This is an experimental game engine project aimed at exploring a very specific design space:

**3D strategy games** rendered with a **pixel-art aesthetic**, achieved through a dedicated rendering pipeline and a camera designed explicitly for that visual style.

A rough mental model of the target result would be:
A **city-builder**, but with a **Hyper Light Drifter–like look**, translated into true 3D.

This project is **not** intended to be a general-purpose engine.

## Core Focus

The project revolves around three pillars:

### 1. 3D Pixel Art Rendering

* True **3D geometry**
* Pixel-art–inspired rendering
* Not limited to simple low-resolution rendering
* Use of **custom shaders** to approximate pixel-art aesthetics beyond simple downscaling
* Support for **multiple resolution formats**

### 2. Highly Specialized Camera System

The camera is a **core design element**, not a generic utility.

Its primary goal is to balance:

* pixel-perfect readability
* visual stability (avoiding jitter and distortion)
* player orientation in dense scenes
* controlled cinematic movement

Key characteristics:

* **Player-driven**, but within deliberate constraints
* Strong emphasis on **protecting the pixel-art look**, while still allowing:
  * controlled freedom of movement
  *  exploration of orthographic ↔ perspective transitions

## Current State

The project is **highly experimental**.

Current capabilities are minimal:

⚠️ **This engine is not usable** for making games yet.
Its current purpose is to explore:

* pixel-art 3D rendering techniques
* camera constraints and movement
* foundational engine architecture

APIs, architecture, and even goals are expected to change significantly.

## Technical Notes

* Language: **Rust**
* Rendering: **OpenGL**
* OS / windowing / input: **SDL**
* Target platforms: **Linux**, **Windows**, **macOS**

## 📝 License:

This project is licensed under the MIT, you can use and modify freely, credit is not mandatory but really appreciated. 
