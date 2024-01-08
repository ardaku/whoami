//! Crate for getting the user's username, realname and environment.
//!
//! ## Getting Started
//! Using the whoami crate is super easy!  All of the public items are simple
//! functions with no parameters that return [`String`]s or [`OsString`]s (with
//! the exception of [`desktop_env()`], [`platform()`], and [`arch()`], which
//! return enums, and [`lang()`] that returns an iterator of [`String`]s).  The
//! following example shows how to use all of the functions (except those that
//! return [`OsString`]):
//!
//! ```rust
//! println!(
//!     "User's Name            whoami::realname():    {}",
//!     whoami::realname(),
//! );
//! println!(
//!     "User's Username        whoami::username():    {}",
//!     whoami::username(),
//! );
//! println!(
//!     "User's Language        whoami::lang():        {:?}",
//!     whoami::lang().collect::<Vec<String>>(),
//! );
//! println!(
//!     "Device's Pretty Name   whoami::devicename():  {}",
//!     whoami::devicename(),
//! );
//! println!(
//!     "Device's Hostname      whoami::hostname():    {}",
//!     whoami::hostname(),
//! );
//! println!(
//!     "Device's Platform      whoami::platform():    {}",
//!     whoami::platform(),
//! );
//! println!(
//!     "Device's OS Distro     whoami::distro():      {}",
//!     whoami::distro(),
//! );
//! println!(
//!     "Device's Desktop Env.  whoami::desktop_env(): {}",
//!     whoami::desktop_env(),
//! );
//! println!(
//!     "Device's CPU Arch      whoami::arch():        {}",
//!     whoami::arch(),
//! );
//! ```

#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences,
    unsafe_code
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/ardaku/whoami/stable/res/icon.svg"
)]

const DEFAULT_USERNAME: &str = "Unknown";
const DEFAULT_HOSTNAME: &str = "LocalHost";

pub mod fallible;

#[allow(unsafe_code)]
// Unix
#[cfg_attr(
    not(any(target_os = "windows", target_arch = "wasm32")),
    path = "unix.rs"
)]
// Wasm32 (Daku) - FIXME: Currently routes to fake.rs
#[cfg_attr(all(target_arch = "wasm32", target_os = "daku"), path = "fake.rs")]
// Wasm32 (Wasi) - FIXME: Currently routes to fake.rs
#[cfg_attr(all(target_arch = "wasm32", target_os = "wasi"), path = "fake.rs")]
// Wasm32 (Web)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        feature = "web",
    ),
    path = "web.rs"
)]
// Wasm32 (Unknown)
#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(target_os = "daku"),
        not(feature = "web"),
    ),
    path = "fake.rs"
)]
// Windows
#[cfg_attr(
    all(target_os = "windows", not(target_arch = "wasm32")),
    path = "windows.rs"
)]
mod platform;

use std::{
    ffi::OsString,
    fmt::{self, Display, Formatter},
    io::{Error, ErrorKind},
    env,                    // for environment variables (LANG, LC_ALL)
};

/// output : en_US.UTF-8
/// en     : language
/// US     : country
#[allow(dead_code)]
fn get_language_and_country() -> String {
    if let Ok(language) = env::var("LC_ALL") {
        // possible output : en_US.UTF-8
        return language;
    } else if let Ok(language) = env::var("LANG") {
        // possible output : en_US.UTF-8
        return language;
    } else if let Ok(language) = env::var("LANGUAGE") {
        // possible output : en_US.UTF-8
        return language;
    }
    String::new()
}


/// This crate's convenience type alias for [`Result`](std::result::Result)s
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Region code for a [`Language`] dialect
///
/// Uses <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
///
/// https://www.suny.edu/media/suny/content-assets/documents/international-student/InternationalCountryCodes.pdf
/// https://www.iban.com/country-codes
#[non_exhaustive]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Region {
    // FIXME: V2: u32::from_ne_bytes for region codes, with `\0` for unused
    // FIXME: Add aliases up to 3-4 letters, but hidden
    /// Any dialect
    Any,

    /// an u32 code for region
    #[doc(hidden)]
    Custom(u32),

    /// `AF`: Afghanistan
    #[doc(hidden)]
    Af,

    /// `AL`: Albania
    #[doc(hidden)]
    Al,

    /// `DZ`: Algeria
    #[doc(hidden)]
    Dz,

    /// `AS`: American Samoa
    #[doc(hidden)]
    As,

    /// `AD`: Andorra
    #[doc(hidden)]
    Ad,

    /// `AO`: Angola
    #[doc(hidden)]
    Ao,

    /// `AI`: Anguilla
    #[doc(hidden)]
    Ai,

    /// `AQ`: Antarctica
    #[doc(hidden)]
    Aq,

    /// `AG`: Antigua and Barbuda
    #[doc(hidden)]
    Ag,

    /// `AR`: Argentina
    #[doc(hidden)]
    Ar,

    /// `AM`: Armenia
    #[doc(hidden)]
    Am,

    /// `AW`: Aruba
    #[doc(hidden)]
    Aw,

    /// `AU`: Australia
    #[doc(hidden)]
    Au,

    /// `AT`: Austria
    #[doc(hidden)]
    At,

    /// `AZ`: Azerbaijan
    #[doc(hidden)]
    Az,

    /// `BS`: Bahamas (the)
    #[doc(hidden)]
    Bs,

    /// `BH`: Bahrain
    #[doc(hidden)]
    Bh,

    /// `BD`: Bangladesh
    #[doc(hidden)]
    Bd,

    /// `BB`: Barbados
    #[doc(hidden)]
    Bb,

    /// `BY`: Belarus
    #[doc(hidden)]
    By,

    /// `BE`: Belgium
    #[doc(hidden)]
    Be,

    /// `BZ`: Belize
    #[doc(hidden)]
    Bz,

    /// `BJ`: Benin
    #[doc(hidden)]
    Bj,

    /// `BM`: Bermuda
    #[doc(hidden)]
    Bm,

    /// `BT`: Bhutan
    #[doc(hidden)]
    Bt,

    /// `BO`: Bolivia (Plurinational State of)
    #[doc(hidden)]
    Bo,

    /// `BQ`: Bonaire, Sint Eustatius and Saba
    #[doc(hidden)]
    Bq,

    /// `BA`: Bosnia and Herzegovina
    #[doc(hidden)]
    Ba,

    /// `BW`: Botswana
    #[doc(hidden)]
    Bw,

    /// `BV`: Bouvet Island
    #[doc(hidden)]
    Bv,

    /// `BR`: Brazil
    #[doc(hidden)]
    Br,

    /// `IO`: British Indian Ocean Territory (the)
    #[doc(hidden)]
    Io,

    /// `BN`: Brunei Darussalam
    #[doc(hidden)]
    Bn,

    /// `BG`: Bulgaria
    #[doc(hidden)]
    Bg,

    /// `BF`: Burkina Faso
    #[doc(hidden)]
    Bf,

    /// `BI`: Burundi
    #[doc(hidden)]
    Bi,

    /// `CV`: Cabo Verde
    #[doc(hidden)]
    Cv,

    /// `KH`: Cambodia
    #[doc(hidden)]
    Kh,

    /// `CM`: Cameroon
    #[doc(hidden)]
    Cm,

    /// `CA`: Canada
    #[doc(hidden)]
    Ca,

    /// `KY`: Cayman Islands (the)
    #[doc(hidden)]
    Ky,

    /// `CF`: Central African Republic (the)
    #[doc(hidden)]
    Cf,

    /// `TD`: Chad
    #[doc(hidden)]
    Td,

    /// `CL`: Chile
    #[doc(hidden)]
    Cl,

    /// `CN`: China
    #[doc(hidden)]
    Cn,

    /// `CX`: Christmas Island
    #[doc(hidden)]
    Cx,

    /// `CC`: Cocos (Keeling) Islands (the)
    #[doc(hidden)]
    Cc,

    /// `CO`: Colombia
    #[doc(hidden)]
    Co,

    /// `KM`: Comoros (the)
    #[doc(hidden)]
    Km,

    /// `CD`: Congo (the Democratic Republic of the)
    #[doc(hidden)]
    Cd,

    /// `CG`: Congo (the)
    #[doc(hidden)]
    Cg,

    /// `CK`: Cook Islands (the)
    #[doc(hidden)]
    Ck,

    /// `CR`: Costa Rica
    #[doc(hidden)]
    Cr,

    /// `HR`: Croatia
    #[doc(hidden)]
    Hr,

    /// `CU`: Cuba
    #[doc(hidden)]
    Cu,

    /// `CW`: Curaçao
    #[doc(hidden)]
    Cw,

    /// `CY`: Cyprus
    #[doc(hidden)]
    Cy,

    /// `CZ`: Czechia
    #[doc(hidden)]
    Cz,

    /// `CI`: Côte d'Ivoire
    #[doc(hidden)]
    Ci,

    /// `DK`: Denmark
    #[doc(hidden)]
    Dk,

    /// `DJ`: Djibouti
    #[doc(hidden)]
    Dj,

    /// `DM`: Dominica
    #[doc(hidden)]
    Dm,

    /// `DO`: Dominican Republic (the)
    #[doc(hidden)]
    Do,

    /// `EC`: Ecuador
    #[doc(hidden)]
    Ec,

    /// `EG`: Egypt
    #[doc(hidden)]
    Eg,

    /// `SV`: El Salvador
    #[doc(hidden)]
    Sv,

    /// `GQ`: Equatorial Guinea
    #[doc(hidden)]
    Gq,

    /// `ER`: Eritrea
    #[doc(hidden)]
    Er,

    /// `EE`: Estonia
    #[doc(hidden)]
    Ee,

    /// `SZ`: Eswatini
    #[doc(hidden)]
    Sz,

    /// `ET`: Ethiopia
    #[doc(hidden)]
    Et,

    /// `FK`: Falkland Islands (the) [Malvinas]
    #[doc(hidden)]
    Fk,

    /// `FO`: Faroe Islands (the)
    #[doc(hidden)]
    Fo,

    /// `FJ`: Fiji
    #[doc(hidden)]
    Fj,

    /// `FI`: Finland
    #[doc(hidden)]
    Fi,

    /// `FR`: France
    #[doc(hidden)]
    Fr,

    /// `GF`: French Guiana
    #[doc(hidden)]
    Gf,

    /// `PF`: French Polynesia
    #[doc(hidden)]
    Pf,

    /// `TF`: French Southern Territories (the)
    #[doc(hidden)]
    Tf,

    /// `GA`: Gabon
    #[doc(hidden)]
    Ga,

    /// `GM`: Gambia (the)
    #[doc(hidden)]
    Gm,

    /// `GE`: Georgia
    #[doc(hidden)]
    Ge,

    /// `DE`: Germany
    #[doc(hidden)]
    De,

    /// `GH`: Ghana
    #[doc(hidden)]
    Gh,

    /// `GI`: Gibraltar
    #[doc(hidden)]
    Gi,

    /// `GR`: Greece
    #[doc(hidden)]
    Gr,

    /// `GL`: Greenland
    #[doc(hidden)]
    Gl,

    /// `GD`: Grenada
    #[doc(hidden)]
    Gd,

    /// `GP`: Guadeloupe
    #[doc(hidden)]
    Gp,

    /// `GU`: Guam
    #[doc(hidden)]
    Gu,

    /// `GT`: Guatemala
    #[doc(hidden)]
    Gt,

    /// `GG`: Guernsey
    #[doc(hidden)]
    Gg,

    /// `GN`: Guinea
    #[doc(hidden)]
    Gn,

    /// `GW`: Guinea-Bissau
    #[doc(hidden)]
    Gw,

    /// `GY`: Guyana
    #[doc(hidden)]
    Gy,

    /// `HT`: Haiti
    #[doc(hidden)]
    Ht,

    /// `HM`: Heard Island and McDonald Islands
    #[doc(hidden)]
    Hm,

    /// `VA`: Holy See (the)
    #[doc(hidden)]
    Va,

    /// `HN`: Honduras
    #[doc(hidden)]
    Hn,

    /// `HK`: Hong Kong
    #[doc(hidden)]
    Hk,

    /// `HU`: Hungary
    #[doc(hidden)]
    Hu,

    /// `IS`: Iceland
    #[doc(hidden)]
    Is,

    /// `IN`: India
    #[doc(hidden)]
    In,

    /// `ID`: Indonesia
    #[doc(hidden)]
    Id,

    /// `IR`: Iran (Islamic Republic of)
    #[doc(hidden)]
    Ir,

    /// `IQ`: Iraq
    #[doc(hidden)]
    Iq,

    /// `IE`: Ireland
    #[doc(hidden)]
    Ie,

    /// `IM`: Isle of Man
    #[doc(hidden)]
    Im,

    /// `IL`: Israel
    #[doc(hidden)]
    Il,

    /// `IT`: Italy
    #[doc(hidden)]
    It,

    /// `JM`: Jamaica
    #[doc(hidden)]
    Jm,

    /// `JP`: Japan
    #[doc(hidden)]
    Jp,

    /// `JE`: Jersey
    #[doc(hidden)]
    Je,

    /// `JO`: Jordan
    #[doc(hidden)]
    Jo,

    /// `KZ`: Kazakhstan
    #[doc(hidden)]
    Kz,

    /// `KE`: Kenya
    #[doc(hidden)]
    Ke,

    /// `KI`: Kiribati
    #[doc(hidden)]
    Ki,

    /// `KP`: Korea (the Democratic People's Republic of)
    #[doc(hidden)]
    Kp,

    /// `KR`: Korea (the Republic of)
    #[doc(hidden)]
    Kr,

    /// `KW`: Kuwait
    #[doc(hidden)]
    Kw,

    /// `KG`: Kyrgyzstan
    #[doc(hidden)]
    Kg,

    /// `LA`: Lao People's Democratic Republic (the)
    #[doc(hidden)]
    La,

    /// `LV`: Latvia
    #[doc(hidden)]
    Lv,

    /// `LB`: Lebanon
    #[doc(hidden)]
    Lb,

    /// `LS`: Lesotho
    #[doc(hidden)]
    Ls,

    /// `LR`: Liberia
    #[doc(hidden)]
    Lr,

    /// `LY`: Libya
    #[doc(hidden)]
    Ly,

    /// `LI`: Liechtenstein
    #[doc(hidden)]
    Li,

    /// `LT`: Lithuania
    #[doc(hidden)]
    Lt,

    /// `LU`: Luxembourg
    #[doc(hidden)]
    Lu,

    /// `MO`: Macao
    #[doc(hidden)]
    Mo,

    /// `MG`: Madagascar
    #[doc(hidden)]
    Mg,

    /// `MW`: Malawi
    #[doc(hidden)]
    Mw,

    /// `MY`: Malaysia
    #[doc(hidden)]
    My,

    /// `MV`: Maldives
    #[doc(hidden)]
    Mv,

    /// `ML`: Mali
    #[doc(hidden)]
    Ml,

    /// `MT`: Malta
    #[doc(hidden)]
    Mt,

    /// `MH`: Marshall Islands (the)
    #[doc(hidden)]
    Mh,

    /// `MQ`: Martinique
    #[doc(hidden)]
    Mq,

    /// `MR`: Mauritania
    #[doc(hidden)]
    Mr,

    /// `MU`: Mauritius
    #[doc(hidden)]
    Mu,

    /// `YT`: Mayotte
    #[doc(hidden)]
    Yt,

    /// `MX`: Mexico
    #[doc(hidden)]
    Mx,

    /// `FM`: Micronesia (Federated States of)
    #[doc(hidden)]
    Fm,

    /// `MD`: Moldova (the Republic of)
    #[doc(hidden)]
    Md,

    /// `MC`: Monaco
    #[doc(hidden)]
    Mc,

    /// `MN`: Mongolia
    #[doc(hidden)]
    Mn,

    /// `ME`: Montenegro
    #[doc(hidden)]
    Me,

    /// `MS`: Montserrat
    #[doc(hidden)]
    Ms,

    /// `MA`: Morocco
    #[doc(hidden)]
    Ma,

    /// `MZ`: Mozambique
    #[doc(hidden)]
    Mz,

    /// `MM`: Myanmar
    #[doc(hidden)]
    Mm,

    /// `NA`: Namibia
    #[doc(hidden)]
    Na,

    /// `NR`: Nauru
    #[doc(hidden)]
    Nr,

    /// `NP`: Nepal
    #[doc(hidden)]
    Np,

    /// `NL`: Netherlands (the)
    #[doc(hidden)]
    Nl,

    /// `NC`: New Caledonia
    #[doc(hidden)]
    Nc,

    /// `NZ`: New Zealand
    #[doc(hidden)]
    Nz,

    /// `NI`: Nicaragua
    #[doc(hidden)]
    Ni,

    /// `NE`: Niger (the)
    #[doc(hidden)]
    Ne,

    /// `NG`: Nigeria
    #[doc(hidden)]
    Ng,

    /// `NU`: Niue
    #[doc(hidden)]
    Nu,

    /// `NF`: Norfolk Island
    #[doc(hidden)]
    Nf,

    /// `MP`: Northern Mariana Islands (the)
    #[doc(hidden)]
    Mp,

    /// `NO`: Norway
    #[doc(hidden)]
    No,

    /// `OM`: Oman
    #[doc(hidden)]
    Om,

    /// `PK`: Pakistan
    #[doc(hidden)]
    Pk,

    /// `PW`: Palau
    #[doc(hidden)]
    Pw,

    /// `PS`: Palestine, State of
    #[doc(hidden)]
    Ps,

    /// `PA`: Panama
    #[doc(hidden)]
    Pa,

    /// `PG`: Papua New Guinea
    #[doc(hidden)]
    Pg,

    /// `PY`: Paraguay
    #[doc(hidden)]
    Py,

    /// `PE`: Peru
    #[doc(hidden)]
    Pe,

    /// `PH`: Philippines (the)
    #[doc(hidden)]
    Ph,

    /// `PN`: Pitcairn
    #[doc(hidden)]
    Pn,

    /// `PL`: Poland
    #[doc(hidden)]
    Pl,

    /// `PT`: Portugal
    #[doc(hidden)]
    Pt,

    /// `PR`: Puerto Rico
    #[doc(hidden)]
    Pr,

    /// `QA`: Qatar
    #[doc(hidden)]
    Qa,

    /// `MK`: Republic of North Macedonia
    #[doc(hidden)]
    Mk,

    /// `RO`: Romania
    #[doc(hidden)]
    Ro,

    /// `RU`: Russian Federation (the)
    #[doc(hidden)]
    Ru,

    /// `RW`: Rwanda
    #[doc(hidden)]
    Rw,

    /// `RE`: Réunion
    #[doc(hidden)]
    Re,

    /// `BL`: Saint Barthélemy
    #[doc(hidden)]
    Bl,

    /// `SH`: Saint Helena, Ascension and Tristan da Cunha
    #[doc(hidden)]
    Sh,

    /// `KN`: Saint Kitts and Nevis
    #[doc(hidden)]
    Kn,

    /// `LC`: Saint Lucia
    #[doc(hidden)]
    Lc,

    /// `MF`: Saint Martin (French part)
    #[doc(hidden)]
    Mf,

    /// `PM`: Saint Pierre and Miquelon
    #[doc(hidden)]
    Pm,

    /// `VC`: Saint Vincent and the Grenadines
    #[doc(hidden)]
    Vc,

    /// `WS`: Samoa
    #[doc(hidden)]
    Ws,

    /// `SM`: San Marino
    #[doc(hidden)]
    Sm,

    /// `ST`: Sao Tome and Principe
    #[doc(hidden)]
    St,

    /// `SA`: Saudi Arabia
    #[doc(hidden)]
    Sa,

    /// `SN`: Senegal
    #[doc(hidden)]
    Sn,

    /// `RS`: Serbia
    #[doc(hidden)]
    Rs,

    /// `SC`: Seychelles
    #[doc(hidden)]
    Sc,

    /// `SL`: Sierra Leone
    #[doc(hidden)]
    Sl,

    /// `SG`: Singapore
    #[doc(hidden)]
    Sg,

    /// `SX`: Sint Maarten (Dutch part)
    #[doc(hidden)]
    Sx,

    /// `SK`: Slovakia
    #[doc(hidden)]
    Sk,

    /// `SI`: Slovenia
    #[doc(hidden)]
    Si,

    /// `SB`: Solomon Islands
    #[doc(hidden)]
    Sb,

    /// `SO`: Somalia
    #[doc(hidden)]
    So,

    /// `ZA`: South Africa
    #[doc(hidden)]
    Za,

    /// `GS`: South Georgia and the South Sandwich Islands
    #[doc(hidden)]
    Gs,

    /// `SS`: South Sudan
    #[doc(hidden)]
    Ss,

    /// `ES`: Spain
    #[doc(hidden)]
    Es,

    /// `LK`: Sri Lanka
    #[doc(hidden)]
    Lk,

    /// `SD`: Sudan (the)
    #[doc(hidden)]
    Sd,

    /// `SR`: Suriname
    #[doc(hidden)]
    Sr,

    /// `SJ`: Svalbard and Jan Mayen
    #[doc(hidden)]
    Sj,

    /// `SE`: Sweden
    #[doc(hidden)]
    Se,

    /// `CH`: Switzerland
    #[doc(hidden)]
    Ch,

    /// `SY`: Syrian Arab Republic
    #[doc(hidden)]
    Sy,

    /// `TW`: Taiwan (Province of China)
    #[doc(hidden)]
    Tw,

    /// `TJ`: Tajikistan
    #[doc(hidden)]
    Tj,

    /// `TZ`: Tanzania, United Republic of
    #[doc(hidden)]
    Tz,

    /// `TH`: Thailand
    #[doc(hidden)]
    Th,

    /// `TL`: Timor-Leste
    #[doc(hidden)]
    Tl,

    /// `TG`: Togo
    #[doc(hidden)]
    Tg,

    /// `TK`: Tokelau
    #[doc(hidden)]
    Tk,

    /// `TO`: Tonga
    #[doc(hidden)]
    To,

    /// `TT`: Trinidad and Tobago
    #[doc(hidden)]
    Tt,

    /// `TN`: Tunisia
    #[doc(hidden)]
    Tn,

    /// `TR`: Turkey
    #[doc(hidden)]
    Tr,

    /// `TM`: Turkmenistan
    #[doc(hidden)]
    Tm,

    /// `TC`: Turks and Caicos Islands (the)
    #[doc(hidden)]
    Tc,

    /// `TV`: Tuvalu
    #[doc(hidden)]
    Tv,

    /// `UG`: Uganda
    #[doc(hidden)]
    Ug,

    /// `UA`: Ukraine
    #[doc(hidden)]
    Ua,

    /// `AE`: United Arab Emirates (the)
    #[doc(hidden)]
    Ae,

    /// `GB`: United Kingdom of Great Britain and Northern Ireland (the)
    #[doc(hidden)]
    Gb,

    /// `UM`: United States Minor Outlying Islands (the)
    #[doc(hidden)]
    Um,

    /// `US`: United States of America (the)
    #[doc(hidden)]
    Us,

    /// `UY`: Uruguay
    #[doc(hidden)]
    Uy,

    /// `UZ`: Uzbekistan
    #[doc(hidden)]
    Uz,

    /// `VU`: Vanuatu
    #[doc(hidden)]
    Vu,

    /// `VE`: Venezuela (Bolivarian Republic of)
    #[doc(hidden)]
    Ve,

    /// `VN`: Viet Nam
    #[doc(hidden)]
    Vn,

    /// `VG`: Virgin Islands (British)
    #[doc(hidden)]
    Vg,

    /// `VI`: Virgin Islands (U.S.)
    #[doc(hidden)]
    Vi,

    /// `WF`: Wallis and Futuna
    #[doc(hidden)]
    Wf,

    /// `EH`: Western Sahara
    #[doc(hidden)]
    Eh,

    /// `YE`: Yemen
    #[doc(hidden)]
    Ye,

    /// `ZM`: Zambia
    #[doc(hidden)]
    Zm,

    /// `ZW`: Zimbabwe
    #[doc(hidden)]
    Zw,

    /// `AX`: Åland Islands
    #[doc(hidden)]
    Ax
}


impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            // Self is a instance of Region
            Self::Any => "**",

            // https://www.iban.com/country-codes
            Region::Custom(value) => {
                match value {
                    4   => "AF",		// Afghanistan
                    8   => "AL",		// Albania
                    12  => "DZ",		// Algeria
                    16  => "AS",		// American Samoa
                    20  => "AD",		// Andorra
                    24  => "AO",		// Angola
                    660 => "AI",		// Anguilla
                    10  => "AQ",		// Antarctica
                    28  => "AG",		// Antigua and Barbuda
                    32  => "AR",		// Argentina
                    51  => "AM",		// Armenia
                    533 => "AW",		// Aruba
                    36  => "AU",		// Australia
                    40  => "AT",		// Austria
                    31  => "AZ",		// Azerbaijan
                    44  => "BS",		// Bahamas (the)
                    48  => "BH",		// Bahrain
                    50  => "BD",		// Bangladesh
                    52  => "BB",		// Barbados
                    112 => "BY",		// Belarus
                    56  => "BE",		// Belgium
                    84  => "BZ",		// Belize
                    204 => "BJ",		// Benin
                    60  => "BM",		// Besrmuda
                    64  => "BT",		// Bhutan
                    68  => "BO",		// Bolivia (Plurinational State of)
                    535 => "BQ",		// Bonaire, Sint Eustatius and Saba
                    70  => "BA",		// Bosnia and Herzegovina
                    72  => "BW",		// Botswana
                    74  => "BV",		// Bouvet Island
                    76  => "BR",		// Brazil
                    86  => "IO",		// British Indian Ocean Territory (the)
                    96  => "BN",		// Brunei Darussalam
                    100 => "BG",		// Bulgaria
                    854 => "BF",		// Burkina Faso
                    108 => "BI",		// Burundi
                    132 => "CV",		// Cabo Verde
                    116 => "KH",		// Cambodia
                    120 => "CM",		// Cameroon
                    124 => "CA",		// Canada
                    136 => "KY",		// Cayman Islands (the)
                    140 => "CF",		// Central African Republic (the)
                    148 => "TD",		// Chad
                    152 => "CL",		// Chile
                    156 => "CN",		// China
                    162 => "CX",		// Christmas Island
                    166 => "CC",		// Cocos (Keeling) Islands (the)
                    170 => "CO",		// Colombia
                    174 => "KM",		// Comoros (the)
                    180 => "CD",		// Congo (the Democratic Republic of the)
                    178 => "CG",		// Congo (the)
                    184 => "CK",		// Cook Islands (the)
                    188 => "CR",		// Costa Rica
                    191 => "HR",		// Croatia
                    192 => "CU",		// Cuba
                    531 => "CW",		// Curaçao
                    196 => "CY",		// Cyprus
                    203 => "CZ",		// Czechia
                    384 => "CI",		// Côte d'Ivoire
                    208 => "DK",		// Denmark
                    262 => "DJ",		// Djibouti
                    212 => "DM",		// Dominica
                    214 => "DO",		// Dominican Republic (the)
                    218 => "EC",		// Ecuador
                    818 => "EG",		// Egypt
                    222 => "SV",		// El Salvador
                    226 => "GQ",		// Equatorial Guinea
                    232 => "ER",		// Eritrea
                    233 => "EE",		// Estonia
                    748 => "SZ",		// Eswatini
                    231 => "ET",		// Ethiopia
                    238 => "FK",		// Falkland Islands (the) [Malvinas]
                    234 => "FO",		// Faroe Islands (the)
                    242 => "FJ",		// Fiji
                    246 => "FI",		// Finland
                    250 => "FR",		// France
                    254 => "GF",		// French Guiana
                    258 => "PF",		// French Polynesia
                    260 => "TF",		// French Southern Territories (the)
                    266 => "GA",		// Gabon
                    270 => "GM",		// Gambia (the)
                    268 => "GE",		// Georgia
                    276 => "DE",		// Germany
                    288 => "GH",		// Ghana
                    292 => "GI",		// Gibraltar
                    300 => "GR",		// Greece
                    304 => "GL",		// Greenland
                    308 => "GD",		// Grenada
                    312 => "GP",		// Guadeloupe
                    316 => "GU",		// Guam
                    320 => "GT",		// Guatemala
                    831 => "GG",		// Guernsey
                    324 => "GN",		// Guinea
                    624 => "GW",		// Guinea-Bissau
                    328 => "GY",		// Guyana
                    332 => "HT",		// Haiti
                    334 => "HM",		// Heard Island and McDonald Islands
                    336 => "VA",		// Holy See (the)
                    340 => "HN",		// Honduras
                    344 => "HK",		// Hong Kong
                    348 => "HU",		// Hungary
                    352 => "IS",		// Iceland
                    356 => "IN",		// India
                    360 => "ID",		// Indonesia
                    364 => "IR",		// Iran (Islamic Republic of)
                    368 => "IQ",		// Iraq
                    372 => "IE",		// Ireland
                    833 => "IM",		// Isle of Man
                    376 => "IL",		// Israel
                    380 => "IT",		// Italy
                    388 => "JM",		// Jamaica
                    392 => "JP",		// Japan
                    832 => "JE",		// Jersey
                    400 => "JO",		// Jordan
                    398 => "KZ",		// Kazakhstan
                    404 => "KE",		// Kenya
                    296 => "KI",		// Kiribati
                    408 => "KP",		// Korea (the Democratic People's Republic of)
                    410 => "KR",		// Korea (the Republic of)
                    414 => "KW",		// Kuwait
                    417 => "KG",		// Kyrgyzstan
                    418 => "LA",		// Lao People's Democratic Republic (the)
                    428 => "LV",		// Latvia
                    422 => "LB",		// Lebanon
                    426 => "LS",		// Lesotho
                    430 => "LR",		// Liberia
                    434 => "LY",		// Libya
                    438 => "LI",		// Liechtenstein
                    440 => "LT",		// Lithuania
                    442 => "LU",		// Luxembourg
                    446 => "MO",		// Macao
                    450 => "MG",		// Madagascar
                    454 => "MW",		// Malawi
                    458 => "MY",		// Malaysia
                    462 => "MV",		// Maldives
                    466 => "ML",		// Mali
                    470 => "MT",		// Malta
                    584 => "MH",		// Marshall Islands (the)
                    474 => "MQ",		// Martinique
                    478 => "MR",		// Mauritania
                    480 => "MU",		// Mauritius
                    175 => "YT",		// Mayotte
                    484 => "MX",		// Mexico
                    583 => "FM",		// Micronesia (Federated States of)
                    498 => "MD",		// Moldova (the Republic of)
                    492 => "MC",		// Monaco
                    496 => "MN",		// Mongolia
                    499 => "ME",		// Montenegro
                    500 => "MS",		// Montserrat
                    504 => "MA",		// Morocco
                    508 => "MZ",		// Mozambique
                    104 => "MM",		// Myanmar
                    516 => "NA",		// Namibia
                    520 => "NR",		// Nauru
                    524 => "NP",		// Nepal
                    528 => "NL",		// Netherlands (the)
                    540 => "NC",		// New Caledonia
                    554 => "NZ",		// New Zealand
                    558 => "NI",		// Nicaragua
                    562 => "NE",		// Niger (the)
                    566 => "NG",		// Nigeria
                    570 => "NU",		// Niue
                    574 => "NF",		// Norfolk Island
                    580 => "MP",		// Northern Mariana Islands (the)
                    578 => "NO",		// Norway
                    512 => "OM",		// Oman
                    586 => "PK",		// Pakistan
                    585 => "PW",		// Palau
                    275 => "PS",		// Palestine, State of
                    591 => "PA",		// Panama
                    598 => "PG",		// Papua New Guinea
                    600 => "PY",		// Paraguay
                    604 => "PE",		// Peru
                    608 => "PH",		// Philippines (the)
                    612 => "PN",		// Pitcairn
                    616 => "PL",		// Poland
                    620 => "PT",		// Portugal
                    630 => "PR",		// Puerto Rico
                    634 => "QA",		// Qatar
                    807 => "MK",		// Republic of North Macedonia
                    642 => "RO",		// Romania
                    643 => "RU",		// Russian Federation (the)
                    646 => "RW",		// Rwanda
                    638 => "RE",		// Réunion
                    652 => "BL",		// Saint Barthélemy
                    654 => "SH",		// Saint Helena, Ascension and Tristan da Cunha
                    659 => "KN",		// Saint Kitts and Nevis
                    662 => "LC",		// Saint Lucia
                    663 => "MF",		// Saint Martin (French part)
                    666 => "PM",		// Saint Pierre and Miquelon
                    670 => "VC",		// Saint Vincent and the Grenadines
                    882 => "WS",		// Samoa
                    674 => "SM",		// San Marino
                    678 => "ST",		// Sao Tome and Principe
                    682 => "SA",		// Saudi Arabia
                    686 => "SN",		// Senegal
                    688 => "RS",		// Serbia
                    690 => "SC",		// Seychelles
                    694 => "SL",		// Sierra Leone
                    702 => "SG",		// Singapore
                    534 => "SX",		// Sint Maarten (Dutch part)
                    703 => "SK",		// Slovakia
                    705 => "SI",		// Slovenia
                    90  => "SB",		// Solomon Islands
                    706 => "SO",		// Somalia
                    710 => "ZA",		// South Africa
                    239 => "GS",		// South Georgia and the South Sandwich Islands
                    728 => "SS",		// South Sudan
                    724 => "ES",		// Spain
                    144 => "LK",		// Sri Lanka
                    729 => "SD",		// Sudan (the)
                    740 => "SR",		// Suriname
                    744 => "SJ",		// Svalbard and Jan Mayen
                    752 => "SE",		// Sweden
                    756 => "CH",		// Switzerland
                    760 => "SY",		// Syrian Arab Republic
                    158 => "TW",		// Taiwan (Province of China)
                    762 => "TJ",		// Tajikistan
                    834 => "TZ",		// Tanzania, United Republic of
                    764 => "TH",		// Thailand
                    626 => "TL",		// Timor-Leste
                    768 => "TG",		// Togo
                    772 => "TK",		// Tokelau
                    776 => "TO",		// Tonga
                    780 => "TT",		// Trinidad and Tobago
                    788 => "TN",		// Tunisia
                    792 => "TR",		// Turkey
                    795 => "TM",		// Turkmenistan
                    796 => "TC",		// Turks and Caicos Islands (the)
                    798 => "TV",		// Tuvalu
                    800 => "UG",		// Uganda
                    804 => "UA",		// Ukraine
                    784 => "AE",		// United Arab Emirates (the)
                    826 => "GB",		// United Kingdom of Great Britain and Northern Ireland (the)
                    581 => "UM",		// United States Minor Outlying Islands (the)
                    840 => "US",		// United States of America (the)
                    858 => "UY",		// Uruguay
                    860 => "UZ",		// Uzbekistan
                    548 => "VU",		// Vanuatu
                    862 => "VE",		// Venezuela (Bolivarian Republic of)
                    704 => "VN",		// Viet Nam
                    92  => "VG",		// Virgin Islands (British)
                    850 => "VI",		// Virgin Islands (U.S.)
                    876 => "WF",		// Wallis and Futuna
                    732 => "EH",		// Western Sahara
                    887 => "YE",		// Yemen
                    894 => "ZM",		// Zambia
                    716 => "ZW",		// Zimbabwe
                    248 => "AX",		// Åland Islands
                    _ => "**", // unknown for other cases
                }
            }

            // https://www.iban.com/country-codes
            Self::Af => "AF",			// Afghanistan
            Self::Al => "AL",			// Albania
            Self::Dz => "DZ",			// Algeria
            Self::As => "AS",			// American Samoa
            Self::Ad => "AD",			// Andorra
            Self::Ao => "AO",			// Angola
            Self::Ai => "AI",			// Anguilla
            Self::Aq => "AQ",			// Antarctica
            Self::Ag => "AG",			// Antigua and Barbuda
            Self::Ar => "AR",			// Argentina
            Self::Am => "AM",			// Armenia
            Self::Aw => "AW",			// Aruba
            Self::Au => "AU",			// Australia
            Self::At => "AT",			// Austria
            Self::Az => "AZ",			// Azerbaijan
            Self::Bs => "BS",			// Bahamas (the)
            Self::Bh => "BH",			// Bahrain
            Self::Bd => "BD",			// Bangladesh
            Self::Bb => "BB",			// Barbados
            Self::By => "BY",			// Belarus
            Self::Be => "BE",			// Belgium
            Self::Bz => "BZ",			// Belize
            Self::Bj => "BJ",			// Benin
            Self::Bm => "BM",			// Bermuda
            Self::Bt => "BT",			// Bhutan
            Self::Bo => "BO",			// Bolivia (Plurinational State of)
            Self::Bq => "BQ",			// Bonaire, Sint Eustatius and Saba
            Self::Ba => "BA",			// Bosnia and Herzegovina
            Self::Bw => "BW",			// Botswana
            Self::Bv => "BV",			// Bouvet Island
            Self::Br => "BR",			// Brazil
            Self::Io => "IO",			// British Indian Ocean Territory (the)
            Self::Bn => "BN",			// Brunei Darussalam
            Self::Bg => "BG",			// Bulgaria
            Self::Bf => "BF",			// Burkina Faso
            Self::Bi => "BI",			// Burundi
            Self::Cv => "CV",			// Cabo Verde
            Self::Kh => "KH",			// Cambodia
            Self::Cm => "CM",			// Cameroon
            Self::Ca => "CA",			// Canada
            Self::Ky => "KY",			// Cayman Islands (the)
            Self::Cf => "CF",			// Central African Republic (the)
            Self::Td => "TD",			// Chad
            Self::Cl => "CL",			// Chile
            Self::Cn => "CN",			// China
            Self::Cx => "CX",			// Christmas Island
            Self::Cc => "CC",			// Cocos (Keeling) Islands (the)
            Self::Co => "CO",			// Colombia
            Self::Km => "KM",			// Comoros (the)
            Self::Cd => "CD",			// Congo (the Democratic Republic of the)
            Self::Cg => "CG",			// Congo (the)
            Self::Ck => "CK",			// Cook Islands (the)
            Self::Cr => "CR",			// Costa Rica
            Self::Hr => "HR",			// Croatia
            Self::Cu => "CU",			// Cuba
            Self::Cw => "CW",			// Curaçao
            Self::Cy => "CY",			// Cyprus
            Self::Cz => "CZ",			// Czechia
            Self::Ci => "CI",			// Côte d'Ivoire
            Self::Dk => "DK",			// Denmark
            Self::Dj => "DJ",			// Djibouti
            Self::Dm => "DM",			// Dominica
            Self::Do => "DO",			// Dominican Republic (the)
            Self::Ec => "EC",			// Ecuador
            Self::Eg => "EG",			// Egypt
            Self::Sv => "SV",			// El Salvador
            Self::Gq => "GQ",			// Equatorial Guinea
            Self::Er => "ER",			// Eritrea
            Self::Ee => "EE",			// Estonia
            Self::Sz => "SZ",			// Eswatini
            Self::Et => "ET",			// Ethiopia
            Self::Fk => "FK",			// Falkland Islands (the) [Malvinas]
            Self::Fo => "FO",			// Faroe Islands (the)
            Self::Fj => "FJ",			// Fiji
            Self::Fi => "FI",			// Finland
            Self::Fr => "FR",			// France
            Self::Gf => "GF",			// French Guiana
            Self::Pf => "PF",			// French Polynesia
            Self::Tf => "TF",			// French Southern Territories (the)
            Self::Ga => "GA",			// Gabon
            Self::Gm => "GM",			// Gambia (the)
            Self::Ge => "GE",			// Georgia
            Self::De => "DE",			// Germany
            Self::Gh => "GH",			// Ghana
            Self::Gi => "GI",			// Gibraltar
            Self::Gr => "GR",			// Greece
            Self::Gl => "GL",			// Greenland
            Self::Gd => "GD",			// Grenada
            Self::Gp => "GP",			// Guadeloupe
            Self::Gu => "GU",			// Guam
            Self::Gt => "GT",			// Guatemala
            Self::Gg => "GG",			// Guernsey
            Self::Gn => "GN",			// Guinea
            Self::Gw => "GW",			// Guinea-Bissau
            Self::Gy => "GY",			// Guyana
            Self::Ht => "HT",			// Haiti
            Self::Hm => "HM",			// Heard Island and McDonald Islands
            Self::Va => "VA",			// Holy See (the)
            Self::Hn => "HN",			// Honduras
            Self::Hk => "HK",			// Hong Kong
            Self::Hu => "HU",			// Hungary
            Self::Is => "IS",			// Iceland
            Self::In => "IN",			// India
            Self::Id => "ID",			// Indonesia
            Self::Ir => "IR",			// Iran (Islamic Republic of)
            Self::Iq => "IQ",			// Iraq
            Self::Ie => "IE",			// Ireland
            Self::Im => "IM",			// Isle of Man
            Self::Il => "IL",			// Israel
            Self::It => "IT",			// Italy
            Self::Jm => "JM",			// Jamaica
            Self::Jp => "JP",			// Japan
            Self::Je => "JE",			// Jersey
            Self::Jo => "JO",			// Jordan
            Self::Kz => "KZ",			// Kazakhstan
            Self::Ke => "KE",			// Kenya
            Self::Ki => "KI",			// Kiribati
            Self::Kp => "KP",			// Korea (the Democratic People's Republic of)
            Self::Kr => "KR",			// Korea (the Republic of)
            Self::Kw => "KW",			// Kuwait
            Self::Kg => "KG",			// Kyrgyzstan
            Self::La => "LA",			// Lao People's Democratic Republic (the)
            Self::Lv => "LV",			// Latvia
            Self::Lb => "LB",			// Lebanon
            Self::Ls => "LS",			// Lesotho
            Self::Lr => "LR",			// Liberia
            Self::Ly => "LY",			// Libya
            Self::Li => "LI",			// Liechtenstein
            Self::Lt => "LT",			// Lithuania
            Self::Lu => "LU",			// Luxembourg
            Self::Mo => "MO",			// Macao
            Self::Mg => "MG",			// Madagascar
            Self::Mw => "MW",			// Malawi
            Self::My => "MY",			// Malaysia
            Self::Mv => "MV",			// Maldives
            Self::Ml => "ML",			// Mali
            Self::Mt => "MT",			// Malta
            Self::Mh => "MH",			// Marshall Islands (the)
            Self::Mq => "MQ",			// Martinique
            Self::Mr => "MR",			// Mauritania
            Self::Mu => "MU",			// Mauritius
            Self::Yt => "YT",			// Mayotte
            Self::Mx => "MX",			// Mexico
            Self::Fm => "FM",			// Micronesia (Federated States of)
            Self::Md => "MD",			// Moldova (the Republic of)
            Self::Mc => "MC",			// Monaco
            Self::Mn => "MN",			// Mongolia
            Self::Me => "ME",			// Montenegro
            Self::Ms => "MS",			// Montserrat
            Self::Ma => "MA",			// Morocco
            Self::Mz => "MZ",			// Mozambique
            Self::Mm => "MM",			// Myanmar
            Self::Na => "NA",			// Namibia
            Self::Nr => "NR",			// Nauru
            Self::Np => "NP",			// Nepal
            Self::Nl => "NL",			// Netherlands (the)
            Self::Nc => "NC",			// New Caledonia
            Self::Nz => "NZ",			// New Zealand
            Self::Ni => "NI",			// Nicaragua
            Self::Ne => "NE",			// Niger (the)
            Self::Ng => "NG",			// Nigeria
            Self::Nu => "NU",			// Niue
            Self::Nf => "NF",			// Norfolk Island
            Self::Mp => "MP",			// Northern Mariana Islands (the)
            Self::No => "NO",			// Norway
            Self::Om => "OM",			// Oman
            Self::Pk => "PK",			// Pakistan
            Self::Pw => "PW",			// Palau
            Self::Ps => "PS",			// Palestine, State of
            Self::Pa => "PA",			// Panama
            Self::Pg => "PG",			// Papua New Guinea
            Self::Py => "PY",			// Paraguay
            Self::Pe => "PE",			// Peru
            Self::Ph => "PH",			// Philippines (the)
            Self::Pn => "PN",			// Pitcairn
            Self::Pl => "PL",			// Poland
            Self::Pt => "PT",			// Portugal
            Self::Pr => "PR",			// Puerto Rico
            Self::Qa => "QA",			// Qatar
            Self::Mk => "MK",			// Republic of North Macedonia
            Self::Ro => "RO",			// Romania
            Self::Ru => "RU",			// Russian Federation (the)
            Self::Rw => "RW",			// Rwanda
            Self::Re => "RE",			// Réunion
            Self::Bl => "BL",			// Saint Barthélemy
            Self::Sh => "SH",			// Saint Helena, Ascension and Tristan da Cunha
            Self::Kn => "KN",			// Saint Kitts and Nevis
            Self::Lc => "LC",			// Saint Lucia
            Self::Mf => "MF",			// Saint Martin (French part)
            Self::Pm => "PM",			// Saint Pierre and Miquelon
            Self::Vc => "VC",			// Saint Vincent and the Grenadines
            Self::Ws => "WS",			// Samoa
            Self::Sm => "SM",			// San Marino
            Self::St => "ST",			// Sao Tome and Principe
            Self::Sa => "SA",			// Saudi Arabia
            Self::Sn => "SN",			// Senegal
            Self::Rs => "RS",			// Serbia
            Self::Sc => "SC",			// Seychelles
            Self::Sl => "SL",			// Sierra Leone
            Self::Sg => "SG",			// Singapore
            Self::Sx => "SX",			// Sint Maarten (Dutch part)
            Self::Sk => "SK",			// Slovakia
            Self::Si => "SI",			// Slovenia
            Self::Sb => "SB",			// Solomon Islands
            Self::So => "SO",			// Somalia
            Self::Za => "ZA",			// South Africa
            Self::Gs => "GS",			// South Georgia and the South Sandwich Islands
            Self::Ss => "SS",			// South Sudan
            Self::Es => "ES",			// Spain
            Self::Lk => "LK",			// Sri Lanka
            Self::Sd => "SD",			// Sudan (the)
            Self::Sr => "SR",			// Suriname
            Self::Sj => "SJ",			// Svalbard and Jan Mayen
            Self::Se => "SE",			// Sweden
            Self::Ch => "CH",			// Switzerland
            Self::Sy => "SY",			// Syrian Arab Republic
            Self::Tw => "TW",			// Taiwan (Province of China)
            Self::Tj => "TJ",			// Tajikistan
            Self::Tz => "TZ",			// Tanzania, United Republic of
            Self::Th => "TH",			// Thailand
            Self::Tl => "TL",			// Timor-Leste
            Self::Tg => "TG",			// Togo
            Self::Tk => "TK",			// Tokelau
            Self::To => "TO",			// Tonga
            Self::Tt => "TT",			// Trinidad and Tobago
            Self::Tn => "TN",			// Tunisia
            Self::Tr => "TR",			// Turkey
            Self::Tm => "TM",			// Turkmenistan
            Self::Tc => "TC",			// Turks and Caicos Islands (the)
            Self::Tv => "TV",			// Tuvalu
            Self::Ug => "UG",			// Uganda
            Self::Ua => "UA",			// Ukraine
            Self::Ae => "AE",			// United Arab Emirates (the)
            Self::Gb => "GB",			// United Kingdom of Great Britain and Northern Ireland (the)
            Self::Um => "UM",			// United States Minor Outlying Islands (the)
            Self::Us => "US",			// United States of America (the)
            Self::Uy => "UY",			// Uruguay
            Self::Uz => "UZ",			// Uzbekistan
            Self::Vu => "VU",			// Vanuatu
            Self::Ve => "VE",			// Venezuela (Bolivarian Republic of)
            Self::Vn => "VN",			// Viet Nam
            Self::Vg => "VG",			// Virgin Islands (British)
            Self::Vi => "VI",			// Virgin Islands (U.S.)
            Self::Wf => "WF",			// Wallis and Futuna
            Self::Eh => "EH",			// Western Sahara
            Self::Ye => "YE",			// Yemen
            Self::Zm => "ZM",			// Zambia
            Self::Zw => "ZW",			// Zimbabwe
            Self::Ax => "AX",			// Åland Islands
        })
    }
}

