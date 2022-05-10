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

## Why not Frinklang?

First, Frinklang is written in Java. Second, it isn't open source. Third, it doesn't use UCUM, which IMHO is the best (i.e., legible and compact) unit coding system we have.  

## Why not an existing library like [UOM](https://github.com/iliekturtles/uom)?

Many existing libraries that work with units and measures are principally non-exhaustive. They enumerate allowed units and measures as types, which is great because it's (probably) faster and can guarantee "unit-safety" at compile time rather than runtime. 

I needed an alternative that focused more on flexibility and extensibility, and I was willing to sacrifice some performance and unit-safety to get this done.


## Why


In the real world, numbers don't have types, but they occasionally have dimensions: length, duration, charge, luminosity... you name it. So it's puzzling to consider that most programming languages have no native support for dimension-aware computation.

Don't get me wrong: you can load in a library, like Mathematica's [units library](https://reference.wolfram.com/language/workflow/CalculateWithUnits.html) or Python's `pint`. But there generally isn't native support within the language's syntax to create "quantity" objects. 

Which is where calculum comes in.

The syntax is based on [UCUM](), the unified coding for units and measures. UCUM 


It gets especially complicated with a standard like UCUM which makes use of a bunch of characters usually reserved for special purposes:

To make it even more difficult, units can contain numbers, like `g/(12.h)` (grams per 12 hours). 


But it's not too hard to come up with solutions: you could wrap all unit declarations in parentheses or quotes, 

```python
>>> 15 (m.kg/s2) / 10 (m/s2) = 1.5 (kg)
>>> 15 'm.kg/s2' / 10 'm/s2' = 1.5 'kg'
```

Or, we could rely on the fact that UCUM doesn't admit white space in their unit codes.

```python
>>> accel = 10 m/s2
>>> 15 m.kg/s2 / accel = 1.5 kg 
>>> 15 m . kg / s2 / accel  # Error 
```

You could force users to wrap both amount and units in parentheses (though I don't like this approach): 

```python
>>> (15 m . kg / s2) / (10 m/s2) = (1.5 kg)
```

What I'm going to go for is a mix between the string-based and whitespace-denying approaches, so you can do either of the following:

```python
>>> 15 'm . kg / s2' / 10m/s2 
1.5 'kg'
```

As long as the first character of your units isn't a special character, and your units include no whitespace, you can omit the quotes. 

(This case is a little contrived, since it reads better without the extra whitespace, but, hey, it's nice to have just in cases).

I'll make one more exception for annotations:

```python
>>> 15 m.kg/s2{Whitespace is allowed in annotations}
```

Gross? Maybe. But it's forgiving to users, and that's the main thing.

# Conversion

Okay, say we have `speed = 10 mi/h`, and we want to use it in a calculation that involves `km`. What to do?

Just wrap the expression you'd like to cast in parentheses and then provide the annotation outside

```python
>>> 10 mi/h / 6 m2 = .745 '/s/m'
>>> (10 mi/h / 6 m2) '/s/mi' = 1216.7 '/s/mi'

```
