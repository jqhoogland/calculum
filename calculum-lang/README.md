
# A Primer on UCUM

- A unit is either a **derived** unit or a **base** unit. E.g., `N` is derived from the base units `kg`, `m`, `s`.
- Units are either **prefixed** or **unprefixed**. `kg` and `mg` are prefixed units, while `g` is an unprefixed unit.
  - Only **metric** units can be prefixed, but not all metric units are prefixed.
- Units are **metric** or **customary**, and occasionally "**arbitrary**" (that they can't be converted to any other unit).    


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


