# 🧮 Calculum
### ⚖️ *Calcul*ator for *U*nits and *M*easures 

Calculum is a calculator and programming language for handling arithmetic with units and measures. It's dimension-aware, and can automatic convert quantities and validate compatibility.

```python
>>> accel = 10 m/s2
>>> 15 m.kg/s2 / accel
1.5 kg 
>>> (accel) mi/h2
8052.9692 mi/h2 
```

It takes inspiration from [Frinklang](https://frinklang.org/fspdocs.html) with a unit syntax based on the [Unified Code for Units & Measures (UCUM)](). 

The rest of the language is based on excel — not because I like excel but because I figured it would make adoption a little easier in the contexts this library is most useful. Forgive me. Fortunately for you (if this is making you feel nauseous), the core is abstracted away from syntactics. Thanks to wasm, you can start calculating with units and measures in any language of choice.

---

### Why not Frinklang?

First, Frinklang is written in Java. Second, it isn't open source. Third, it doesn't use UCUM, which IMHO is the best (i.e., legible and compact) unit coding system we have.  

### Why not an existing library like [UOM](https://github.com/iliekturtles/uom)?

Many existing libraries that work with units and measures are principally non-exhaustive. They enumerate allowed units and measures as types, which is great because it's (probably) faster and can guarantee "unit-safety" at compile time rather than runtime. 

I needed an alternative that focused more on flexibility and extensibility, and I was willing to sacrifice some performance and unit-safety to get this done.

