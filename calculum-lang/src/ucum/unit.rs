/// # Units
/// A unit is of the kind:
/// - `m`, `bit`, `atm`: A unit "atom".
/// - `kg`, `Gib`, `mK`: A metric unit "atom" with a prefix.
/// - `m1`, `s-2`, `W-1`: A unit with an exponent (default is 1).
///
/// # Todos
/// - [ ] Support for `10^`. (I'm not going to implement `10*`).
///
/// # Notes
/// - I've added in `"` as an alias for `''` (angle seconds).
/// 

pub mod constants {
    pub const PREFIXES: [&str; 23] = [
        "Y", "Z", "E", "P", "T", "G", "M", "k", "h", "da", "d", "c", "mu", "n",
        "p", "f", "a", "z", "y",
        // Powers of 2 (usu. reserved to bits)
        "Ki", "Mi", "Gi", "Ti",
    ];

    // No support for prefixed ' & " (minutes & seconds)
    pub const METRIC_UNIT_ATOMS: [&str; 96] = [
        "m", "s", "g", "rad", "K", "C", "cd", "mol", "sr", "Hz", "N", "Pa", "J",
        "W", "A", "V", "F", "Ohm", "S", "Wb", "Cel", "T", "H", "lm", "lx", "Bq",
        "Gy", "Sv", "l", "L", "ar", "t", "bar", "u", "eV", "pc", "[c]", "[h]",
        "[k]", "[eps_0]", "[mu_0]", "[e]", "[m_e]", "[m_p]", "[G]", "[g]", "[ly]",
        "gf", "Ky", "Gal", "dyn", "erg", "P", "Bi", "St", "Mx", "G", "Oe", "Gb",
        "sb", "Lmb", "ph", "Ci", "R", "RAD", "REM", "cal_[15]", "cal_[20]",
        "cal_m", "cal_IT", "cal_th", "cal", "tex", "m[H2O]", "m[Hg]", "eq", "osm",
        "g%", "kat", "U", "[iU]", "[IU]", "Np", "B", "B[SPL]", "B[V]", "B[mV]",
        "B[uV]", "B[10.nV]", "B[W]", "B[kW]", "st", "mho", "bit", "By", "Bd",
    ];

