mod constants {
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
}

mod tokenizer {
    use std::cmp::min;
    use super::constants::*;

    #[derive(Debug, PartialEq)]
    pub struct Token {
        pub prefix: String,
        pub atom: String,
        pub exp: i8,
    }

    impl Token {
        pub fn new(s: String, exp: i8) -> Self {
            let mut s_0: &str = "";
            let mut s_rest: &str = &s[..];

            // Prefixes can be 1 or 2 characters long
            for i in 1..min(3, s.chars().count()) {
                s_0 = &s[..i];
                s_rest = &s[i..];

                if PREFIXES.contains(&s_0) {
                    if METRIC_UNIT_ATOMS.contains(&s_rest) {
                        return Token {
                            prefix: s_0.to_string(),
                            atom: s_rest.to_string(),
                            exp,
                        }
                    }
                }
            }

            return Token {
                prefix: "".to_string(),
                atom: (s_0.to_owned() + s_rest),
                exp
            }
        }
    }
}

pub mod parser {
    use super::tokenizer::Token;

    #[derive(Debug, PartialEq)]
    pub struct Unit(Token);

    impl Unit {
        pub fn new(s: String, exp: i8) -> Self {
            Unit(Token::new(s, exp))
        }
    }
}


#[cfg(test)]
mod test {
    use super::{constants::*, tokenizer::*};

    #[test]
    fn it_tokenizes_prefixes_on_metric_units() {
        for prefix in PREFIXES {
            for metric_atom in METRIC_UNIT_ATOMS {
                let unit = String::from(prefix) + metric_atom;
                assert_eq!(
                    Token {prefix: prefix.to_string(), atom: metric_atom.to_string(), exp: 1},
                    Token::new(unit, 1),
                );
            }
        }
    }

    #[test]
    fn it_ignores_prefixes_on_metric_units() {
        for atom in NON_METRIC_UNIT_ATOMS {
            assert_eq!(
                Token { prefix: "".to_string(), atom: atom.to_string(), exp: 1},
                Token::new(atom.to_string(), 1),
            );
        }
    }
}