/// the `function get_language_and_country` returns a String
/// `en_US.UTF-8`       : `language_country.text_encoding`
/// the two uppercase letters between the underscore `_` and `.UTF-8`
/// represent the `country`
impl Region {
    #[allow(dead_code)]
    fn get() -> Self {
        let var_env: String = get_language_and_country();
        // output : en_US.UTF-8
        // en     : language
        // US     : country (region)

        // the country is written in uppercase
        if var_env.is_empty() {
            return Region::Any;
        }

        get_region_helper()
    }
}

fn get_region_helper() -> Region {
    let var_env: String = get_language_and_country();

    if var_env.contains("AF") {
        return Region::Af;
    } else if var_env.contains("AL") {
        return Region::Al;
    } else if var_env.contains("DZ") {
        return Region::Dz;
    } else if var_env.contains("AS") {
        return Region::As;
    } else if var_env.contains("AD") {
        return Region::Ad;
    } else if var_env.contains("AO") {
        return Region::Ao;
    } else if var_env.contains("AI") {
        return Region::Ai;
    } else if var_env.contains("AQ") {
        return Region::Aq;
    } else if var_env.contains("AG") {
        return Region::Ag;
    } else if var_env.contains("AR") {
        return Region::Ar;
    } else if var_env.contains("AM") {
        return Region::Am;
    } else if var_env.contains("AW") {
        return Region::Aw;
    } else if var_env.contains("AU") {
        return Region::Au;
    } else if var_env.contains("AT") {
        return Region::At;
    } else if var_env.contains("AZ") {
        return Region::Az;
    } else if var_env.contains("BS") {
        return Region::Bs;
    } else if var_env.contains("BH") {
        return Region::Bh;
    } else if var_env.contains("BD") {
        return Region::Bd;
    } else if var_env.contains("BB") {
        return Region::Bb;
    } else if var_env.contains("BY") {
        return Region::By;
    } else if var_env.contains("BE") {
        return Region::Be;
    } else if var_env.contains("BZ") {
        return Region::Bz;
    } else if var_env.contains("BJ") {
        return Region::Bj;
    } else if var_env.contains("BM") {
        return Region::Bm;
    } else if var_env.contains("BT") {
        return Region::Bt;
    } else if var_env.contains("BO") {
        return Region::Bo;
    } else if var_env.contains("BQ") {
        return Region::Bq;
    } else if var_env.contains("BA") {
        return Region::Ba;
    } else if var_env.contains("BW") {
        return Region::Bw;
    } else if var_env.contains("BV") {
        return Region::Bv;
    } else if var_env.contains("BR") {
        return Region::Br;
    } else if var_env.contains("IO") {
        return Region::Io;
    } else if var_env.contains("BN") {
        return Region::Bn;
    } else if var_env.contains("BG") {
        return Region::Bg;
    } else if var_env.contains("BF") {
        return Region::Bf;
    } else if var_env.contains("BI") {
        return Region::Bi;
    } else if var_env.contains("CV") {
        return Region::Cv;
    } else if var_env.contains("KH") {
        return Region::Kh;
    } else if var_env.contains("CM") {
        return Region::Cm;
    } else if var_env.contains("CA") {
        return Region::Ca;
    } else if var_env.contains("KY") {
        return Region::Ky;
    } else if var_env.contains("CF") {
        return Region::Cf;
    } else if var_env.contains("TD") {
        return Region::Td;
    } else if var_env.contains("CL") {
        return Region::Cl;
    } else if var_env.contains("CN") {
        return Region::Cn;
    } else if var_env.contains("CX") {
        return Region::Cx;
    } else if var_env.contains("CC") {
        return Region::Cc;
    } else if var_env.contains("CO") {
        return Region::Cd;
    }
    second_get_region_helper(var_env)
}

