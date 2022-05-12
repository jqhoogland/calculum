//! # Unified Code for Units and Measures
//!
//! UCUM was designed by the folks at [Regenstrief](https://www.regenstrief.org/),
//! (who also happen to be the wizards behind LOINC) out of frustration with
//! standards like ISO 2955 and ANSI X3.50.
//!
//! In particular, they weren't fans of:
//! - Naming conflicts (e.g., "a" for both "year" and "are"),
//! - Missing units (e.g., ISO neglects the degree Fahrenheit), &
//! - Ambiguity surrounding customary units (did you know there are
//!   distinctions between an international foot, a US foot, a survey foot,
//!   and a British foot?).
//!
//! UCUM provides an unambiguous and exhaustive coding system for units.
//! (No surprises -- it's in the name!).
//!
//! # Examples
//! The coding ends up looking something like this:
//! - `N = kg.m/s2` (Newton)
//! - `G = m3.kg-1.s-2` (Gravitation constant)
//! - `mu_0 = 4.[pi].10*-7.N/A2` (Magnetic permeability)
//! - `[in_i'Hg] = m[hg].[in_i]/m`  (One inch of mercury column)
//! - `deg = 2 [pi].rad/360` (Degree)
//!
//! That is: `.` for multiplication, `/` for division, and implicit
//! exponentiation. In practice, `.` for multiplication doesn't lead to
//! ambiguity because floating point numbers aren't allowed in unit term.
//!
//! Customary (non-metric) units, as well as potentially conflicting units, are
//! wrapped in square brackets (intended to shame their users), and you can add
//! annotations with curly brace blocks, e.g., `kg{wet tis.}`. (UCUM strongly
//! recommends against annotations but accepts that this practice is too
//! ingrained to leave out.)
//!
//! # Note
//! - This departs from UCUM in one significant way, and that is that `*` is
//!   parsed as an alternative to `.`, (multiplication) rather than `^`
//!   (exponentiation). (In what universe does `*` make sense as exponentiation?)
//!
//! # Vocabulary
//!
//! #### Atoms, Prefixes, Units, Unit Terms, Quantities, and Expressions
//! - `km` is a "**unit**" in `ucum::unit`. It consists of a "**prefix**"
//!   (`k`) and "**unit atom**" (`m`). Only metric units may include prefixes.
//! - `kg.m/s2` is parsed as a "**unit term**" in `ucum::term`.
//! - `10 kg.m/s2` or `10kg.m/s2` is a "**quantity**".
//! - `10 kg.m/s2 / 10 m/s2 == 1 kg` is an "**expression**" in
//!   `ucum::expression`.
//!
//! ### Comparing units, terms, & quantities
//! - Two units are "**equal**" iff they are the exact same unit.
//! - Two unit terms are "**equal**" if they consist of the same base units
//!   (e.g., `N` and `kg.m/s2`).
//! - Two unit terms are "**commensurable**" if they they have the same dimension
//!   & can be converted (e.g., `m/s`, `km/h`, and `[mi]/h`).
//! - Two quantities are "**equal**" if their unit terms are commensurable,
//!   and if their magnitudes are equal after conversion.
//!

pub mod term;
pub mod unit;
pub mod expression;