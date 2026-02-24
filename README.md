# Kynex

## Vec2 (Fundamental Unit)

```rust
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
```

This is literally just a 2D Vector

Position → (x, y)

Velocity → (vx, vy)

Force → (fx, fy)

Normal → direction vector

Offset from center → r

## Dot product

```rust
pub fn dot(self, rhs: Vec2) -> f32
```

formula

```
x1*x2 + y1*y2

```

Test

## Learning Plan

# Physics Engine Math Roadmap (Practical)

This file is a **math-only checklist + resource hub** for building a physics engine.
Work top-to-bottom. Don’t skip phases.

---

## Phase 1 — Vectors (Core of Everything)
**Goals**
- What a vector is (2D, 3D)
- Add / subtract / scale
- Length (magnitude)
- Normalize
- Distance between two points
- Dot product (meaning + use cases)
- Cross product
  - 2D (scalar z-value)
  - 3D (vector result)
- Projection of one vector onto another
- Reflection of a vector across a normal

**Resources**
- 3Blue1Brown — Essence of Linear Algebra (visual intuition):
  https://www.3blue1brown.com/essence-of-linear-algebra/
- Khan Academy — Linear Algebra:
  https://www.khanacademy.org/math/linear-algebra
- BetterExplained — Linear Algebra Guide:
  https://betterexplained.com/articles/linear-algebra-guide/

---

## Phase 2 — Trigonometry (Rotation & Directions)
**Goals**
- Degrees ↔ radians
- sin, cos, tan (unit circle meaning)
- Build a direction vector from an angle
- Rotate a vector using sin/cos
- Angle between two vectors (via dot product)

**Resources**
- Khan Academy — Trigonometry:
  https://www.khanacademy.org/math/trigonometry
- PatrickJMT (short, clear math videos):
  https://www.youtube.com/@patrickJMT

---

## Phase 3 — Linear Algebra (Practical Transforms)
**Goals**
- What a matrix is (as a transform)
- 2×2 matrices for 2D rotation & scale
- Matrix × vector multiplication
- 3×3 matrices for 2D transforms (homogeneous coords)
- (Optional later) 4×4 matrices for 3D
- Basis vectors & coordinate systems
- Inverse of a transform (conceptually)

**Resources**
- 3Blue1Brown — Essence of Linear Algebra:
  https://www.3blue1brown.com/essence-of-linear-algebra/
- Khan Academy — Linear Algebra:
  https://www.khanacademy.org/math/linear-algebra

---

## Phase 4 — Geometry (Collision Foundations)
**Goals**
- Distance: point ↔ point
- Distance: point ↔ line
- Distance: point ↔ segment
- Closest point on a line / segment
- Line–line intersection (2D)
- Circle–circle intersection
- AABB–AABB overlap test
- Ray–circle intersection
- Ray–AABB intersection

**Resources**
- Scratchapixel (excellent practical geometry & graphics math):
  https://www.scratchapixel.com/

---

## Phase 5 — Projections & SAT (2D Collision MVP)
**Goals**
- Project a shape onto an axis
- Interval overlap test
- What the Separating Axis Theorem (SAT) says
- SAT for:
  - Box vs box
  - Convex polygon vs polygon
  - Circle vs polygon
- Find collision normal & penetration depth

**Resources**
- Scratchapixel (projections, intersections, geometry):
  https://www.scratchapixel.com/
- Book: "Real-Time Collision Detection" — Christer Ericson (gold standard reference)

---

## Phase 6 — Calculus (Simulation Basics, No Overkill)
**Goals**
- What a derivative represents (velocity, acceleration)
- What an integral represents (accumulation over time)
- Discrete integration with time steps
- Euler integration
- Semi-implicit (symplectic) Euler
- Understand what `dt` means physically

**Resources**
- Khan Academy — Calculus 1:
  https://www.khanacademy.org/math/calculus-1
- 3Blue1Brown — Calculus (visual intuition):
  https://www.3blue1brown.com/topics/calculus

---

## Phase 7 — Numerical Stability (Engine Survivability)
**Goals**
- Floating point error (intuition)
- Fixed timestep concept
- Accumulator update loop
- Error accumulation & drift
- Iterative methods (idea of “repeat to converge”)

**Resources**
- Game Physics Engine Development — Ian Millington (practical engine-focused book)
- Various articles on "fixed timestep game loop" and "numerical integration stability"

---

## Books (Highly Recommended)
- "Mathematics for 3D Game Programming and Computer Graphics" — Eric Lengyel  
- "Real-Time Collision Detection" — Christer Ericson  
- "Game Physics Engine Development" — Ian Millington  

---

## Rule of Progression
Do NOT move on until you can:
- Explain the concept in plain words
- Solve a few problems by hand
- Implement it in code later

---

## End Goal Checklist
You should be able to:
- Implement your own Vec2 / Vec3
- Rotate vectors with sin/cos or matrices
- Compute dot/cross without thinking
- Compute distances & closest points
- Explain SAT in plain English
- Integrate motion using dt