    // Excluding `10*` which is a stupid way to represent "ten to the power"
    pub const NON_METRIC_UNIT_ATOMS: [&str; 221] = [
        "'", "\"", "10^", "[pi]", "%", "[ppth]", "[ppm]", "[ppb]", "[pptr]", "gon",
        "deg", "'", "min", "h", "d", "a_t", "a_j", "a_g", "a", "wk", "mo_s",
        "mo_j", "mo_g", "mo", "AU", "atm", "[lbf_av]", "[in_i]", "[ft_i]",
        "[yd_i]", "[mi_i]", "[fth_i]", "[nmi_i]", "[kn_i]", "[sin_i]", "[sft_i]",
        "[syd_i]", "[cin_i]", "[cft_i]", "[cyd_i]", "[bf_i]", "[cr_i]", "[mil_i]",
        "[cml_i]", "[hd_i]", "[ft_us]", "[yd_us]", "[in_us]", "[rd_us]", "[ch_us]",
        "[lk_us]", "[rch_us]", "[rlk_us]", "[fth_us]", "[fur_us]", "[mi_us]",
        "[acr_us]", "[srd_us]", "[smi_us]", "[sct]", "[twp]", "[mil_us]", "[in_br]",
        "[ft_br]", "[rd_br]", "[ch_br]", "[lk_br]", "[fth_br]", "[pc_br]",
        "[yd_br]", "[mi_br]", "[nmi_br]", "[kn_br]", "[acr_br]", "[gal_us]",
        "[bbl_us]", "[qt_us]", "[pt_us]", "[gil_us]", "[foz_us]", "[fdr_us]",
        "[min_us]", "[crd_us]", "[bu_us]", "[gal_wi]", "[pk_us]", "[dqt_us]",
        "[dpt_us]", "[tbs_us]", "[tsp_us]", "[cup_us]", "[foz_m]", "[cup_m]",
        "[tsp_m]", "[tbs_m]", "[gal_br]", "[pk_br]", "[bu_br]", "[qt_br]",
        "[pt_br]", "[gil_br]", "[foz_br]", "[fdr_br]", "[min_br]", "[gr]",
        "[lb_av]", "[oz_av]", "[dr_av]", "[scwt_av]", "[lcwt_av]", "[ston_av]",
        "[lton_av]", "[stone_av]", "[pwt_tr]", "[oz_tr]", "[lb_tr]", "[sc_ap]",
        "[dr_ap]", "[oz_ap]", "[lb_ap]", "[oz_m]", "[lne]", "[pnt]", "[pca]",
        "[pnt_pr]", "[pca_pr]", "[pied]", "[pouce]", "[ligne]", "[didot]",
        "[cicero]", "[degF]", "[degR]", "[degRe]", "[Cal]", "[Btu_39]", "[Btu_59]",
        "[Btu_60]", "[Btu_m]", "[Btu_IT]", "[Btu_th]", "[Btu]", "[HP]", "[den]",
        "[in_i'H2O]", "[in_i'Hg]", "[PRU]", "[wood'U]", "[diop]", "[p'diop]",
        "%[slope]", "[mesh_i]", "[Ch]", "[drp]", "[hnsf'U]", "[MET]", "[hp'_X]",
        "[hp'_C]", "[hp'_M]", "[hp'_Q]", "[hp_X]", "[hp_C]", "[hp_M]", "[hp_Q]",
        "[kp_X]", "[kp_C]", "[kp_M]", "[kp_Q]", "[pH]", "[S]", "[HPF]", "[LPF]",
        "[arb'U]", "[USP'U]", "[GPL'U]", "[MPL'U]", "[APL'U]", "[beth'U]",
        "[anti'Xa'U]", "[todd'U]", "[dye'U]", "[smgy'U]", "[bdsk'U]", "[ka'U]",
        "[knk'U]", "[mclg'U]", "[tb'U]", "[CCID_50]", "[TCID_50]", "[EID_50]",
        "[PFU]", "[FFU]", "[CFU]", "[IR]", "[BAU]", "[AU]", "[Amb'a'1'U]", "[PNU]",
        "[Lf]", "[D'ag'U]", "[FEU]", "[ELU]", "[EU]", "Ao", "b", "att", "[psi]",
        "circ", "sph", "[car_m]", "[car_Au]", "[smoot]", "[m/s2/Hz^(1/2)]", "bit_s",
        "{tot}", "{tbl}", "{rbc}", "g.m/{H.B.}", "gf.m/{H.B.}", "kg{wet'tis}",
        "mg{creat}"
    ];