fn second_get_region_helper(var_env: String) -> Region {
    if var_env.contains("KM") {
        return Region::Km;
    } else if var_env.contains("CD") {
        return Region::Cd;
    } else if var_env.contains("CG") {
        return Region::Cg;
    } else if var_env.contains("CK") {
        return Region::Ck;
    } else if var_env.contains("CR") {
        return Region::Cr;
    } else if var_env.contains("HR") {
        return Region::Hr;
    } else if var_env.contains("CU") {
        return Region::Cu;
    } else if var_env.contains("CW") {
        return Region::Cw;
    } else if var_env.contains("CY") {
        return Region::Cy;
    } else if var_env.contains("CZ") {
        return Region::Cz;
    } else if var_env.contains("CI") {
        return Region::Ci;
    } else if var_env.contains("DK") {
        return Region::Dk;
    } else if var_env.contains("DJ") {
        return Region::Dj;
    } else if var_env.contains("DM") {
        return Region::Dm;
    } else if var_env.contains("DO") {
        return Region::Do;
    } else if var_env.contains("EC") {
        return Region::Ec;
    } else if var_env.contains("EG") {
        return Region::Eg;
    } else if var_env.contains("SV") {
        return Region::Sv;
    } else if var_env.contains("GQ") {
        return Region::Gq;
    } else if var_env.contains("ER") {
        return Region::Er;
    } else if var_env.contains("EE") {
        return Region::Ee;
    } else if var_env.contains("SZ") {
        return Region::Sz;
    } else if var_env.contains("ET") {
        return Region::Et;
    } else if var_env.contains("FK") {
        return Region::Fk;
    }

    third_get_region_helper(var_env)
}

    fn third_get_region_helper(var_env: String) -> Region {
        if var_env.contains("FO") {
            return Region::Fo;
        } else if var_env.contains("FJ") {
            return Region::Fj;
        } else if var_env.contains("FI") {
            return Region::Fi;
        } else if var_env.contains("FR") {
            return Region::Fr;
        } else if var_env.contains("GF") {
            return Region::Gf;
        } else if var_env.contains("PF") {
            return Region::Pf;
        } else if var_env.contains("TF") {
            return Region::Tf;
        } else if var_env.contains("GA") {
            return Region::Ga;
        } else if var_env.contains("GM") {
            return Region::Gm;
        } else if var_env.contains("GE") {
            return Region::Ge;
        } else if var_env.contains("DE") {
            return Region::De;
        } else if var_env.contains("GH") {
            return Region::Gh;
        } else if var_env.contains("GI") {
            return Region::Gi;
        } else if var_env.contains("GR") {
            return Region::Gr;
        } else if var_env.contains("GL") {
            return Region::Gl;
        } else if var_env.contains("GD") {
            return Region::Gd;
        } else if var_env.contains("GP") {
            return Region::Gp;
        } else if var_env.contains("GU") {
            return Region::Gu;
        } else if var_env.contains("GT") {
            return Region::Gt;
        } else if var_env.contains("GG") {
            return Region::Gg;
        } else if var_env.contains("GN") {
            return Region::Gn;
        } else if var_env.contains("GW") {
            return Region::Gw;
        } else if var_env.contains("GY") {
            return Region::Gy;
        } else if var_env.contains("HT") {
            return Region::Ht;
        }

        forth_get_region_helper(var_env)
    }

fn forth_get_region_helper(var_env: String) -> Region {
    if var_env.contains("HM") {
        return Region::Hm;
    } else if var_env.contains("VA") {
        return Region::Va;
    } else if var_env.contains("HN") {
        return Region::Hn;
    } else if var_env.contains("HK") {
        return Region::Hk;
    } else if var_env.contains("HU") {
        return Region::Hu;
    } else if var_env.contains("IS") {
        return Region::Is;
    } else if var_env.contains("IN") {
        return Region::In;
    } else if var_env.contains("ID") {
        return Region::Id;
    } else if var_env.contains("IR") {
        return Region::Ir;
    } else if var_env.contains("IQ") {
        return Region::Iq;
    } else if var_env.contains("IE") {
        return Region::Ie;
    } else if var_env.contains("IM") {
        return Region::Im;
    } else if var_env.contains("IL") {
        return Region::Il;
    } else if var_env.contains("IT") {
        return Region::It;
    } else if var_env.contains("JM") {
        return Region::Jm;
    } else if var_env.contains("JP") {
        return Region::Jp;
    } else if var_env.contains("JE") {
        return Region::Je;
    } else if var_env.contains("JO") {
        return Region::Jo;
    } else if var_env.contains("KZ") {
        return Region::Kz;
    } else if var_env.contains("KE") {
        return Region::Ke;
    } else if var_env.contains("KI") {
        return Region::Ki;
    } else if var_env.contains("KP") {
        return Region::Kp;
    } else if var_env.contains("KR") {
        return Region::Kr;
    } else if var_env.contains("KW") {
        return Region::Kw;
    }

    fifth_get_region_helper(var_env)
}

fn fifth_get_region_helper(var_env: String) -> Region {
    if var_env.contains("KG") {
        return Region::Kg;
    } else if var_env.contains("LA") {
        return Region::La;
    } else if var_env.contains("LV") {
        return Region::Lv;
    } else if var_env.contains("LB") {
        return Region::Lb;
    } else if var_env.contains("LS") {
        return Region::Ls;
    } else if var_env.contains("LR") {
        return Region::Lr;
    } else if var_env.contains("LY") {
        return Region::Ly;
    } else if var_env.contains("LI") {
        return Region::Li;
    } else if var_env.contains("LT") {
        return Region::Lt;
    } else if var_env.contains("LU") {
        return Region::Lu;
    } else if var_env.contains("MO") {
        return Region::Mo;
    } else if var_env.contains("MG") {
        return Region::Mg;
    } else if var_env.contains("MW") {
        return Region::Mw;
    } else if var_env.contains("MY") {
        return Region::My;
    } else if var_env.contains("MV") {
        return Region::Mv;
    } else if var_env.contains("ML") {
        return Region::Ml;
    } else if var_env.contains("MT") {
        return Region::Mt;
    } else if var_env.contains("MH") {
        return Region::Mh;
    } else if var_env.contains("MQ") {
        return Region::Mq;
    } else if var_env.contains("MR") {
        return Region::Mr;
    } else if var_env.contains("MU") {
        return Region::Mu;
    } else if var_env.contains("YT") {
        return Region::Yt;
    } else if var_env.contains("MX") {
        return Region::Mx;
    } else if var_env.contains("FM") {
        return Region::Fm;
    }

    sixth_get_region_helper(var_env)
}

fn sixth_get_region_helper(var_env: String) -> Region {
    if var_env.contains("MD") {
        return Region::Md;
    } else if var_env.contains("MC") {
        return Region::Mc;
    } else if var_env.contains("MN") {
        return Region::Mn;
    } else if var_env.contains("ME") {
        return Region::Me;
    } else if var_env.contains("MS") {
        return Region::Ms;
    } else if var_env.contains("MA") {
        return Region::Ma;
    } else if var_env.contains("MZ") {
        return Region::Mz;
    } else if var_env.contains("MM") {
        return Region::Mm;
    } else if var_env.contains("NA") {
        return Region::Na;
    } else if var_env.contains("NR") {
        return Region::Nr;
    } else if var_env.contains("NP") {
        return Region::Np;
    } else if var_env.contains("NL") {
        return Region::Nl;
    } else if var_env.contains("NC") {
        return Region::Nc;
    } else if var_env.contains("NZ") {
        return Region::Nz;
    } else if var_env.contains("NI") {
        return Region::Ni;
    } else if var_env.contains("NE") {
        return Region::Ne;
    } else if var_env.contains("NG") {
        return Region::Ng;
    } else if var_env.contains("NU") {
        return Region::Nu;
    } else if var_env.contains("NF") {
        return Region::Nf;
    } else if var_env.contains("MP") {
        return Region::Mp;
    } else if var_env.contains("NO") {
        return Region::No;
    } else if var_env.contains("OM") {
        return Region::Om;
    } else if var_env.contains("PK") {
        return Region::Pk;
    } else if var_env.contains("PW") {
        return Region::Pw;
    }

    seventh_get_region_helper(var_env)
}


fn seventh_get_region_helper(var_env: String) -> Region {
    if var_env.contains("PS") {
        return Region::Ps;
    } else if var_env.contains("PA") {
        return Region::Pa;
    } else if var_env.contains("PG") {
        return Region::Pg;
    } else if var_env.contains("PY") {
        return Region::Py;
    } else if var_env.contains("PE") {
        return Region::Pe;
    } else if var_env.contains("PH") {
        return Region::Ph;
    } else if var_env.contains("PN") {
        return Region::Pn;
    } else if var_env.contains("PL") {
        return Region::Pl;
    } else if var_env.contains("PT") {
        return Region::Pt;
    } else if var_env.contains("PR") {
        return Region::Pr;
    } else if var_env.contains("QA") {
        return Region::Qa;
    } else if var_env.contains("MK") {
        return Region::Mk;
    } else if var_env.contains("RO") {
        return Region::Ro;
    } else if var_env.contains("RU") {
        return Region::Ru;
    } else if var_env.contains("RW") {
        return Region::Rw;
    } else if var_env.contains("RE") {
        return Region::Re;
    } else if var_env.contains("BL") {
        return Region::Bl;
    } else if var_env.contains("SH") {
        return Region::Sh;
    } else if var_env.contains("KN") {
        return Region::Kn;
    } else if var_env.contains("LC") {
        return Region::Lc;
    } else if var_env.contains("MF") {
        return Region::Mf;
    } else if var_env.contains("PM") {
        return Region::Pm;
    } else if var_env.contains("VC") {
        return Region::Vc;
    } else if var_env.contains("WS") {
        return Region::Ws;
    }

    eight_get_region_helper(var_env)
}

