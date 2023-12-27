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
};

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
    Ax,

}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            // Self is a instance of Region
            Self::Any => "**",

            // https://www.iban.com/country-codes
            Region::Custom(value) => {
                match value {
                    004 => "AF",		// Afghanistan
                    008 => "AL",		// Albania
                    012 => "DZ",		// Algeria
                    016 => "AS",		// American Samoa
                    020 => "AD",		// Andorra
                    024 => "AO",		// Angola
                    660 => "AI",		// Anguilla
                    010 => "AQ",		// Antarctica
                    028 => "AG",		// Antigua and Barbuda
                    032 => "AR",		// Argentina
                    051 => "AM",		// Armenia
                    533 => "AW",		// Aruba
                    036 => "AU",		// Australia
                    040 => "AT",		// Austria
                    031 => "AZ",		// Azerbaijan
                    044 => "BS",		// Bahamas (the)
                    048 => "BH",		// Bahrain
                    050 => "BD",		// Bangladesh
                    052 => "BB",		// Barbados
                    112 => "BY",		// Belarus
                    056 => "BE",		// Belgium
                    084 => "BZ",		// Belize
                    204 => "BJ",		// Benin
                    060 => "BM",		// BM BMU 060 Besrmuda
                    064 => "BT",		// BT BTN 064 Bhutan
                    068 => "BO",		// BO BOL 068 Bolivia (Plurinational State of)
                    535 => "BQ",		// BQ BES 535 Bonaire, Sint Eustatius and Saba
                    070 => "BA",		// BA BIH 070 Bosnia and Herzegovina
                    072 => "BW",		// BW BWA 072 Botswana
                    074 => "BV",		// BV BVT 074 Bouvet Island
                    076 => "BR",		// BR BRA 076 Brazil
                    086 => "IO",		// IO IOT 086 British Indian Ocean Territory (the)
                    096 => "BN",		// BN BRN 096 Brunei Darussalam
                    100 => "BG",		// BG BGR 100 Bulgaria
                    854 => "BF",		// BF BFA 854 Burkina Faso
                    108 => "BI",		// BI BDI 108 Burundi
                    132 => "CV",		// CV CPV 132 Cabo Verde
                    116 => "KH",		// KH KHM 116 Cambodia
                    120 => "CM",		// CM CMR 120 Cameroon
                    124 => "CA",		// CA CAN 124 Canada
                    136 => "KY",		// KY CYM 136 Cayman Islands (the)
                    140 => "CF",		// CF CAF 140 Central African Republic (the)
                    148 => "TD",		// TD TCD 148 Chad
                    152 => "CL",		// CL CHL 152 Chile
                    156 => "CN",		// CN CHN 156 China
                    162 => "CX",		// CX CXR 162 Christmas Island
                    166 => "CC",		// CC CCK 166 Cocos (Keeling) Islands (the)
                    170 => "CO",		// CO COL 170 Colombia
                    174 => "KM",		// KM COM 174 Comoros (the)
                    180 => "CD",		// CD COD 180 Congo (the Democratic Republic of the)
                    178 => "CG",		// CG COG 178 Congo (the)
                    184 => "CK",		// CK COK 184 Cook Islands (the)
                    188 => "CR",		// CR CRI 188 Costa Rica
                    191 => "HR",		// HR HRV 191 Croatia
                    192 => "CU",		// CU CUB 192 Cuba
                    531 => "CW",		// CW CUW 531 Curaçao
                    196 => "CY",		// CY CYP 196 Cyprus
                    203 => "CZ",		// CZ CZE 203 Czechia
                    384 => "CI",		// CI CIV 384 Côte d'Ivoire
                    208 => "DK",		// DK DNK 208 Denmark
                    262 => "DJ",		// DJ DJI 262 Djibouti
                    212 => "DM",		// DM DMA 212 Dominica
                    214 => "DO",		// DO DOM 214 Dominican Republic (the)
                    218 => "EC",		// EC ECU 218 Ecuador
                    818 => "EG",		// EG EGY 818 Egypt
                    222 => "SV",		// SV SLV 222 El Salvador
                    226 => "GQ",		// GQ GNQ 226 Equatorial Guinea
                    232 => "ER",		// ER ERI 232 Eritrea
                    233 => "EE",		// EE EST 233 Estonia
                    748 => "SZ",		// SZ SWZ 748 Eswatini
                    231 => "ET",		// ET ETH 231 Ethiopia
                    238 => "FK",		// FK FLK 238 Falkland Islands (the) [Malvinas]
                    234 => "FO",		// FO FRO 234 Faroe Islands (the)
                    242 => "FJ",		// FJ FJI 242 Fiji
                    246 => "FI",		// FI FIN 246 Finland
                    250 => "FR",		// FR FRA 250 France
                    254 => "GF",		// GF GUF 254 French Guiana
                    258 => "PF",		// PF PYF 258 French Polynesia
                    260 => "TF",		// TF ATF 260 French Southern Territories (the)
                    266 => "GA",		// GA GAB 266 Gabon
                    270 => "GM",		// GM GMB 270 Gambia (the)
                    268 => "GE",		// GE GEO 268 Georgia
                    276 => "DE",		// DE DEU 276 Germany
                    288 => "GH",		// GH GHA 288 Ghana
                    292 => "GI",		// GI GIB 292 Gibraltar
                    300 => "GR",		// GR GRC 300 Greece
                    304 => "GL",		// GL GRL 304 Greenland
                    308 => "GD",		// GD GRD 308 Grenada
                    312 => "GP",		// GP GLP 312 Guadeloupe
                    316 => "GU",		// GU GUM 316 Guam
                    320 => "GT",		// GT GTM 320 Guatemala
                    831 => "GG",		// GG GGY 831 Guernsey
                    324 => "GN",		// GN GIN 324 Guinea
                    624 => "GW",		// GW GNB 624 Guinea-Bissau
                    328 => "GY",		// GY GUY 328 Guyana
                    332 => "HT",		// HT HTI 332 Haiti
                    334 => "HM",		// HM HMD 334 Heard Island and McDonald Islands
                    336 => "VA",		// VA VAT 336 Holy See (the)
                    340 => "HN",		// HN HND 340 Honduras
                    344 => "HK",		// HK HKG 344 Hong Kong
                    348 => "HU",		// HU HUN 348 Hungary
                    352 => "IS",		// IS ISL 352 Iceland
                    356 => "IN",		// IN IND 356 India
                    360 => "ID",		// ID IDN 360 Indonesia
                    364 => "IR",		// IR IRN 364 Iran (Islamic Republic of)
                    368 => "IQ",		// IQ IRQ 368 Iraq
                    372 => "IE",		// IE IRL 372 Ireland
                    833 => "IM",		// IM IMN 833 Isle of Man
                    376 => "IL",		// IL ISR 376 Israel
                    380 => "IT",		// IT ITA 380 Italy
                    388 => "JM",		// JM JAM 388 Jamaica
                    392 => "JP",		// JP JPN 392 Japan
                    832 => "JE",		// JE JEY 832 Jersey
                    400 => "JO",		// JO JOR 400 Jordan
                    398 => "KZ",		// KZ KAZ 398 Kazakhstan
                    404 => "KE",		// KE KEN 404 Kenya
                    296 => "KI",		// KI KIR 296 Kiribati
                    408 => "KP",		// KP PRK 408 Korea (the Democratic People's Republic of)
                    410 => "KR",		// KR KOR 410 Korea (the Republic of)
                    414 => "KW",		// KW KWT 414 Kuwait
                    417 => "KG",		// KG KGZ 417 Kyrgyzstan
                    418 => "LA",		// LA LAO 418 Lao People's Democratic Republic (the)
                    428 => "LV",		// LV LVA 428 Latvia
                    422 => "LB",		// LB LBN 422 Lebanon
                    426 => "LS",		// LS LSO 426 Lesotho
                    430 => "LR",		// LR LBR 430 Liberia
                    434 => "LY",		// LY LBY 434 Libya
                    438 => "LI",		// LI LIE 438 Liechtenstein
                    440 => "LT",		// LT LTU 440 Lithuania
                    442 => "LU",		// LU LUX 442 Luxembourg
                    446 => "MO",		// MO MAC 446 Macao
                    450 => "MG",		// MG MDG 450 Madagascar
                    454 => "MW",		// MW MWI 454 Malawi
                    458 => "MY",		// MY MYS 458 Malaysia
                    462 => "MV",		// MV MDV 462 Maldives
                    466 => "ML",		// ML MLI 466 Mali
                    470 => "MT",		// MT MLT 470 Malta
                    584 => "MH",		// MH MHL 584 Marshall Islands (the)
                    474 => "MQ",		// MQ MTQ 474 Martinique
                    478 => "MR",		// MR MRT 478 Mauritania
                    480 => "MU",		// MU MUS 480 Mauritius
                    175 => "YT",		// YT MYT 175 Mayotte
                    484 => "MX",		// MX MEX 484 Mexico
                    583 => "FM",		// FM FSM 583 Micronesia (Federated States of)
                    498 => "MD",		// MD MDA 498 Moldova (the Republic of)
                    492 => "MC",		// MC MCO 492 Monaco
                    496 => "MN",		// MN MNG 496 Mongolia
                    499 => "ME",		// ME MNE 499 Montenegro
                    500 => "MS",		// MS MSR 500 Montserrat
                    504 => "MA",		// MA MAR 504 Morocco
                    508 => "MZ",		// MZ MOZ 508 Mozambique
                    104 => "MM",		// MM MMR 104 Myanmar
                    516 => "NA",		// NA NAM 516 Namibia
                    520 => "NR",		// NR NRU 520 Nauru
                    524 => "NP",		// NP NPL 524 Nepal
                    528 => "NL",		// NL NLD 528 Netherlands (the)
                    540 => "NC",		// NC NCL 540 New Caledonia
                    554 => "NZ",		// NZ NZL 554 New Zealand
                    558 => "NI",		// NI NIC 558 Nicaragua
                    562 => "NE",		// NE NER 562 Niger (the)
                    566 => "NG",		// NG NGA 566 Nigeria
                    570 => "NU",		// NU NIU 570 Niue
                    574 => "NF",		// NF NFK 574 Norfolk Island
                    580 => "MP",		// MP MNP 580 Northern Mariana Islands (the)
                    578 => "NO",		// NO NOR 578 Norway
                    512 => "OM",		// OM OMN 512 Oman
                    586 => "PK",		// PK PAK 586 Pakistan
                    585 => "PW",		// PW PLW 585 Palau
                    275 => "PS",		// PS PSE 275 Palestine, State of
                    591 => "PA",		// PA PAN 591 Panama
                    598 => "PG",		// PG PNG 598 Papua New Guinea
                    600 => "PY",		// PY PRY 600 Paraguay
                    604 => "PE",		// PE PER 604 Peru
                    608 => "PH",		// PH PHL 608 Philippines (the)
                    612 => "PN",		// PN PCN 612 Pitcairn
                    616 => "PL",		// PL POL 616 Poland
                    620 => "PT",		// PT PRT 620 Portugal
                    630 => "PR",		// PR PRI 630 Puerto Rico
                    634 => "QA",		// QA QAT 634 Qatar
                    807 => "MK",		// MK MKD 807 Republic of North Macedonia
                    642 => "RO",		// RO ROU 642 Romania
                    643 => "RU",		// RU RUS 643 Russian Federation (the)
                    646 => "RW",		// RW RWA 646 Rwanda
                    638 => "RE",		// RE REU 638 Réunion
                    652 => "BL",		// BL BLM 652 Saint Barthélemy
                    654 => "SH",		// SH SHN 654 Saint Helena, Ascension and Tristan da Cunha
                    659 => "KN",		// KN KNA 659 Saint Kitts and Nevis
                    662 => "LC",		// LC LCA 662 Saint Lucia
                    663 => "MF",		// MF MAF 663 Saint Martin (French part)
                    666 => "PM",		// PM SPM 666 Saint Pierre and Miquelon
                    670 => "VC",		// VC VCT 670 Saint Vincent and the Grenadines
                    882 => "WS",		// WS WSM 882 Samoa
                    674 => "SM",		// SM SMR 674 San Marino
                    678 => "ST",		// ST STP 678 Sao Tome and Principe
                    682 => "SA",		// SA SAU 682 Saudi Arabia
                    686 => "SN",		// SN SEN 686 Senegal
                    688 => "RS",		// RS SRB 688 Serbia
                    690 => "SC",		// SC SYC 690 Seychelles
                    694 => "SL",		// SL SLE 694 Sierra Leone
                    702 => "SG",		// SG SGP 702 Singapore
                    534 => "SX",		// SX SXM 534 Sint Maarten (Dutch part)
                    703 => "SK",		// SK SVK 703 Slovakia
                    705 => "SI",		// SI SVN 705 Slovenia
                    090 => "SB",		// SB SLB 090 Solomon Islands
                    706 => "SO",		// SO SOM 706 Somalia
                    710 => "ZA",		// ZA ZAF 710 South Africa
                    239 => "GS",		// GS SGS 239 South Georgia and the South Sandwich Islands
                    728 => "SS",		// SS SSD 728 South Sudan
                    724 => "ES",		// ES ESP 724 Spain
                    144 => "LK",		// LK LKA 144 Sri Lanka
                    729 => "SD",		// SD SDN 729 Sudan (the)
                    740 => "SR",		// SR SUR 740 Suriname
                    744 => "SJ",		// SJ SJM 744 Svalbard and Jan Mayen
                    752 => "SE",		// SE SWE 752 Sweden
                    756 => "CH",		// CH CHE 756 Switzerland
                    760 => "SY",		// SY SYR 760 Syrian Arab Republic
                    158 => "TW",		// TW TWN 158 Taiwan (Province of China)
                    762 => "TJ",		// TJ TJK 762 Tajikistan
                    834 => "TZ",		// TZ TZA 834 Tanzania, United Republic of
                    764 => "TH",		// TH THA 764 Thailand
                    626 => "TL",		// TL TLS 626 Timor-Leste
                    768 => "TG",		// TG TGO 768 Togo
                    772 => "TK",		// TK TKL 772 Tokelau
                    776 => "TO",		// TO TON 776 Tonga
                    780 => "TT",		// TT TTO 780 Trinidad and Tobago
                    788 => "TN",		// TN TUN 788 Tunisia
                    792 => "TR",		// TR TUR 792 Turkey
                    795 => "TM",		// TM TKM 795 Turkmenistan
                    796 => "TC",		// TC TCA 796 Turks and Caicos Islands (the)
                    798 => "TV",		// TV TUV 798 Tuvalu
                    800 => "UG",		// UG UGA 800 Uganda
                    804 => "UA",		// UA UKR 804 Ukraine
                    784 => "AE",		// AE ARE 784 United Arab Emirates (the)
                    826 => "GB",		// GB GBR 826 United Kingdom of Great Britain and Northern Ireland (the)
                    581 => "UM",		// UM UMI 581 United States Minor Outlying Islands (the)
                    840 => "US",		// US USA 840 United States of America (the)
                    858 => "UY",		// UY URY 858 Uruguay
                    860 => "UZ",		// UZ UZB 860 Uzbekistan
                    548 => "VU",		// VU VUT 548 Vanuatu
                    862 => "VE",		// VE VEN 862 Venezuela (Bolivarian Republic of)
                    704 => "VN",		// VN VNM 704 Viet Nam
                    092 => "VG",		// VG VGB 092 Virgin Islands (British)
                    850 => "VI",		// VI VIR 850 Virgin Islands (U.S.)
                    876 => "WF",		// WF WLF 876 Wallis and Futuna
                    732 => "EH",		// EH ESH 732 Western Sahara
                    887 => "YE",		// YE YEM 887 Yemen
                    894 => "ZM",		// ZM ZMB 894 Zambia
                    716 => "ZW",		// ZW ZWE 716 Zimbabwe
                    248 => "AX",		// AX ALA 248 Åland Islands
                    _ => "??", // unknow for other cases
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
            Self::Kg => "KG",			// KG KGZ 417 Kyrgyzstan
            Self::La => "LA",			// LA LAO 418 Lao People's Democratic Republic (the)
            Self::Lv => "LV",			// LV LVA 428 Latvia
            Self::Lb => "LB",			// LB LBN 422 Lebanon
            Self::Ls => "LS",			// LS LSO 426 Lesotho
            Self::Lr => "LR",			// LR LBR 430 Liberia
            Self::Ly => "LY",			// LY LBY 434 Libya
            Self::Li => "LI",			// LI LIE 438 Liechtenstein
            Self::Lt => "LT",			// LT LTU 440 Lithuania
            Self::Lu => "LU",			// LU LUX 442 Luxembourg
            Self::Mo => "MO",			// MO MAC 446 Macao
            Self::Mg => "MG",			// MG MDG 450 Madagascar
            Self::Mw => "MW",			// MW MWI 454 Malawi
            Self::My => "MY",			// MY MYS 458 Malaysia
            Self::Mv => "MV",			// MV MDV 462 Maldives
            Self::Ml => "ML",			// ML MLI 466 Mali
            Self::Mt => "MT",			// MT MLT 470 Malta
            Self::Mh => "MH",			// MH MHL 584 Marshall Islands (the)
            Self::Mq => "MQ",			// MQ MTQ 474 Martinique
            Self::Mr => "MR",			// MR MRT 478 Mauritania
            Self::Mu => "MU",			// MU MUS 480 Mauritius
            Self::Yt => "YT",			// YT MYT 175 Mayotte
            Self::Mx => "MX",			// MX MEX 484 Mexico
            Self::Fm => "FM",			// FM FSM 583 Micronesia (Federated States of)
            Self::Md => "MD",			// MD MDA 498 Moldova (the Republic of)
            Self::Mc => "MC",			// MC MCO 492 Monaco
            Self::Mn => "MN",			// MN MNG 496 Mongolia
            Self::Me => "ME",			// ME MNE 499 Montenegro
            Self::Ms => "MS",			// MS MSR 500 Montserrat
            Self::Ma => "MA",			// MA MAR 504 Morocco
            Self::Mz => "MZ",			// MZ MOZ 508 Mozambique
            Self::Mm => "MM",			// MM MMR 104 Myanmar
            Self::Na => "NA",			// NA NAM 516 Namibia
            Self::Nr => "NR",			// NR NRU 520 Nauru
            Self::Np => "NP",			// NP NPL 524 Nepal
            Self::Nl => "NL",			// NL NLD 528 Netherlands (the)
            Self::Nc => "NC",			// NC NCL 540 New Caledonia
            Self::Nz => "NZ",			// NZ NZL 554 New Zealand
            Self::Ni => "NI",			// NI NIC 558 Nicaragua
            Self::Ne => "NE",			// NE NER 562 Niger (the)
            Self::Ng => "NG",			// NG NGA 566 Nigeria
            Self::Nu => "NU",			// NU NIU 570 Niue
            Self::Nf => "NF",			// NF NFK 574 Norfolk Island
            Self::Mp => "MP",			// MP MNP 580 Northern Mariana Islands (the)
            Self::No => "NO",			// NO NOR 578 Norway
            Self::Om => "OM",			// OM OMN 512 Oman
            Self::Pk => "PK",			// PK PAK 586 Pakistan
            Self::Pw => "PW",			// PW PLW 585 Palau
            Self::Ps => "PS",			// PS PSE 275 Palestine, State of
            Self::Pa => "PA",			// PA PAN 591 Panama
            Self::Pg => "PG",			// PG PNG 598 Papua New Guinea
            Self::Py => "PY",			// PY PRY 600 Paraguay
            Self::Pe => "PE",			// PE PER 604 Peru
            Self::Ph => "PH",			// PH PHL 608 Philippines (the)
            Self::Pn => "PN",			// PN PCN 612 Pitcairn
            Self::Pl => "PL",			// PL POL 616 Poland
            Self::Pt => "PT",			// PT PRT 620 Portugal
            Self::Pr => "PR",			// PR PRI 630 Puerto Rico
            Self::Qa => "QA",			// QA QAT 634 Qatar
            Self::Mk => "MK",			// MK MKD 807 Republic of North Macedonia
            Self::Ro => "RO",			// RO ROU 642 Romania
            Self::Ru => "RU",			// RU RUS 643 Russian Federation (the)
            Self::Rw => "RW",			// RW RWA 646 Rwanda
            Self::Re => "RE",			// RE REU 638 Réunion
            Self::Bl => "BL",			// BL BLM 652 Saint Barthélemy
            Self::Sh => "SH",			// SH SHN 654 Saint Helena, Ascension and Tristan da Cunha
            Self::Kn => "KN",			// KN KNA 659 Saint Kitts and Nevis
            Self::Lc => "LC",			// LC LCA 662 Saint Lucia
            Self::Mf => "MF",			// MF MAF 663 Saint Martin (French part)
            Self::Pm => "PM",			// PM SPM 666 Saint Pierre and Miquelon
            Self::Vc => "VC",			// VC VCT 670 Saint Vincent and the Grenadines
            Self::Ws => "WS",			// WS WSM 882 Samoa
            Self::Sm => "SM",			// SM SMR 674 San Marino
            Self::St => "ST",			// ST STP 678 Sao Tome and Principe
            Self::Sa => "SA",			// SA SAU 682 Saudi Arabia
            Self::Sn => "SN",			// SN SEN 686 Senegal
            Self::Rs => "RS",			// RS SRB 688 Serbia
            Self::Sc => "SC",			// SC SYC 690 Seychelles
            Self::Sl => "SL",			// SL SLE 694 Sierra Leone
            Self::Sg => "SG",			// SG SGP 702 Singapore
            Self::Sx => "SX",			// SX SXM 534 Sint Maarten (Dutch part)
            Self::Sk => "SK",			// SK SVK 703 Slovakia
            Self::Si => "SI",			// SI SVN 705 Slovenia
            Self::Sb => "SB",			// SB SLB 090 Solomon Islands
            Self::So => "SO",			// SO SOM 706 Somalia
            Self::Za => "ZA",			// ZA ZAF 710 South Africa
            Self::Gs => "GS",			// GS SGS 239 South Georgia and the South Sandwich Islands
            Self::Ss => "SS",			// SS SSD 728 South Sudan
            Self::Es => "ES",			// ES ESP 724 Spain
            Self::Lk => "LK",			// LK LKA 144 Sri Lanka
            Self::Sd => "SD",			// SD SDN 729 Sudan (the)
            Self::Sr => "SR",			// SR SUR 740 Suriname
            Self::Sj => "SJ",			// SJ SJM 744 Svalbard and Jan Mayen
            Self::Se => "SE",			// SE SWE 752 Sweden
            Self::Ch => "CH",			// CH CHE 756 Switzerland
            Self::Sy => "SY",			// SY SYR 760 Syrian Arab Republic
            Self::Tw => "TW",			// TW TWN 158 Taiwan (Province of China)
            Self::Tj => "TJ",			// TJ TJK 762 Tajikistan
            Self::Tz => "TZ",			// TZ TZA 834 Tanzania, United Republic of
            Self::Th => "TH",			// TH THA 764 Thailand
            Self::Tl => "TL",			// TL TLS 626 Timor-Leste
            Self::Tg => "TG",			// TG TGO 768 Togo
            Self::Tk => "TK",			// TK TKL 772 Tokelau
            Self::To => "TO",			// TO TON 776 Tonga
            Self::Tt => "TT",			// TT TTO 780 Trinidad and Tobago
            Self::Tn => "TN",			// TN TUN 788 Tunisia
            Self::Tr => "TR",			// TR TUR 792 Turkey
            Self::Tm => "TM",			// TM TKM 795 Turkmenistan
            Self::Tc => "TC",			// TC TCA 796 Turks and Caicos Islands (the)
            Self::Tv => "TV",			// TV TUV 798 Tuvalu
            Self::Ug => "UG",			// UG UGA 800 Uganda
            Self::Ua => "UA",			// UA UKR 804 Ukraine
            Self::Ae => "AE",			// AE ARE 784 United Arab Emirates (the)
            Self::Gb => "GB",			// GB GBR 826 United Kingdom of Great Britain and Northern Ireland (the)
            Self::Um => "UM",			// UM UMI 581 United States Minor Outlying Islands (the)
            Self::Us => "US",			// US USA 840 United States of America (the)
            Self::Uy => "UY",			// UY URY 858 Uruguay
            Self::Uz => "UZ",			// UZ UZB 860 Uzbekistan
            Self::Vu => "VU",			// VU VUT 548 Vanuatu
            Self::Ve => "VE",			// VE VEN 862 Venezuela (Bolivarian Republic of)
            Self::Vn => "VN",			// VN VNM 704 Viet Nam
            Self::Vg => "VG",			// VG VGB 092 Virgin Islands (British)
            Self::Vi => "VI",			// VI VIR 850 Virgin Islands (U.S.)
            Self::Wf => "WF",			// WF WLF 876 Wallis and Futuna
            Self::Eh => "EH",			// EH ESH 732 Western Sahara
            Self::Ye => "YE",			// YE YEM 887 Yemen
            Self::Zm => "ZM",			// ZM ZMB 894 Zambia
            Self::Zw => "ZW",			// ZW ZWE 716 Zimbabwe
            Self::Ax => "AX",			// AX ALA 248 Åland Islands
            _ => "**",
        })
    }
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
    /// `en`: English
    #[doc(hidden)]
    En(Region),
    /// `es`: Spanish
    #[doc(hidden)]
    Es(Region),
}

impl Language {
    /// Retrieve the region code for this language dialect.
    pub fn region(&self) -> Region {
        match self {
            Self::__(_) => Region::Any,
            Self::En(region) | Self::Es(region) => *region,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::__(code) => f.write_str(code.as_str()),
            Self::En(region) => {
                if *region != Region::Any {
                    f.write_str("en_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("en")
                }
            }
            Self::Es(region) => {
                if *region != Region::Any {
                    f.write_str("es_")?;
                    <Region as Display>::fmt(region, f)
                } else {
                    f.write_str("es")
                }
            }
        }
    }
}

// FIXME: V2: Move `Unknown` variants to the top of the enum.

/// The desktop environment of a system
#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum DesktopEnv {
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
    /// Unknown desktop environment
    Unknown(String),
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