    pub const CONVERSIONS: [(&str, f64, &str); 316] = [
        ("m", 1., "m"), ("s", 1., "s"), ("g", 1., "g"), ("rad", 1., "rad"), 
        ("K", 1., "K"), ("C", 1., "C"), ("cd", 1., "cd"), 
        ("[pi]", std::f64::consts::PI, "1"), ("%", 10e-2, "1"),
        ("[ppth]", 10e-3, "1"), ("[ppm]", 10e-6,  "1"), ("[ppb]", 10e-9, "1"), 
        ("[pptr]", 10e-12,  "1"), ("mol", 6.0221367e23, "1"), ("sr", 1., "rad2"),
        ("Hz", 1., "s-1"), ("N", 1., "kg.m/s2"), ("Pa", 1., "N/m2"), 
        ("J", 1., "N.m"), ("W", 1., "J/s"), ("A", 1., "C/s"), ("V", 1., "J/C"),
        ("F", 1., "C/V"), ("Ohm", 1., "V/A"), ("S", 1., "Ohm-1"), 
        ("Wb", 1., "V.s"), ("Cel", 1., "=cel(1 K)"), ("T", 1., "Wb/m2"), 
        ("H", 1., "Wb/A"), ("lm", 1., "cd.sr"), ("lx", 1., "lm/m2"), 
        ("Bq", 1., "s-1"), ("Gy", 1., "J/kg"), ("Sv", 1., "J/kg"),
        ("gon", 0.9, "deg"), ("deg", 2./360., "[pi].rad"), ("'", 1./60., "deg"),
        ("''", 1./60., "'"), ("\"", 1./60., "'"), ("l", 1., "dm3"),
        ("L", 1., "l"), ("ar", 100., "m2"), ("min", 60., "s"), ("h", 60., "min"),
        ("d", 24., "h"), ("a_t", 365.24219, "d"), ("a_j", 365.25, "d"), 
        ("a_g", 365.2425, "d"), ("a", 1., "a_j"), ("wk", 7., "d"), 
        ("mo_s", 29.53059, "d"), ("mo_j", 1., "a_j/12"), ("mo_g", 1., "a_g/12"),
        ("mo", 1., "mo_j"), ("t", 1e3, "kg"), ("bar", 1e5, "Pa"),
        ("u", 1.6605402e-24, "g"), ("eV", 1., "[e].V"),
        ("AU", 149597.8707, "Mm"), ("pc", 3.085678e16, "m"), 
        ("[c]", 299792458., "m/s"), ("[h]", 6.6260755e-34, "J.s"), 
        ("[k]", 1.380658e-23, "J/K"), ("[eps_0]", 8.854187817e-12, "F/m"), 
        ("[mu_0]",  4. * 10e-7, "[pi].N/A2"), ("[e]", 1.60217733e19, "C" ),
        ("[m_e]", 9.1093897e28, "g" ), ("[m_p]", 1.6726231e24, "g" ),
        ("[G]", 6.67259e11, "m3.kg-1.s-2" ), ("[g]", 9.80665, "m/s2" ),
        ("atm", 101325., "Pa"), ("[ly]", 1., "[c].a_j" ), ("gf", 1., "g.[g]"),
        ("[lbf_av]", 1., "[lb_av].[g]"), ("Ky", 1., "cm-1"),
        ("Gal", 1., "cm/s2"), ("dyn", 1., "g.cm/s2"), ("erg", 1., "dyn.cm"), 
        ("P", 1., "dyn.s/cm2"), ("Bi", 10., "A"), ("St", 1., "cm2/s"), 
        ("Mx", 1e8, "Wb"), ("G", 1e4, "T"), ("Oe", 250., "/[pi].A/m"), 
        ("Gb", 1., "Oe.cm"), ("sb", 1., "cd/cm2"), ("Lmb", 1., "cd/cm2/[pi]"),
        ("ph", 1e4, "lx"), ("Ci", 3.7e10, "Bq"), ("R", 2.58e4, "C/kg"), 
        ("RAD", 100., "erg/g"), ("REM", 1., "RAD"), ("[in_i]", 2.54, "cm"),
        ("[ft_i]", 12., "[in_i]"), ("[yd_i]", 3., "[ft_i]"), 
        ("[mi_i]", 5280., "[ft_i]"), ("[fth_i]", 6., "[ft_i]"), 
        ("[nmi_i]", 1852., "m"), ("[kn_i]", 1., "[nmi_i]/h"),
        ("[sin_i]", 1., "[in_i]2"), ("[sft_i]", 1., "[ft_i]2"),
        ("[syd_i]", 1., "[yd_i]2"), ("[cin_i]", 1., "[in_i]3"), 
        ("[cft_i]", 1., "[ft_i]3"), ("[cyd_i]", 1., "[yd_i]3"), 
        ("[bf_i]", 144., "[in_i]3"), ("[cr_i]", 128., "[ft_i]3"), 
        ("[mil_i]", 1e3,"[in_i]"), ("[cml_i]", 1., "[pi]/4.[mil_i]2"), 
        ("[hd_i]", 4., "[in_i]"), ("[ft_us]", 1200./3937., "m"), 
        ("[yd_us]", 3., "[ft_us]"), ("[in_us]", 1./12., "[ft_us]"),
        ("[rd_us]", 16.5, "[ft_us]"), ("[ch_us]", 4., "[rd_us]"), 
        ("[lk_us]", 1./100., "[ch_us]"), ("[rch_us]", 100., "[ft_us]"),
        ("[rlk_us]", 1./100., "[rch_us]"), ("[fth_us]", 6., "[ft_us]"),
        ("[fur_us]", 40., "[rd_us]"), ("[mi_us]", 8., "[fur_us]"), 
        ("[acr_us]", 160., "[rd_us]2"), ("[srd_us]", 1., "[rd_us]2"), 
        ("[smi_us]", 1., "[mi_us]2"), ("[sct]", 1., "[mi_us]2"), 
        ("[twp]", 36., "[sct]"), ("[mil_us]", 1e3,"[in_us]"),        
        ("[in_br]", 2.539998, "cm"), ("[ft_br]", 12., "[in_br]"), 
        ("[rd_br]", 16.5, "[ft_br]"), ("[ch_br]", 4., "[rd_br]"), 
        ("[lk_br]", 1., "[ch_br]/100"), ("[fth_br]", 6., "[ft_br]"), 
        ("[pc_br]", 2.5, "[ft_br]"), ("[yd_br]", 3., "[ft_br]"), 
        ("[mi_br]", 5280., "[ft_br]"), ("[nmi_br]", 6080., "[ft_br]"),
        ("[kn_br]", 1., "[nmi_br]/h"), ("[acr_br]", 4840., "[yd_br]2"),  
        ("[gal_us]", 231., "[in_i]3"), ("[bbl_us]", 42., "[gal_us]"),
        ("[qt_us]", 1., "[gal_us]/4"), ("[pt_us]", 1., "[qt_us]/2"), 
        ("[gil_us]", 1., "[pt_us]/4"), ("[foz_us]", 1., "[gil_us]/4"), 
        ("[fdr_us]", 1., "[foz_us]/8"), ("[min_us]", 1., "[fdr_us]/60"),
        ("[crd_us]", 128., "[ft_i]3"), ("[bu_us]", 2150.42, "[in_i]3"), 
        ("[gal_wi]", 1., "[bu_us]/8"), ("[pk_us]", 1., "[bu_us]/4"), 
        ("[dqt_us]", 1., "[pk_us]/8"), ("[dpt_us]", 1., "[dqt_us]/2"),
        ("[tbs_us]", 1., "[foz_us]/2"), ("[tsp_us]", 1., "[tbs_us]/3"), 
        ("[cup_us]", 16., "[tbs_us]"), ("[foz_m]", 30., "mL"), 
        ("[cup_m]", 240., "mL"), ("[tsp_m]", 5., "mL"), ("[tbs_m]", 15., "mL"),
        ("[gal_br]", 4.54609, "l"), ("[pk_br]", 2., "[gal_br]"),
        ("[bu_br]", 4., "[pk_br]"), ("[qt_br]", 1., "[gal_br]/4"), 
        ("[pt_br]", 1., "[qt_br]/2"), ("[gil_br]", 1., "[pt_br]/4"), 
        ("[foz_br]", 1., "[gil_br]/5"), ("[fdr_br]", 1., "[foz_br]/8"),
        ("[min_br]", 1., "[fdr_br]/60"), ("[gr]", 64.79891, "mg"), 
        ("[lb_av]", 7000., "[gr]"), ("[oz_av]", 1., "[lb_av]/16"), 
        ("[dr_av]", 1., "[oz_av]/16"), ("[scwt_av]", 100., "[lb_av]"),
        ("[lcwt_av]", 112., "[lb_av]"), ("[ston_av]", 20., "[scwt_av]"),
        ("[lton_av]", 20., "[lcwt_av]"), ("[stone_av]", 14., "[lb_av]"),
        ("[pwt_tr]", 24., "[gr]"), ("[oz_tr]", 20., "[pwt_tr]"), 
        ("[lb_tr]", 12., "[oz_tr]"), ("[sc_ap]", 20., "[gr]"),
        ("[dr_ap]", 3., "[sc_ap]"), ("[oz_ap]", 8., "[dr_ap]"),
        ("[lb_ap]", 12., "[oz_ap]"), ("[oz_m]", 28., "g"),
        ("[lne]", 1., "[in_i]/12"), ("[pnt]", 1., "[lne]/6"),
        ("[pca]", 12., "[pnt]"), ("[pnt_pr]", 0.013837, "[in_i]"),
        ("[pca_pr]", 12., "[pnt_pr]"), ("[pied]", 32.48, "cm"),
        ("[pouce]", 1., "[pied]/12"), ("[ligne]", 1., "[pouce]/12"),
        ("[didot]", 1., "[ligne]/6"), ("[cicero]", 12., "[didot]"),
        ("[degF]", 1., "=degf(5/9 K)"), ("[degR]", 5., "K/9"),
        ("[degRe]", 1., "=degre(5/4 K)"), ("cal_[15]", 4.1858, "J"),
        ("cal_[20]", 4.1819, "J"), ("cal_m", 4.19002, "J"),
        ("cal_IT", 4.1868, "J"), ("cal_th", 4.184, "J"),
        ("cal", 1., "cal_th"), ("[Cal]", 1., "kcal_th"),
        ("[Btu_39]", 1.05967, "kJ"), ("[Btu_59]", 1.0548, "kJ"),
        ("[Btu_60]", 1.05468, "kJ"), ("[Btu_m]", 1.05587, "kJ"),
        ("[Btu_IT]", 1.055055853, "kJ"), ("[Btu_th]", 1.05435, "kJ"),
        ("[Btu]", 1., "[Btu_th]"), ("[HP]", 550., "[ft_i].[lbf_av]/s"),
        ("tex", 1., "g/km"), ("[den]", 1., "g/9/km"), ("m[H2O]", 9.80665, "kPa"),
        ("m[Hg]", 133.322, "kPa"), ("[in_i'H2O]", 1., "m[H2O].[in_i]/m"),
        ("[in_i'Hg]", 1., "m[Hg].[in_i]/m"), ("[PRU]", 1., "mm[Hg].s/ml"),
        ("[wood'U]", 1., "mm[Hg].min/L"), ("[diop]", 1., "/m"),
        ("[p'diop]", 100., "=tan(1, rad)"), ("%[slope]", 100., "=tan(1, rad)"),
        ("[mesh_i]", 1., "/[in_i]"), ("[Ch]", 1., "mm/3"), ("[drp]", 1., "ml/20"),
        ("[hnsf'U]", 1., "1"), ("[MET]", 3.5, "=mL/min/kg"),
        ("[hp'_X]", 1., "=hpX(1, 1)"), ("[hp'_C]", 1., "=hpC(1, 1)"),
        ("[hp'_M]", 1., "=hpM(1, 1)"), ("[hp'_Q]", 1., "=hpQ(1, 1)"),
        ("[hp_X]", 1., "="), ("[hp_C]", 1., "="), ("[hp_M]", 1., "="),
        ("[hp_Q]", 1., "="), ("[kp_X]", 1., "="), ("[kp_C]", 1., "="),
        ("[kp_M]", 1., "="), ("[kp_Q]", 1., "="), ("eq", 1., "mol"),
        ("osm", 1., "mol"), ("[pH]", 1., "=pH(1., mol/l)"), ("g% 1", 1., "g/dl"),
        ("[S]", 1., "10*-13.s"), ("[HPF]", 1., "1"), ("[LPF]", 100., "1"),
        ("kat", 1., "mol/s"), ("U", 1., "umol/min"), ("[iU]", 1., "="),
        ("[IU]", 1., "="), ("[arb'U]", 1., "="), ("[USP'U]", 1., "="),
        ("[GPL'U]", 1., "="), ("[MPL'U]", 1., "="), ("[APL'U]", 1., "="),
        ("[beth'U]", 1., "="), ("[anti'Xa'U]", 1., "="), ("[todd'U]", 1., "="),
        ("[dye'U]", 1., "="), ("[smgy'U]", 1., "="), ("[bdsk'U]", 1., "="),
        ("[ka'U]", 1., "="), ("[knk'U]", 1., "="), ("[mclg'U]", 1., "="),
        ("[tb'U]", 1., "="), ("[CCID_50]", 1., "="), ("[TCID_50]", 1., "="),
        ("[EID_50]", 1., "="), ("[PFU]", 1., "="), ("[FFU]", 1., "="),
        ("[CFU]", 1., "="), ("[IR]", 1., "="), ("[BAU]", 1., "="),
        ("[AU]", 1., "="), ("[Amb'a'1'U]", 1., "="), ("[PNU]", 1., "="),
        ("[Lf]", 1., "="), ("[D'ag'U]", 1., "="), ("[FEU]", 1., "="),
        ("[ELU]", 1., "="), ("[EU]", 1., "="), ("Np", 1., "=ln(1, 1)"),
        ("B", 1., "=lg(1, 1)"), ("B[SPL]", 1., "=2*lg(2, 10e-5 Pa)"),
        ("B[V]", 1., "=2*lg(1., V)"), ("B[mV]", 1., "=2*lg(1., mV)"),
        ("B[uV]", 1., "=2*lg(1., uV)"), ("B[10.nV]", 1., "=2*lg(10., nV)"),
        ("B[W]", 1., "=lg(1., W)"), ("B[kW]", 1., "=lg(1., kW)"),
        ("st", 1., "m3"), ("Ao", 0.1, "nm"), ("b", 100., "fm2"),
        ("att", 1., "kgf/cm2"), ("mho", 1., "S"),
        ("[psi]", 1., "[lbf_av]/[in_i]2"), ("circ", 2., "[pi].rad"),
        ("sph", 4., "[pi].sr"), ("[car_m]", 0.2, "g"), ("[car_Au]", 1., "/24"),
        ("[smoot]", 67., "[in_i]"), ("[m/s2/Hz^(1/2)]", 1., "=sqrt(1., m2/s4/Hz)"),
        ("bit_s", 1., "ld(1 1)"), ("bit", 1., "1"), ("By", 8., "bit"),
        ("Bd", 1., "/s"), ("{tot}", 1., "1"), ("{tbl}", 1., "1"), ("{rbc}", 1., "1"),
        ("g.m/{H.B.}", 1., "g.m"), ("gf.m/{H.B.}", 1., "gf.m"), ("kg{wet'tis}", 1., "kg"),
        ("mg{creat}", 1., "mg"),
    ];
}