fn eight_get_region_helper(var_env: String) -> Region {
    if var_env.contains("SM") {
        return Region::Sm;
    } else if var_env.contains("ST") {
        return Region::St;
    } else if var_env.contains("SA") {
        return Region::Sa;
    } else if var_env.contains("SN") {
        return Region::Sn;
    } else if var_env.contains("RS") {
        return Region::Rs;
    } else if var_env.contains("SC") {
        return Region::Sc;
    } else if var_env.contains("SL") {
        return Region::Sl;
    } else if var_env.contains("SG") {
        return Region::Sg;
    } else if var_env.contains("SX") {
        return Region::Sx;
    } else if var_env.contains("SK") {
        return Region::Sk;
    } else if var_env.contains("SI") {
        return Region::Si;
    } else if var_env.contains("SB") {
        return Region::Sb;
    } else if var_env.contains("SO") {
        return Region::So;
    } else if var_env.contains("ZA") {
        return Region::Za;
    } else if var_env.contains("GS") {
        return Region::Gs;
    } else if var_env.contains("SS") {
        return Region::Ss;
    } else if var_env.contains("ES") {
        return Region::Es;
    } else if var_env.contains("LK") {
        return Region::Lk;
    } else if var_env.contains("SD") {
        return Region::Sd;
    } else if var_env.contains("SR") {
        return Region::Sr;
    } else if var_env.contains("SJ") {
        return Region::Sj;
    } else if var_env.contains("SE") {
        return Region::Se;
    } else if var_env.contains("CH") {
        return Region::Ch;
    } else if var_env.contains("SY") {
        return Region::Sy;
    }

    ninth_get_region_helper(var_env)
}

fn ninth_get_region_helper(var_env: String) -> Region {
    if var_env.contains("TW") {
        return Region::Tw;
    } else if var_env.contains("TJ") {
        return Region::Tj;
    } else if var_env.contains("TZ") {
        return Region::Tz;
    } else if var_env.contains("TH") {
        return Region::Th;
    } else if var_env.contains("TL") {
        return Region::Tl;
    } else if var_env.contains("TG") {
        return Region::Tg;
    } else if var_env.contains("TK") {
        return Region::Tk;
    } else if var_env.contains("TO") {
        return Region::To;
    } else if var_env.contains("TT") {
        return Region::Tt;
    } else if var_env.contains("TN") {
        return Region::Tn;
    } else if var_env.contains("TR") {
        return Region::Tr;
    } else if var_env.contains("TM") {
        return Region::Tm;
    } else if var_env.contains("TC") {
        return Region::Tc;
    } else if var_env.contains("TV") {
        return Region::Tv;
    } else if var_env.contains("UG") {
        return Region::Ug;
    } else if var_env.contains("UA") {
        return Region::Ua;
    } else if var_env.contains("AE") {
        return Region::Ae;
    } else if var_env.contains("GB") {
        return Region::Gb;
    } else if var_env.contains("UM") {
        return Region::Um;
    } else if var_env.contains("US") {
        return Region::Us;
    } else if var_env.contains("UY") {
        return Region::Uy;
    } else if var_env.contains("UZ") {
        return Region::Uz;
    } else if var_env.contains("VU") {
        return Region::Vu;
    } else if var_env.contains("VE") {
        return Region::Ve;
    } else if var_env.contains("VN") {
        return Region::Vn;
    } else if var_env.contains("VG") {
        return Region::Vg;
    } else if var_env.contains("VI") {
        return Region::Vi;
    } else if var_env.contains("WF") {
        return Region::Wf;
    } else if var_env.contains("EH") {
        return Region::Eh;
    } else if var_env.contains("YE") {
        return Region::Ye;
    } else if var_env.contains("ZM") {
        return Region::Zm;
    } else if var_env.contains("ZW") {
        return Region::Zw;
    } else if var_env.contains("AX") {
        return Region::Ax;
    }
    Region::Any
}

/// A spoken language
///
/// Use [`ToString::to_string()`] to convert to string of two letter lowercase
/// language code followed and underscore and uppercase region code (example:
/// `en_US`).
///
/// Uses <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
// #[allow(variant_size_differences)]
pub enum Language {
    #[doc(hidden)]
    __(Box<String>),

    /// `ab`: Abkhazian
    #[doc(hidden)]
    Ab(Region),

    /// `aa`: Afar
    #[doc(hidden)]
    Aa(Region),

    /// `AF`: Afrikaans
    #[doc(hidden)]
    Af(Region),

    /// `SQ`: Albanian
    #[doc(hidden)]
    Sq(Region),

    /// `AM`: Amharic
    #[doc(hidden)]
    Am(Region),

    /// `AR`: Arabic
    #[doc(hidden)]
    Ar(Region),

    /// `HY`: Armenian
    #[doc(hidden)]
    Hy(Region),

    /// `AS`: Assamese
    #[doc(hidden)]
    As(Region),

    /// `AY`: Aymara
    #[doc(hidden)]
    Ay(Region),

    /// `AZ`: Azerbaijani
    #[doc(hidden)]
    Az(Region),

    /// `BA`: Bashkir
    #[doc(hidden)]
    Ba(Region),

    /// `EU`: Basque
    #[doc(hidden)]
    Eu(Region),

    /// `bn`: Bengali, Bangla
    #[doc(hidden)]
    Bn(Region),

    /// `dz`: Bhutani
    #[doc(hidden)]
    Dz(Region),

    /// `bh`: Bihari
    #[doc(hidden)]
    Bh(Region),

    /// `bi`: Bislama
    #[doc(hidden)]
    Bi(Region),

    /// `br`: Breton
    #[doc(hidden)]
    Br(Region),

    /// `bg`: Bulgarian
    #[doc(hidden)]
    Bg(Region),

    /// `my`: Burmese
    #[doc(hidden)]
    My(Region),

    /// `be`: Byelorussian
    #[doc(hidden)]
    Be(Region),

    /// `km`: Cambodian
    #[doc(hidden)]
    Km(Region),

    /// `ca`: Catalan
    #[doc(hidden)]
    Ca(Region),

    /// `zh`: Chinese
    #[doc(hidden)]
    Zh(Region),

    /// `co`: Corsican
    #[doc(hidden)]
    Co(Region),

    /// `hr`: Croatian
    #[doc(hidden)]
    Hr(Region),

    /// `CS`: Czech
    #[doc(hidden)]
    Cs(Region),

    /// `DA`: Danish
    #[doc(hidden)]
    Da(Region),

    /// `nl`: Dutch
    #[doc(hidden)]
    Nl(Region),

    /// `en`: English, American
    #[doc(hidden)]
    En(Region),

    /// `EO`: Esperanto
    #[doc(hidden)]
    Eo(Region),

    /// `Et`: Estonian
    #[doc(hidden)]
    Et(Region),

    /// `Fo`: Faeroese
    #[doc(hidden)]
    Fo(Region),

    /// `FJ`: Fiji
    #[doc(hidden)]
    Fj(Region),

    /// `FI`: Finnish
    #[doc(hidden)]
    Fi(Region),

    /// `FR`: French
    #[doc(hidden)]
    Fr(Region),

    /// `fy`: Frisian
    #[doc(hidden)]
    Fy(Region),

    /// `gd`: Gaelic (Scots Gaelic)
    #[doc(hidden)]
    Gd(Region),

    /// `GL`: Galician
    #[doc(hidden)]
    Gl(Region),

    /// `KA`: Georgian
    #[doc(hidden)]
    Ka(Region),

    /// `DE`: German
    #[doc(hidden)]
    De(Region),

    /// `EL`: Greek
    #[doc(hidden)]
    El(Region),

    /// `KL`: Greenlandic
    #[doc(hidden)]
    Kl(Region),

    /// `GN`: Guarani
    #[doc(hidden)]
    Gn(Region),

    /// `GU`: Gujarati
    #[doc(hidden)]
    Gu(Region),

    /// `HA`: Hausa
    #[doc(hidden)]
    Ha(Region),

    /// `IW`: Hebrew
    #[doc(hidden)]
    Iw(Region),

    /// `HI`: Hindi
    #[doc(hidden)]
    Hi(Region),

    /// `HU`: Hungarian
    #[doc(hidden)]
    Hu(Region),

    /// `IS`: Icelandic
    #[doc(hidden)]
    Is(Region),

    /// `IN`: Indonesian
    #[doc(hidden)]
    In(Region),

    /// `IA`: Interlingua
    #[doc(hidden)]
    Ia(Region),

    /// `IE`: Interlingue
    #[doc(hidden)]
    Ie(Region),

    /// `IK`: Inupiak
    #[doc(hidden)]
    Ik(Region),

    /// `GA`: Irish
    #[doc(hidden)]
    Ga(Region),

    /// `IT`: Italian
    #[doc(hidden)]
    It(Region),

    /// `JA`: Japanese
    #[doc(hidden)]
    Ja(Region),

    /// `JW`: Javanese
    #[doc(hidden)]
    Jw(Region),

    /// `KN`: Kannada
    #[doc(hidden)]
    Kn(Region),

    /// `KS`: Kashmiri
    #[doc(hidden)]
    Ks(Region),

    /// `KK`: Kazakh
    #[doc(hidden)]
    Kk(Region),

    /// `RW`: Kinyarwanda
    #[doc(hidden)]
    Rw(Region),

    /// `KY`: Kirghiz
    #[doc(hidden)]
    Ky(Region),

    /// `RN`: Kirundi
    #[doc(hidden)]
    Rn(Region),

    /// `KO`: Korean
    #[doc(hidden)]
    Ko(Region),

    /// `ku`: Kurdish
    #[doc(hidden)]
    Ku(Region),

    /// `lo`: Laothian
    #[doc(hidden)]
    Lo(Region),

    /// `LA`: Latin
    #[doc(hidden)]
    La(Region),

    /// `lv`: Latvian, Lettish
    #[doc(hidden)]
    Lv(Region),

    /// `ln`: Lingala
    #[doc(hidden)]
    Ln(Region),

    /// `lt`: Lithuanian
    #[doc(hidden)]
    Lt(Region),

    /// `mk`: Macedonian
    #[doc(hidden)]
    Mk(Region),

    /// `mg`: Malagasy
    #[doc(hidden)]
    Mg(Region),

    /// `ms`: Malay
    #[doc(hidden)]
    Ms(Region),

    /// `ml`: Malayalam
    #[doc(hidden)]
    Ml(Region),

    /// `mt`: Maltese
    #[doc(hidden)]
    Mt(Region),

    /// `mi`: Maori
    #[doc(hidden)]
    Mi(Region),

    /// `mr`: Marathi
    #[doc(hidden)]
    Mr(Region),

    /// `mo`: Moldavian
    #[doc(hidden)]
    Mo(Region),

    /// `mn`: Mongolian
    #[doc(hidden)]
    Mn(Region),

    /// `na`: Nauru
    #[doc(hidden)]
    Na(Region),

    /// `ne`: Nepali
    #[doc(hidden)]
    Ne(Region),

    /// `no`: Norwegian
    #[doc(hidden)]
    No(Region),

    /// `oc`: Occitan
    #[doc(hidden)]
    Oc(Region),

    /// `or`: Oriya
    #[doc(hidden)]
    Or(Region),

    /// `om`: Oromo, Afan
    #[doc(hidden)]
    Om(Region),

    /// `ps`: Pashto, Pushto
    #[doc(hidden)]
    Ps(Region),

    /// `fa`: Persian
    #[doc(hidden)]
    Fa(Region),

    /// `pl`: Polish
    #[doc(hidden)]
    Pl(Region),

    /// `pt`: Portuguese
    #[doc(hidden)]
    Pt(Region),

    /// `pa`: Punjabi
    #[doc(hidden)]
    Pa(Region),

    /// `qu`: Quechua
    #[doc(hidden)]
    Qu(Region),

    /// `rm`: Rhaeto-Romance
    #[doc(hidden)]
    Rm(Region),

    /// `ro`: Romanian
    #[doc(hidden)]
    Ro(Region),

    /// `ru`: Russian
    #[doc(hidden)]
    Ru(Region),

    /// `sm`: Samoan
    #[doc(hidden)]
    Sm(Region),

    /// `SG`: Sangro
    #[doc(hidden)]
    Sg(Region),

    /// `sa`: Sanskrit
    #[doc(hidden)]
    Sa(Region),

    /// `sr`: Serbian
    #[doc(hidden)]
    Sr(Region),

    /// `sh`: Serbo-Croatian
    #[doc(hidden)]
    Sh(Region),

    /// `st`: Sesotho
    #[doc(hidden)]
    St(Region),

    /// `tn`: Setswana
    #[doc(hidden)]
    Tn(Region),

    /// `sn`: Shona
    #[doc(hidden)]
    Sn(Region),

    /// `sd`: Sindhi
    #[doc(hidden)]
    Sd(Region),

    /// `si`: Singhalese
    #[doc(hidden)]
    Si(Region),

    /// `ss`: Siswati
    #[doc(hidden)]
    Ss(Region),

    /// `sk`: Slovak
    #[doc(hidden)]
    Sk(Region),

    /// `sl`: Slovenian
    #[doc(hidden)]
    Sl(Region),

    /// `so`: Somali
    #[doc(hidden)]
    So(Region),

    /// `es`: Spanish
    #[doc(hidden)]
    Es(Region),

    /// `su`: Sudanese
    #[doc(hidden)]
    Su(Region),

    /// `sw`: Swahili
    #[doc(hidden)]
    Sw(Region),

    /// `sv`: Swedish
    #[doc(hidden)]
    Sv(Region),

    /// `tl`: Tagalog
    #[doc(hidden)]
    Tl(Region),

    /// `tg`: Tajik
    #[doc(hidden)]
    Tg(Region),

    /// `ta`: Tamil
    #[doc(hidden)]
    Ta(Region),

    /// `tt`: Tatar
    #[doc(hidden)]
    Tt(Region),

    /// `te`: Tegulu
    #[doc(hidden)]
    Te(Region),

    /// `th`: Thai
    #[doc(hidden)]
    Th(Region),

    /// `bo`: Tibetan
    #[doc(hidden)]
    Bo(Region),

    /// `ti`: Tigrinya
    #[doc(hidden)]
    Ti(Region),

    /// `to`: Tonga
    #[doc(hidden)]
    To(Region),

    /// `ts`: Tsonga
    #[doc(hidden)]
    Ts(Region),

    /// `tr`: Turkish
    #[doc(hidden)]
    Tr(Region),

    /// `tk`: Turkmen
    #[doc(hidden)]
    Tk(Region),

    /// `tw`: Twi
    #[doc(hidden)]
    Tw(Region),

    /// `uk`: Ukrainian
    #[doc(hidden)]
    Uk(Region),

    /// `ur`: Urdu
    #[doc(hidden)]
    Ur(Region),

    /// `uz`: Uzbek
    #[doc(hidden)]
    Uz(Region),

    /// `Vi`: Vietnamese
    #[doc(hidden)]
    Vi(Region),

    /// `vo`: Volapuk
    #[doc(hidden)]
    Vo(Region),

    /// `cy`: Welsh
    #[doc(hidden)]
    Cy(Region),

    /// `wo`: Wolof
    #[doc(hidden)]
    Wo(Region),

    /// `Xh`: Xhosa
    #[doc(hidden)]
    Xh(Region),

    /// `ji`: Yiddish
    #[doc(hidden)]
    Ji(Region),

    /// `yo`: Yoruba
    #[doc(hidden)]
    Yo(Region),

    /// `zu`: Zulu
    #[doc(hidden)]
    Zu(Region),

}


