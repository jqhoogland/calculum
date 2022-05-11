# Calculum
## Calculator for Units and Measures 

Calculum is a minilanguage for arithmetic with units and measures.

```python
>>> accel = 10 m/s2
>>> 15 m.kg/s2 / accel
1.5 kg 
>>> (accel) mi/h2
8052.9692 mi/h2 
```

It takes inspiration from [Frinklang](https://frinklang.org/fspdocs.html) with a unit syntax based on the [Unified Coding for Units & Measures (Ucum)](). 

The rest of the language is based on excel — not because I like excel but because I figured it would make adoption a little easier in the contexts this library is most useful. Forgive me. Fortunately for you (if this is making you feel nauseous), the core is abstracted away from syntactics. Thanks to wasm, you can start calculating with units and measures in any language of choice.

---

# A Primer on UCUM

- Units are either **derived** or **base**. E.g., $\text{N}$ is derived from the base units $\text{kg}$, $\text{m}$, $\text{s}$.
- Units are either **prefixed** or **unprefixed**. $\text{kg}$ and $\text{mg}$ are prefixed units, while $\text{g}$ is an unprefixed unit.
  - Only **metric** units can be prefixed, but not all metric units are prefixed.
- Units are **metric** or **customary**, and occasionally "**arbitrary**" (not commensurable with any other unit).    


# Notation (Backus-Naur Form)

```
<sign>          ::= "+" | "-"
<digit>         ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<digits>        ::= <digit><digits> | <digit>
<factor>        ::= <digits>
<exponent>      ::= <sign><digits> | <digits>
<simple-unit>   ::= <ATOM-SYMBOL>
                    | <PREFIX-SYMBOL><ATOM-SYMBOL[metric]>
<annotatable>   ::= <simple-unit><exponent>
                    | <simple-unit>
<component>     ::= <annotatable><annotation>
                    | <annotatable>
                    | <annotation>
                    | <factor>
                    | "("<term>")"
<term>          ::= <term>"."<component>
                    | <term>"/"<component>
                    | <component>
<main-term>     ::= "/"<term>
                    | <term>
<annotation>    ::= "{"<ANNOTATION-STRING>"}"
```


---

## Why not Frinklang?

First, Frinklang is written in Java. Second, it isn't open source. Third, it doesn't use UCUM, which IMHO is the best (i.e., legible and compact) unit coding system we have.  

## Why not an existing library like [UOM](https://github.com/iliekturtles/uom)?

Many existing libraries that work with units and measures are principally non-exhaustive. They enumerate allowed units and measures as types, which is great because it's (probably) faster and can guarantee "unit-safety" at compile time rather than runtime. 

I needed an alternative that focused more on flexibility and extensibility, and I was willing to sacrifice some performance and unit-safety to get this done.