mod tokenizer {
    use std::fmt;
    use std::cmp::min;

    use super::constants::*;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Unit {
        pub prefix: String,
        pub atom: String,
        pub exp: i8,
        pub annotation: Option<String>
    }

    impl Unit {
        pub fn new(s: String, exp: i8, annotation: Option<String>) -> Self {
            let mut s_0: &str = "";
            let mut s_rest: &str = &s[..];

            // Prefixes can be 1 or 2 characters long
            for i in 1..min(3, s.chars().count()) {
                s_0 = &s[..i];
                s_rest = &s[i..];

                if PREFIXES.contains(&s_0) {
                    if METRIC_UNIT_ATOMS.contains(&s_rest) {
                        return Unit {
                            prefix: s_0.to_string(),
                            atom: s_rest.to_string(),
                            exp,
                            annotation
                        }
                    }
                }
            }

            return Unit {
                prefix: "".to_string(),
                atom: (s_0.to_owned() + s_rest),
                exp,
                annotation
            }
        }

        pub fn invert(&self) -> Self {
            Unit {
                 prefix: self.prefix.clone(),
                 atom: self.atom.clone(),
                 exp: -self.exp,
                 annotation: self.annotation.clone()
             }
        }

        pub fn unit(&self) -> String {
            self.prefix.clone() + &self.atom
        }