impl Language {
    /// Retrieve the region code for this language dialect.
    pub fn region(&self) -> Region {
        match self {
            Self::__(_) => Region::Any,
            Self::Ab(region) => *region,		// Abkhazian	AB
            Self::Aa(region) => *region,		// Afar	AA
            Self::Af(region) => *region,		// Afrikaans	AF
            Self::Sq(region) => *region,		// Albanian	SQ
            Self::Am(region) => *region,		// Amharic	AM
            Self::Ar(region) => *region,		// Arabic	AR
            Self::Hy(region) => *region,		// Armenian	HY
            Self::As(region) => *region,		// Assamese	AS
            Self::Ay(region) => *region,		// Aymara	AY
            Self::Az(region) => *region,		// Azerbaijani	AZ
            Self::Ba(region) => *region,		// Bashkir	BA
            Self::Eu(region) => *region,		// Basque	EU
            Self::Bn(region) => *region,		// Bengali, Bangla	BN
            Self::Dz(region) => *region,		// Bhutani	DZ
            Self::Bh(region) => *region,		// Bihari	BH
            Self::Bi(region) => *region,		// Bislama	BI
            Self::Br(region) => *region,		// Breton	BR
            Self::Bg(region) => *region,		// Bulgarian	BG
            Self::My(region) => *region,		// Burmese	MY
            Self::Be(region) => *region,		// Byelorussian	BE
            Self::Km(region) => *region,		// Cambodian	KM
            Self::Ca(region) => *region,		// Catalan	CA
            Self::Zh(region) => *region,		// Chinese	ZH
            Self::Co(region) => *region,		// Corsican	CO
            Self::Hr(region) => *region,		// Croatian	HR
            Self::Cs(region) => *region,		// Czech	CS
            Self::Da(region) => *region,		// Danish	DA
            Self::Nl(region) => *region,		// Dutch	NL
            Self::En(region) => *region,		// English, American	EN
            Self::Eo(region) => *region,		// Esperanto	EO
            Self::Et(region) => *region,		// Estonian	ET
            Self::Fo(region) => *region,		// Faeroese	FO
            Self::Fj(region) => *region,		// Fiji	FJ
            Self::Fi(region) => *region,		// Finnish	FI
            Self::Fr(region) => *region,		// French	FR
            Self::Fy(region) => *region,		// Frisian	FY
            Self::Gd(region) => *region,		// Gaelic (Scots Gaelic)	GD
            Self::Gl(region) => *region,		// Galician	GL
            Self::Ka(region) => *region,		// Georgian	KA
            Self::De(region) => *region,		// German	DE
            Self::El(region) => *region,		// Greek	EL
            Self::Kl(region) => *region,		// Greenlandic	KL
            Self::Gn(region) => *region,		// Guarani	GN
            Self::Gu(region) => *region,		// Gujarati	GU
            Self::Ha(region) => *region,		// Hausa	HA
            Self::Iw(region) => *region,		// Hebrew	IW
            Self::Hi(region) => *region,		// Hindi	HI
            Self::Hu(region) => *region,		// Hungarian	HU
            Self::Is(region) => *region,		// Icelandic	IS
            Self::In(region) => *region,		// Indonesian	IN
            Self::Ia(region) => *region,		// Interlingua	IA
            Self::Ie(region) => *region,		// Interlingue	IE
            Self::Ik(region) => *region,		// Inupiak	IK
            Self::Ga(region) => *region,		// Irish	GA
            Self::It(region) => *region,		// Italian	IT
            Self::Ja(region) => *region,		// Japanese	JA
            Self::Jw(region) => *region,		// Javanese	JW
            Self::Kn(region) => *region,		// Kannada	KN
            Self::Ks(region) => *region,		// Kashmiri	KS
            Self::Kk(region) => *region,		// Kazakh	KK
            Self::Rw(region) => *region,		// Kinyarwanda	RW
            Self::Ky(region) => *region,		// Kirghiz	KY
            Self::Rn(region) => *region,		// Kirundi	RN
            Self::Ko(region) => *region,		// Korean	KO
            Self::Ku(region) => *region,		// Kurdish	KU
            Self::Lo(region) => *region,		// Laothian	LO
            Self::La(region) => *region,		// Latin	LA
            Self::Lv(region) => *region,		// Latvian, Lettish	LV
            Self::Ln(region) => *region,		// Lingala	LN
            Self::Lt(region) => *region,		// Lithuanian	LT
            Self::Mk(region) => *region,		// Macedonian	MK
            Self::Mg(region) => *region,		// Malagasy	MG
            Self::Ms(region) => *region,		// Malay	MS
            Self::Ml(region) => *region,		// Malayalam	ML
            Self::Mt(region) => *region,		// Maltese	MT
            Self::Mi(region) => *region,		// Maori	MI
            Self::Mr(region) => *region,		// Marathi	MR
            Self::Mo(region) => *region,		// Moldavian	MO
            Self::Mn(region) => *region,		// Mongolian	MN
            Self::Na(region) => *region,		// Nauru	NA
            Self::Ne(region) => *region,		// Nepali	NE
            Self::No(region) => *region,		// Norwegian	NO
            Self::Oc(region) => *region,		// Occitan	OC
            Self::Or(region) => *region,		// Oriya	OR
            Self::Om(region) => *region,		// Oromo, Afan	OM
            Self::Ps(region) => *region,		// Pashto, Pushto	PS
            Self::Fa(region) => *region,		// Persian	FA
            Self::Pl(region) => *region,		// Polish	PL
            Self::Pt(region) => *region,		// Portuguese	PT
            Self::Pa(region) => *region,		// Punjabi	PA
            Self::Qu(region) => *region,		// Quechua	QU
            Self::Rm(region) => *region,		// Rhaeto-Romance	RM
            Self::Ro(region) => *region,		// Romanian	RO
            Self::Ru(region) => *region,		// Russian	RU
            Self::Sm(region) => *region,		// Samoan	SM
            Self::Sg(region) => *region,		// Sangro	SG
            Self::Sa(region) => *region,		// Sanskrit	SA
            Self::Sr(region) => *region,		// Serbian	SR
            Self::Sh(region) => *region,		// Serbo-Croatian	SH
            Self::St(region) => *region,		// Sesotho	ST
            Self::Tn(region) => *region,		// Setswana	TN
            Self::Sn(region) => *region,		// Shona	SN
            Self::Sd(region) => *region,		// Sindhi	SD
            Self::Si(region) => *region,		// Singhalese	SI
            Self::Ss(region) => *region,		// Siswati	SS
            Self::Sk(region) => *region,		// Slovak	SK
            Self::Sl(region) => *region,		// Slovenian	SL
            Self::So(region) => *region,		// Somali	SO
            Self::Es(region) => *region,		// Spanish	ES
            Self::Su(region) => *region,		// Sudanese	SU
            Self::Sw(region) => *region,		// Swahili	SW
            Self::Sv(region) => *region,		// Swedish	SV
            Self::Tl(region) => *region,		// Tagalog	TL
            Self::Tg(region) => *region,		// Tajik	TG
            Self::Ta(region) => *region,		// Tamil	TA
            Self::Tt(region) => *region,		// Tatar	TT
            Self::Te(region) => *region,		// Tegulu	TE
            Self::Th(region) => *region,		// Thai	TH
            Self::Bo(region) => *region,		// Tibetan	BO
            Self::Ti(region) => *region,		// Tigrinya	TI
            Self::To(region) => *region,		// Tonga	TO
            Self::Ts(region) => *region,		// Tsonga	TS
            Self::Tr(region) => *region,		// Turkish	TR
            Self::Tk(region) => *region,		// Turkmen	TK
            Self::Tw(region) => *region,		// Twi	TW
            Self::Uk(region) => *region,		// Ukrainian	UK
            Self::Ur(region) => *region,		// Urdu	UR
            Self::Uz(region) => *region,		// Uzbek	UZ
            Self::Vi(region) => *region,		// Vietnamese	VI
            Self::Vo(region) => *region,		// Volapuk	VO
            Self::Cy(region) => *region,		// Welsh	CY
            Self::Wo(region) => *region,		// Wolof	WO
            Self::Xh(region) => *region,		// Xhosa	XH
            Self::Ji(region) => *region,		// Yiddish	JI
            Self::Yo(region) => *region,		// Yoruba	YO
            Self::Zu(region) => *region,		// Zulu	ZU
        }
    }
}

impl Display for Language {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::__(code) => f.write_str(code.as_str()),

            // Abkhazian	AB
            Self::Ab(region) => {
                if *region != Region::Any {
                    f.write_str("ab_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ab")
                }
            }

