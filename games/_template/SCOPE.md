---
title: "Template Prototype"
phase: "Prototype"
progress: 25
dimension: "2d"
current_objective: "Implement core jump and movement physics"
blockers: "None"
---

# Project: Template Prototype

**Pitch:** A 2D physics-driven action platformer starter template.

## Architectural Decisions
- **Physics:** Avian2D (`Dynamic` character controller with capsule collider)
- **Perspective:** 2D Side-Scroller
- **Inputs:** Leafwing `PlayerAction` (Move VirtualAxis/DPad, Jump button)

## Milestone Checklist
- [x] Initial Scaffolding & State Pipeline (`Loading -> Menu -> Playing`)
- [x] Player Entity with Avian Collider & Leafwing Input Mapping
- [ ] Core Gameplay Mechanic #1
- [ ] Win/Loss State Loop

## Agent Session Log
- **Latest Update:** Initialized template with headless physics + input test.
- **Next Logical Step:** Add level geometry and sensor triggers for win conditions.