        pub fn has_same_unit(&self, other: &Self) -> bool {
            self.unit() == other.unit()
        }
    }

    impl fmt::Display for Unit {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}{}{}", self.prefix, self.atom, self.exp)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Units<'a>(pub &'a Vec<Unit>);

    impl<'a> fmt::Display for Units<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let display: String = self.0
                .iter()
                .fold(
                    "".to_string(),
                    |result, u| {
                        let u_exp = match u.exp {
                            1  => "".to_string(),
                            _ => u.exp.to_string()
                        };

                        match result.as_ref() {
                            "" => format!("{}{}", u.unit(), u_exp),
                            _ => format!("{}.{}{}", result, u.unit(), u_exp)
                        }
                    });


            write!(f, "{}", display)
        }
    }
}


pub use tokenizer::Unit;
pub use tokenizer::Units;

#[cfg(test)]
mod test {
    use super::{constants::*, tokenizer::*};

    #[test]
    fn it_tokenizes_prefixes_on_metric_units() {
        for prefix in PREFIXES {
            for metric_atom in METRIC_UNIT_ATOMS {
                let unit = String::from(prefix) + metric_atom;
                assert_eq!(
                    Unit {prefix: prefix.to_string(), atom: metric_atom.to_string(), exp: 1, annotation: None},
                    Unit::new(unit, 1, None),
                );
            }
        }
    }

    #[test]
    fn it_ignores_prefixes_on_metric_units() {
        for atom in NON_METRIC_UNIT_ATOMS {
            assert_eq!(
                Unit { prefix: "".to_string(), atom: atom.to_string(), exp: 1, annotation: None},
                Unit::new(atom.to_string(), 1, None),
            );
        }
    }
}