            // Afar	AA
            Self::Aa(region) => {
                if *region != Region::Any {
                    f.write_str("aa_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("aa")
                }
            }

            // Afrikaans	AF
            Self::Af(region) => {
                if *region != Region::Any {
                    f.write_str("af_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("af")
                }
            }

            // Albanian	SQ
            Self::Sq(region) => {
                if *region != Region::Any {
                    f.write_str("sq_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sq")
                }
            }

            // Amharic	AM
            Self::Am(region) => {
                if *region != Region::Any {
                    f.write_str("am_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("am")
                }
            }

            // Arabic	AR
            Self::Ar(region) => {
                if *region != Region::Any {
                    f.write_str("ar_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ar")
                }
            }

            // Armenian	HY
            Self::Hy(region) => {
                if *region != Region::Any {
                    f.write_str("hy_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("hy")
                }
            }

            // Assamese	AS
            Self::As(region) => {
                if *region != Region::Any {
                    f.write_str("as_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("as")
                }
            }

            // Aymara	AY
            Self::Ay(region) => {
                if *region != Region::Any {
                    f.write_str("ay_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ay")
                }
            }

            // Azerbaijani	AZ
            Self::Az(region) => {
                if *region != Region::Any {
                    f.write_str("az_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("az")
                }
            }

            // Bashkir	BA
            Self::Ba(region) => {
                if *region != Region::Any {
                    f.write_str("ba_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ba")
                }
            }

            // Basque	EU
            Self::Eu(region) => {
                if *region != Region::Any {
                    f.write_str("eu_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("eu")
                }
            }

            // Bengali, Bangla	BN
            Self::Bn(region) => {
                if *region != Region::Any {
                    f.write_str("bn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("bn")
                }
            }

            // Bhutani	DZ
            Self::Dz(region) => {
                if *region != Region::Any {
                    f.write_str("dz_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("dz")
                }
            }

            // Bihari	BH
            Self::Bh(region) => {
                if *region != Region::Any {
                    f.write_str("bh_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("bh")
                }
            }

            // Bislama	BI
            Self::Bi(region) => {
                if *region != Region::Any {
                    f.write_str("bi_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("bi")
                }
            }

            // Breton	BR
            Self::Br(region) => {
                if *region != Region::Any {
                    f.write_str("br_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("br")
                }
            }

        // Bulgarian	BG
            Self::Bg(region) => {
                if *region != Region::Any {
                    f.write_str("bg_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("bg")
                }
            }

            // Burmese	MY
            Self::My(region) => {
                if *region != Region::Any {
                    f.write_str("my_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("my")
                }
            }

            // Byelorussian	BE
            Self::Be(region) => {
                if *region != Region::Any {
                    f.write_str("be_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("be")
                }
            }

            // Cambodian	KM
            Self::Km(region) => {
                if *region != Region::Any {
                    f.write_str("km_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("km")
                }
            }

            // Catalan	CA
            Self::Ca(region) => {
                if *region != Region::Any {
                    f.write_str("ca_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ca")
                }
            }

            // Chinese	ZH
            Self::Zh(region) => {
                if *region != Region::Any {
                    f.write_str("zh_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("zh")
                }
            }

            // Corsican	CO
            Self::Co(region) => {
                if *region != Region::Any {
                    f.write_str("co_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("co")
                }
            }

            // Croatian	HR
            Self::Hr(region) => {
                if *region != Region::Any {
                    f.write_str("hr_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("hr")
                }
            }

            // Czech	CS
            Self::Cs(region) => {
                if *region != Region::Any {
                    f.write_str("cs_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("cs")
                }
            }

            // Danish	DA
            Self::Da(region) => {
                if *region != Region::Any {
                    f.write_str("da_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("da")
                }
            }

            // Dutch	NL
            Self::Nl(region) => {
                if *region != Region::Any {
                    f.write_str("nl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("nl")
                }
            }

            // English, American	EN
            Self::En(region) => {
                if *region != Region::Any {
                    f.write_str("en_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("en")
                }
            }

            // Esperanto	EO
            Self::Eo(region) => {
                if *region != Region::Any {
                    f.write_str("eo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("eo")
                }
            }

            // Estonian	ET
            Self::Et(region) => {
                if *region != Region::Any {
                    f.write_str("et_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("et")
                }
            }

            // Faeroese	FO
            Self::Fo(region) => {
                if *region != Region::Any {
                    f.write_str("fo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fo")
                }
            }

            // Fiji	FJ
            Self::Fj(region) => {
                if *region != Region::Any {
                    f.write_str("fj_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fj")
                }
            }

            // Finnish	FI
            Self::Fi(region) => {
                if *region != Region::Any {
                    f.write_str("fi_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fi")
                }
            }

            // French	FR
            Self::Fr(region) => {
                if *region != Region::Any {
                    f.write_str("fr_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fr")
                }
            }

            // Frisian	FY
            Self::Fy(region) => {
                if *region != Region::Any {
                    f.write_str("fy_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fy")
                }
            }

            // Gaelic (Scots Gaelic)	GD
            Self::Gd(region) => {
                if *region != Region::Any {
                    f.write_str("gd_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("gd")
                }
            }

            // Galician	GL
            Self::Gl(region) => {
                if *region != Region::Any {
                    f.write_str("gl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("gl")
                }
            }

            // Georgian	KA
            Self::Ka(region) => {
                if *region != Region::Any {
                    f.write_str("ka_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ka")
                }
            }

            // German	DE
            Self::De(region) => {
                if *region != Region::Any {
                    f.write_str("de_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("de")
                }
            }

            // Greek	EL
            Self::El(region) => {
                if *region != Region::Any {
                    f.write_str("el_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("el")
                }
            }

            // Greenlandic	KL
            Self::Kl(region) => {
                if *region != Region::Any {
                    f.write_str("kl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("kl")
                }
            }

            // Guarani	GN
            Self::Gn(region) => {
                if *region != Region::Any {
                    f.write_str("gn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("gn")
                }
            }

            // Gujarati	GU
            Self::Gu(region) => {
                if *region != Region::Any {
                    f.write_str("gu_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("gu")
                }
            }

            // Hausa	HA
            Self::Ha(region) => {
                if *region != Region::Any {
                    f.write_str("ha_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ha")
                }
            }

            // Hebrew	IW
            Self::Iw(region) => {
                if *region != Region::Any {
                    f.write_str("iw_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("iw")
                }
            }

            // Hindi	HI
            Self::Hi(region) => {
                if *region != Region::Any {
                    f.write_str("hi_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("hi")
                }
            }

            // Hungarian	HU
            Self::Hu(region) => {
                if *region != Region::Any {
                    f.write_str("hu_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("hu")
                }
            }

            // Icelandic	IS
            Self::Is(region) => {
                if *region != Region::Any {
                    f.write_str("is_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("is")
                }
            }

            // Indonesian	IN
            Self::In(region) => {
                if *region != Region::Any {
                    f.write_str("in_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("in")
                }
            }

            // Interlingua	IA
            Self::Ia(region) => {
                if *region != Region::Any {
                    f.write_str("ia_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ia")
                }
            }

            // Interlingue	IE
            Self::Ie(region) => {
                if *region != Region::Any {
                    f.write_str("ie_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ie")
                }
            }

            // Inupiak	IK
            Self::Ik(region) => {
                if *region != Region::Any {
                    f.write_str("ik_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ik")
                }
            }

            // Irish	GA
            Self::Ga(region) => {
                if *region != Region::Any {
                    f.write_str("ga_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ga")
                }
            }

            // Italian	IT
            Self::It(region) => {
                if *region != Region::Any {
                    f.write_str("it_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("it")
                }
            }

            // Japanese	JA
            Self::Ja(region) => {
                if *region != Region::Any {
                    f.write_str("ja_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ja")
                }
            }

            // Javanese	JW
            Self::Jw(region) => {
                if *region != Region::Any {
                    f.write_str("jw_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("jw")
                }
            }

            // Kannada	KN
            Self::Kn(region) => {
                if *region != Region::Any {
                    f.write_str("kn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("kn")
                }
            }

            // Kashmiri	KS
            Self::Ks(region) => {
                if *region != Region::Any {
                    f.write_str("ks_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ks")
                }
            }

            // Kazakh	KK
            Self::Kk(region) => {
                if *region != Region::Any {
                    f.write_str("kk_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("kk")
                }
            }

            // Kinyarwanda	RW
            Self::Rw(region) => {
                if *region != Region::Any {
                    f.write_str("rw_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("rw")
                }
            }

            // Kirghiz	KY
            Self::Ky(region) => {
                if *region != Region::Any {
                    f.write_str("ky_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ky")
                }
            }

            // Kirundi	RN
            Self::Rn(region) => {
                if *region != Region::Any {
                    f.write_str("rn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("rn")
                }
            }

            // Korean	KO
            Self::Ko(region) => {
                if *region != Region::Any {
                    f.write_str("ko_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ko")
                }
            }

            // Kurdish	KU
            Self::Ku(region) => {
                if *region != Region::Any {
                    f.write_str("ku_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ku")
                }
            }

            // Laothian	LO
            Self::Lo(region) => {
                if *region != Region::Any {
                    f.write_str("lo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("lo")
                }
            }

            // Latin	LA
            Self::La(region) => {
                if *region != Region::Any {
                    f.write_str("la_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("la")
                }
            }

            // Latvian, Lettish	LV
            Self::Lv(region) => {
                if *region != Region::Any {
                    f.write_str("lv_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("lv")
                }
            }

            // Lingala	LN
            Self::Ln(region) => {
                if *region != Region::Any {
                    f.write_str("ln_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ln")
                }
            }

            // Lithuanian	LT
            Self::Lt(region) => {
                if *region != Region::Any {
                    f.write_str("lt_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("lt")
                }
            }

            // Macedonian	MK
            Self::Mk(region) => {
                if *region != Region::Any {
                    f.write_str("mk_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mk")
                }
            }

            // Malagasy	MG
            Self::Mg(region) => {
                if *region != Region::Any {
                    f.write_str("mg_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mg")
                }
            }

            // Malay	MS
            Self::Ms(region) => {
                if *region != Region::Any {
                    f.write_str("ms_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ms")
                }
            }

            // Malayalam	ML
            Self::Ml(region) => {
                if *region != Region::Any {
                    f.write_str("ml_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ml")
                }
            }

            // Maltese	MT
            Self::Mt(region) => {
                if *region != Region::Any {
                    f.write_str("mt_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mt")
                }
            }

            // Maori	MI
            Self::Mi(region) => {
                if *region != Region::Any {
                    f.write_str("mi_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mi")
                }
            }

            // Marathi	MR
            Self::Mr(region) => {
                if *region != Region::Any {
                    f.write_str("mr_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mr")
                }
            }

            // Moldavian	MO
            Self::Mo(region) => {
                if *region != Region::Any {
                    f.write_str("mo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mo")
                }
            }

            // Mongolian	MN
            Self::Mn(region) => {
                if *region != Region::Any {
                    f.write_str("mn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("mn")
                }
            }

            // Nauru	NA
            Self::Na(region) => {
                if *region != Region::Any {
                    f.write_str("na_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("na")
                }
            }

            // Nepali	NE
            Self::Ne(region) => {
                if *region != Region::Any {
                    f.write_str("ne_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ne")
                }
            }

            // Norwegian	NO
            Self::No(region) => {
                if *region != Region::Any {
                    f.write_str("no_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("no")
                }
            }

            // Occitan	OC
            Self::Oc(region) => {
                if *region != Region::Any {
                    f.write_str("oc_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("oc")
                }
            }

            // Oriya	OR
            Self::Or(region) => {
                if *region != Region::Any {
                    f.write_str("or_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("or")
                }
            }

            // Oromo, Afan	OM
            Self::Om(region) => {
                if *region != Region::Any {
                    f.write_str("om_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("om")
                }
            }

            // Pashto, Pushto	PS
            Self::Ps(region) => {
                if *region != Region::Any {
                    f.write_str("ps_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ps")
                }
            }

            // Persian	FA
            Self::Fa(region) => {
                if *region != Region::Any {
                    f.write_str("fa_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("fa")
                }
            }

            // Polish	PL
            Self::Pl(region) => {
                if *region != Region::Any {
                    f.write_str("pl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("pl")
                }
            }

            // Portuguese	PT
            Self::Pt(region) => {
                if *region != Region::Any {
                    f.write_str("pt_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("pt")
                }
            }

            // Punjabi	PA
            Self::Pa(region) => {
                if *region != Region::Any {
                    f.write_str("pd_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("pa")
                }
            }

            // Quechua	QU
            Self::Qu(region) => {
                if *region != Region::Any {
                    f.write_str("qu_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("qu")
                }
            }

            // Rhaeto-Romance	RM
            Self::Rm(region) => {
                if *region != Region::Any {
                    f.write_str("rm_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("rm")
                }
            }

            // Romanian	RO
            Self::Ro(region) => {
                if *region != Region::Any {
                    f.write_str("ro_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ro")
                }
            }

            // Russian	RU
            Self::Ru(region) => {
                if *region != Region::Any {
                    f.write_str("ru_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ru")
                }
            }

            // Samoan	SM
            Self::Sm(region) => {
                if *region != Region::Any {
                    f.write_str("sm_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sm")
                }
            }

            // Sangro	SG
            Self::Sg(region) => {
                if *region != Region::Any {
                    f.write_str("sg_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sg")
                }
            }

            // Sanskrit	SA
            Self::Sa(region) => {
                if *region != Region::Any {
                    f.write_str("sa_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sa")
                }
            }

            // Serbian	SR
            Self::Sr(region) => {
                if *region != Region::Any {
                    f.write_str("sr_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sr")
                }
            }

            // Serbo-Croatian	SH
            Self::Sh(region) => {
                if *region != Region::Any {
                    f.write_str("sh_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sh")
                }
            }

            // Sesotho	ST
            Self::St(region) => {
                if *region != Region::Any {
                    f.write_str("st_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("st")
                }
            }

            // Setswana	TN
            Self::Tn(region) => {
                if *region != Region::Any {
                    f.write_str("tn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tn")
                }
            }

            // Shona	SN
            Self::Sn(region) => {
                if *region != Region::Any {
                    f.write_str("sn_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sn")
                }
            }

            // Sindhi	SD
            Self::Sd(region) => {
                if *region != Region::Any {
                    f.write_str("sd_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sd")
                }
            }

            // Singhalese	SI
            Self::Si(region) => {
                if *region != Region::Any {
                    f.write_str("si_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("si")
                }
            }

            // Siswati	SS
            Self::Ss(region) => {
                if *region != Region::Any {
                    f.write_str("ss_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ss")
                }
            }

            // Slovak	SK
            Self::Sk(region) => {
                if *region != Region::Any {
                    f.write_str("sk_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sk")
                }
            }

            // Slovenian	SL
            Self::Sl(region) => {
                if *region != Region::Any {
                    f.write_str("Sl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("Sl")
                }
            }

            // Somali	SO
            Self::So(region) => {
                if *region != Region::Any {
                    f.write_str("so_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("so")
                }
            }

            // Spanish	ES
            Self::Es(region) => {
                if *region != Region::Any {
                    f.write_str("es_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("es")
                }
            }

            // Sudanese	SU
            Self::Su(region) => {
                if *region != Region::Any {
                    f.write_str("su_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("su")
                }
            }

            // Swahili	SW
            Self::Sw(region) => {
                if *region != Region::Any {
                    f.write_str("sw_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sw")
                }
            }

            // Swedish	SV
            Self::Sv(region) => {
                if *region != Region::Any {
                    f.write_str("sv_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("sv")
                }
            }

            // Tagalog	TL
            Self::Tl(region) => {
                if *region != Region::Any {
                    f.write_str("tl_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tl")
                }
            }

            // Tajik	TG
            Self::Tg(region) => {
                if *region != Region::Any {
                    f.write_str("tg_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tg")
                }
            }

            // Tamil	TA
            Self::Ta(region) => {
                if *region != Region::Any {
                    f.write_str("ta_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ta")
                }
            }

            // Tatar	TT
            Self::Tt(region) => {
                if *region != Region::Any {
                    f.write_str("tt_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tt")
                }
            }

            // Tegulu	TE
            Self::Te(region) => {
                if *region != Region::Any {
                    f.write_str("te_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("te")
                }
            }

            // Thai	TH
            Self::Th(region) => {
                if *region != Region::Any {
                    f.write_str("th_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("th")
                }
            }

            // Tibetan	BO
            Self::Bo(region) => {
                if *region != Region::Any {
                    f.write_str("bo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("bo")
                }
            }

            // Tigrinya	TI
            Self::Ti(region) => {
                if *region != Region::Any {
                    f.write_str("ti_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ti")
                }
            }

            // Tonga	TO
            Self::To(region) => {
                if *region != Region::Any {
                    f.write_str("to_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("to")
                }
            }

            // Tsonga	TS
            Self::Ts(region) => {
                if *region != Region::Any {
                    f.write_str("ts_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ts")
                }
            }

            // Turkish	TR
            Self::Tr(region) => {
                if *region != Region::Any {
                    f.write_str("tr_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tr")
                }
            }

            // Turkmen	TK
            Self::Tk(region) => {
                if *region != Region::Any {
                    f.write_str("tk_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tk")
                }
            }

            // Twi	TW
            Self::Tw(region) => {
                if *region != Region::Any {
                    f.write_str("tw_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("tw")
                }
            }

            // Ukrainian	UK
            Self::Uk(region) => {
                if *region != Region::Any {
                    f.write_str("uk_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("uk")
                }
            }

            // Urdu	UR
            Self::Ur(region) => {
                if *region != Region::Any {
                    f.write_str("ur_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ur")
                }
            }

            // Uzbek	UZ
            Self::Uz(region) => {
                if *region != Region::Any {
                    f.write_str("uz_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("uz")
                }
            }

            // Vietnamese	VI
            Self::Vi(region) => {
                if *region != Region::Any {
                    f.write_str("vi_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("vi")
                }
            }

            // Volapuk	VO
            Self::Vo(region) => {
                if *region != Region::Any {
                    f.write_str("vo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("vo")
                }
            }

            // Welsh	CY
            Self::Cy(region) => {
                if *region != Region::Any {
                    f.write_str("cy_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("cy")
                }
            }

            // Wolof	WO
            Self::Wo(region) => {
                if *region != Region::Any {
                    f.write_str("wo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("wo")
                }
            }

            // Xhosa	XH
            Self::Xh(region) => {
                if *region != Region::Any {
                    f.write_str("xh_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("xh")
                }
            }

            // Yiddish	JI
            Self::Ji(region) => {
                if *region != Region::Any {
                    f.write_str("ji_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("ji")
                }
            }

            // Yoruba	YO
            Self::Yo(region) => {
                if *region != Region::Any {
                    f.write_str("yo_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("yo")
                }
            }

            // Zulu	ZU
            Self::Zu(region) => {
                if *region != Region::Any {
                    f.write_str("zu_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("zu")
                }
            }
        }
    }
}





/// the `function get_language_and_country` returns a String
/// `en_US.UTF-8`       : `language_country.textencoding`
/// the two lowercase letters before the underscore `_`
/// represents the `language`
impl Language {
    #[allow(dead_code)]
    fn get() -> Self {

        // the language is written with two lowercase characters
        get_language_helper()
    }
}

fn get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("ab") {
        return Language::Ab(region);
    } else if var_env.contains("aa") {
        return Language::Aa(region);
    } else if var_env.contains("af") {
        return Language::Af(region);
    } else if var_env.contains("sq") {
        return Language::Sq(region);
    } else if var_env.contains("am") {
        return Language::Am(region);
    } else if var_env.contains("ar") {
        return Language::Ar(region);
    } else if var_env.contains("hy") {
        return Language::Hy(region);
    } else if var_env.contains("as") {
        return Language::As(region);
    } else if var_env.contains("ay") {
        return Language::Ay(region);
    } else if var_env.contains("az") {
        return Language::Az(region);
    } else if var_env.contains("ba") {
        return Language::Ba(region);
    } else if var_env.contains("eu") {
        return Language::Eu(region);
    } else if var_env.contains("bn") {
        return Language::Bn(region);
    } else if var_env.contains("dz") {
        return Language::Dz(region);
    } else if var_env.contains("bh") {
        return Language::Bh(region);
    } else if var_env.contains("bi") {
        return Language::Bi(region);
    } else if var_env.contains("br") {
        return Language::Br(region);
    } else if var_env.contains("bg") {
        return Language::Bg(region);
    } else if var_env.contains("my") {
        return Language::My(region);
    } else if var_env.contains("be") {
        return Language::Be(region);
    } else if var_env.contains("km") {
        return Language::Km(region);
    } else if var_env.contains("ca") {
        return Language::Ca(region);
    } else if var_env.contains("zh") {
        return Language::Zh(region);
    } else if var_env.contains("co") {
        return Language::Co(region);
    }

    second_get_language_helper()
}

fn second_get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("hr") {
        return Language::Hr(region);
    } else if var_env.contains("cs") {
        return Language::Cs(region);
    } else if var_env.contains("da") {
        return Language::Da(region);
    } else if var_env.contains("nl") {
        return Language::Nl(region);
    } else if var_env.contains("en") {
        return Language::En(region);
    } else if var_env.contains("eo") {
        return Language::Eo(region);
    } else if var_env.contains("et") {
        return Language::Et(region);
    } else if var_env.contains("fo") {
        return Language::Fo(region);
    } else if var_env.contains("fj") {
        return Language::Fj(region);
    } else if var_env.contains("fi") {
        return Language::Fi(region);
    } else if var_env.contains("fr") {
        return Language::Fr(region);
    } else if var_env.contains("fy") {
        return Language::Fy(region);
    } else if var_env.contains("gd") {
        return Language::Gd(region);
    } else if var_env.contains("gl") {
        return Language::Gl(region);
    } else if var_env.contains("ka") {
        return Language::Ka(region);
    } else if var_env.contains("de") {
        return Language::De(region);
    } else if var_env.contains("el") {
        return Language::El(region);
    } else if var_env.contains("kl") {
        return Language::Kl(region);
    } else if var_env.contains("gn") {
        return Language::Gn(region);
    } else if var_env.contains("gu") {
        return Language::Gu(region);
    } else if var_env.contains("ha") {
        return Language::Ha(region);
    } else if var_env.contains("iw") {
        return Language::Iw(region);
    } else if var_env.contains("hi") {
        return Language::Hi(region);
    } else if var_env.contains("hu") {
        return Language::Hu(region);
    }

    third_get_language_helper()
}


fn third_get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("is") {
        return Language::Is(region);
    } else if var_env.contains("in") {
        return Language::In(region);
    } else if var_env.contains("ia") {
        return Language::Ia(region);
    } else if var_env.contains("ie") {
        return Language::Ie(region);
    } else if var_env.contains("ik") {
        return Language::Ik(region);
    } else if var_env.contains("ga") {
        return Language::Ga(region);
    } else if var_env.contains("it") {
        return Language::It(region);
    } else if var_env.contains("ja") {
        return Language::Ja(region);
    } else if var_env.contains("jw") {
        return Language::Jw(region);
    } else if var_env.contains("kn") {
        return Language::Kn(region);
    } else if var_env.contains("ks") {
        return Language::Ks(region);
    } else if var_env.contains("kk") {
        return Language::Kk(region);
    } else if var_env.contains("rw") {
        return Language::Rw(region);
    } else if var_env.contains("ky") {
        return Language::Ky(region);
    } else if var_env.contains("rn") {
        return Language::Rn(region);
    } else if var_env.contains("ko") {
        return Language::Ko(region);
    } else if var_env.contains("ku") {
        return Language::Ku(region);
    } else if var_env.contains("lo") {
        return Language::Lo(region);
    } else if var_env.contains("la") {
        return Language::La(region);
    } else if var_env.contains("lv") {
        return Language::Lv(region);
    } else if var_env.contains("ln") {
        return Language::Ln(region);
    } else if var_env.contains("lt") {
        return Language::Lt(region);
    } else if var_env.contains("mk") {
        return Language::Mk(region);
    } else if var_env.contains("mg") {
        return Language::Mg(region);
    }

    forth_get_language_helper()
}


fn forth_get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("ms") {
        return Language::Ms(region);
    } else if var_env.contains("ml") {
        return Language::Ml(region);
    } else if var_env.contains("mt") {
        return Language::Mt(region);
    } else if var_env.contains("mi") {
        return Language::Mi(region);
    } else if var_env.contains("mr") {
        return Language::Mr(region);
    } else if var_env.contains("mo") {
        return Language::Mo(region);
    } else if var_env.contains("mn") {
        return Language::Mn(region);
    } else if var_env.contains("na") {
        return Language::Na(region);
    } else if var_env.contains("ne") {
        return Language::Ne(region);
    } else if var_env.contains("no") {
        return Language::No(region);
    } else if var_env.contains("oc") {
        return Language::Oc(region);
    } else if var_env.contains("or") {
        return Language::Or(region);
    } else if var_env.contains("om") {
        return Language::Om(region);
    } else if var_env.contains("ps") {
        return Language::Ps(region);
    } else if var_env.contains("fa") {
        return Language::Fa(region);
    } else if var_env.contains("pl") {
        return Language::Pl(region);
    } else if var_env.contains("pt") {
        return Language::Pt(region);
    } else if var_env.contains("pa") {
        return Language::Pa(region);
    } else if var_env.contains("qu") {
        return Language::Qu(region);
    } else if var_env.contains("rm") {
        return Language::Rm(region);
    } else if var_env.contains("ro") {
        return Language::Ro(region);
    } else if var_env.contains("ru") {
        return Language::Ru(region);
    } else if var_env.contains("sm") {
        return Language::Sm(region);
    } else if var_env.contains("sg") {
        return Language::Sg(region);
    }

    fifth_get_language_helper()
}


fn fifth_get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("sa") {
        return Language::Sa(region);
    } else if var_env.contains("sr") {
        return Language::Sr(region);
    } else if var_env.contains("sh") {
        return Language::Sh(region);
    } else if var_env.contains("st") {
        return Language::St(region);
    } else if var_env.contains("tn") {
        return Language::Tn(region);
    } else if var_env.contains("sn") {
        return Language::Sn(region);
    } else if var_env.contains("sd") {
        return Language::Sd(region);
    } else if var_env.contains("si") {
        return Language::Si(region);
    } else if var_env.contains("ss") {
        return Language::Ss(region);
    } else if var_env.contains("sk") {
        return Language::Sk(region);
    } else if var_env.contains("sl") {
        return Language::Sl(region);
    } else if var_env.contains("so") {
        return Language::So(region);
    } else if var_env.contains("es") {
        return Language::Es(region);
    } else if var_env.contains("su") {
        return Language::Su(region);
    } else if var_env.contains("sw") {
        return Language::Sw(region);
    } else if var_env.contains("sv") {
        return Language::Sv(region);
    } else if var_env.contains("tl") {
        return Language::Tl(region);
    } else if var_env.contains("tg") {
        return Language::Tg(region);
    } else if var_env.contains("ta") {
        return Language::Ta(region);
    } else if var_env.contains("tt") {
        return Language::Tt(region);
    } else if var_env.contains("te") {
        return Language::Te(region);
    } else if var_env.contains("th") {
        return Language::Th(region);
    } else if var_env.contains("bo") {
        return Language::Bo(region);
    } else if var_env.contains("ti") {
        return Language::Ti(region);
    }

    sixth_get_language_helper()
}


fn sixth_get_language_helper() -> Language {
    let var_env: String = get_language_and_country();
    let region: Region = Region::get();

    if var_env.contains("to") {
        return Language::To(region);
    } else if var_env.contains("ts") {
        return Language::Ts(region);
    } else if var_env.contains("tr") {
        return Language::Tr(region);
    } else if var_env.contains("tk") {
        return Language::Tk(region);
    } else if var_env.contains("tw") {
        return Language::Tw(region);
    } else if var_env.contains("uk") {
        return Language::Uk(region);
    } else if var_env.contains("ur") {
        return Language::Ur(region);
    } else if var_env.contains("uz") {
        return Language::Uz(region);
    } else if var_env.contains("vi") {
        return Language::Vi(region);
    } else if var_env.contains("vo") {
        return Language::Vo(region);
    } else if var_env.contains("cy") {
        return Language::Cy(region);
    } else if var_env.contains("wo") {
        return Language::Wo(region);
    } else if var_env.contains("xh") {
        return Language::Xh(region);
    } else if var_env.contains("ji") {
        return Language::Ji(region);
    } else if var_env.contains("yo") {
        return Language::Yo(region);
    } else if var_env.contains("zu") {
        return Language::Zu(region);
    }

    // unknown language
    Language::En(Region::Any)
}

// FIXME: V2: Move `Unknown` variants to the top of the enum.

/// The desktop environment of a system
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DesktopEnv {
    /// Unknown desktop environment
    Unknown(String),

    /// Popular GTK-based desktop environment on Linux
    Gnome,
    /// One of the desktop environments for a specific version of Windows
    Windows,
    /// Linux desktop environment optimized for low resource requirements
    Lxde,
    /// Stacking window manager for X Windows on Linux
    Openbox,
    /// Desktop environment for Linux, BSD and Illumos
    Mate,
    /// Lightweight desktop enivornment for unix-like operating systems
    Xfce,
    /// KDE Plasma desktop enviroment
    // FIXME: Rename to 'Plasma' in whoami 2.0.0
    Kde,
    /// Default desktop environment on Linux Mint
    Cinnamon,
    /// Tiling window manager for Linux
    I3,
    /// Desktop environment for MacOS
    Aqua,
    /// Desktop environment for iOS
    Ios,
    /// Desktop environment for Android
    Android,
    /// Running as Web Assembly on a web page
    WebBrowser,
    /// A desktop environment for a video game console
    Console,
    /// Ubuntu-branded GNOME
    Ubuntu,
    /// Default shell for Fuchsia
    Ermine,
    /// Default desktop environment for Redox
    Orbital,
}

impl Display for DesktopEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::Gnome => "Gnome",
            Self::Windows => "Windows",
            Self::Lxde => "LXDE",
            Self::Openbox => "Openbox",
            Self::Mate => "Mate",
            Self::Xfce => "XFCE",
            Self::Kde => "KDE",
            Self::Cinnamon => "Cinnamon",
            Self::I3 => "I3",
            Self::Aqua => "Aqua",
            Self::Ios => "IOS",
            Self::Android => "Android",
            Self::WebBrowser => "Web Browser",
            Self::Console => "Console",
            Self::Ubuntu => "Ubuntu",
            Self::Ermine => "Ermine",
            Self::Orbital => "Orbital",
            Self::Unknown(a) => a,
        })
    }
}

/// The underlying platform for a system
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum Platform {
    Linux,
    Bsd,
    Windows,
    // FIXME: Non-standard casing; Rename to 'Mac' rather than 'MacOs' in
    // whoami 2.0.0
    MacOS,
    Illumos,
    Ios,
    Android,
    Nintendo,
    Xbox,
    PlayStation,
    Fuchsia,
    Redox,
    Unknown(String),
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::Linux => "Linux",
            Self::Bsd => "BSD",
            Self::Windows => "Windows",
            Self::MacOS => "Mac OS",
            Self::Illumos => "Illumos",
            Self::Ios => "iOS",
            Self::Android => "Android",
            Self::Nintendo => "Nintendo",
            Self::Xbox => "XBox",
            Self::PlayStation => "PlayStation",
            Self::Fuchsia => "Fuchsia",
            Self::Redox => "Redox",
            Self::Unknown(a) => a,
        })
    }
}

/// The architecture of a CPU
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Arch {
    /// ARMv5
    ArmV5,
    /// ARMv6 (Sometimes just referred to as ARM)
    ArmV6,
    /// ARMv7 (May or may not support Neon/Thumb)
    ArmV7,
    /// ARM64 (aarch64)
    Arm64,
    /// i386 (x86)
    I386,
    /// i586 (x86)
    I586,
    /// i686 (x86)
    I686,
    /// X86_64 / Amd64
    X64,
    /// MIPS
    Mips,
    /// MIPS (LE)
    MipsEl,
    /// MIPS64
    Mips64,
    /// MIPS64 (LE)
    Mips64El,
    /// PowerPC
    PowerPc,
    /// PowerPC64
    PowerPc64,
    /// PowerPC64LE
    PowerPc64Le,
    /// 32-bit RISC-V
    Riscv32,
    /// 64-bit RISC-V
    Riscv64,
    /// S390x
    S390x,
    /// SPARC
    Sparc,
    /// SPARC64
    Sparc64,
    /// 32-bit Web Assembly
    Wasm32,
    /// 64-bit Web Assembly
    Wasm64,
    /// Unknown Architecture
    Unknown(String),
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Self::Unknown(_) = self {
            f.write_str("Unknown: ")?;
        }

        f.write_str(match self {
            Self::ArmV5 => "armv5",
            Self::ArmV6 => "armv6",
            Self::ArmV7 => "armv7",
            Self::Arm64 => "arm64",
            Self::I386 => "i386",
            Self::I586 => "i586",
            Self::I686 => "i686",
            Self::Mips => "mips",
            Self::MipsEl => "mipsel",
            Self::Mips64 => "mips64",
            Self::Mips64El => "mips64el",
            Self::PowerPc => "powerpc",
            Self::PowerPc64 => "powerpc64",
            Self::PowerPc64Le => "powerpc64le",
            Self::Riscv32 => "riscv32",
            Self::Riscv64 => "riscv64",
            Self::S390x => "s390x",
            Self::Sparc => "sparc",
            Self::Sparc64 => "sparc64",
            Self::Wasm32 => "wasm32",
            Self::Wasm64 => "wasm64",
            Self::X64 => "x86_64",
            Self::Unknown(arch) => arch,
        })
    }
}

/// The address width of a CPU architecture
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[non_exhaustive]
pub enum Width {
    /// 32 bits
    Bits32,
    /// 64 bits
    Bits64,
}

impl Display for Width {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Width::Bits32 => "32 bits",
            Width::Bits64 => "64 bits",
        })
    }
}

impl Arch {
    /// Get the width of this architecture.
    pub fn width(&self) -> Result<Width> {
        match self {
            Arch::ArmV5
            | Arch::ArmV6
            | Arch::ArmV7
            | Arch::I386
            | Arch::I586
            | Arch::I686
            | Arch::Mips
            | Arch::MipsEl
            | Arch::PowerPc
            | Arch::Riscv32
            | Arch::Sparc
            | Arch::Wasm32 => Ok(Width::Bits32),
            Arch::Arm64
            | Arch::Mips64
            | Arch::Mips64El
            | Arch::PowerPc64
            | Arch::PowerPc64Le
            | Arch::Riscv64
            | Arch::S390x
            | Arch::Sparc64
            | Arch::Wasm64
            | Arch::X64 => Ok(Width::Bits64),
            Arch::Unknown(unknown_arch) => Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Tried getting width of unknown arch ({})",
                    unknown_arch,
                ),
            )),
        }
    }
}

/// Get the CPU Architecture.
#[inline(always)]
pub fn arch() -> Arch {
    platform::arch()
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname()`] most notably in that spaces
/// are not allowed in the username.
#[inline(always)]
pub fn username() -> String {
    fallible::username().unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase())
}

/// Get the user's username.
///
/// On unix-systems this differs from [`realname_os()`] most notably in that
/// spaces are not allowed in the username.
#[inline(always)]
pub fn username_os() -> OsString {
    fallible::username_os()
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_lowercase().into())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname() -> String {
    fallible::realname()
        .or_else(|_| fallible::username())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned())
}

/// Get the user's real (full) name.
#[inline(always)]
pub fn realname_os() -> OsString {
    fallible::realname_os()
        .or_else(|_| fallible::username_os())
        .unwrap_or_else(|_| DEFAULT_USERNAME.to_owned().into())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename() -> String {
    fallible::devicename()
        .or_else(|_| fallible::hostname())
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string())
}

/// Get the device name (also known as "Pretty Name").
///
/// Often used to identify device for bluetooth pairing.
#[inline(always)]
pub fn devicename_os() -> OsString {
    fallible::devicename_os()
        .or_else(|_| fallible::hostname_os())
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_string().into())
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname() -> String {
    fallible::hostname().unwrap_or_else(|_| DEFAULT_HOSTNAME.to_lowercase())
}

/// Get the host device's hostname.
///
/// Limited to a-z (case insensitve), 0-9, and dashes.  This limit also applies
/// to `devicename()` when targeting Windows.  Since the hostname is
/// case-insensitive, this method normalizes to lowercase (unlike
/// [`devicename()`]).
#[inline(always)]
pub fn hostname_os() -> OsString {
    fallible::hostname_os()
        .unwrap_or_else(|_| DEFAULT_HOSTNAME.to_lowercase().into())
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro() -> String {
    fallible::distro().unwrap_or_else(|_| format!("Unknown {}", platform()))
}

/// Get the name of the operating system distribution and (possibly) version.
///
/// Example: "Windows 10" or "Fedora 26 (Workstation Edition)"
#[inline(always)]
pub fn distro_os() -> OsString {
    fallible::distro_os()
        .unwrap_or_else(|_| format!("Unknown {}", platform()).into())
}

/// Get the desktop environment.
///
/// Example: "gnome" or "windows"
#[inline(always)]
pub fn desktop_env() -> DesktopEnv {
    platform::desktop_env()
}

/// Get the platform.
#[inline(always)]
pub fn platform() -> Platform {
    platform::platform()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of two letter language codes (lowercase), optionally
/// followed by a dash (-) and a two letter region code (uppercase).  The most
/// preferred language is returned first, followed by next preferred, and so on.
#[inline(always)]
#[deprecated(note = "use `langs()` instead", since = "1.5.0")]
pub fn lang() -> impl Iterator<Item = String> {
    platform::lang()
}

/// Get the user's preferred language(s).
///
/// Returned as iterator of [`Language`]s wrapped in [`Result`]s.  The most
/// preferred language is returned first, followed by next preferred, and so on.
/// Unrecognized languages may return an error.
#[inline(always)]
pub fn langs() -> impl Iterator<Item = Result<Language>> {
    #[allow(deprecated)]
    lang().map(|string| Ok(Language::__(Box::new(string))))
}
