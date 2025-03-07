use std::fmt::{self, Display, Formatter};

/// Country code for a [`Language`] dialect
///
/// Uses <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[non_exhaustive]
#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Country {
    // FIXME: V2: u32::from_ne_bytes for country codes, with `\0` for unused
    // FIXME: Add aliases up to 3-4 letters, but hidden
    /// Any dialect
    Any,
    /// `US`: United States of America
    #[doc(hidden)]
    Us,
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Any => "**",
            Self::Us => "US",
        })
    }
}

/// A spoken language
///
/// Use [`ToString::to_string()`] to convert to string of two letter lowercase
/// language code followed an forward slash and uppercase country code (example:
/// `en/US`).
///
/// Language codes defined in ISO 639 <https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes>,
/// Country codes defined in ISO 3166 <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
// #[allow(variant_size_differences)]
pub enum Language {
    #[doc(hidden)]
    __(Box<String>),
    /// `en`: English
    #[doc(hidden)]
    En(Country),
    /// `es`: Spanish
    #[doc(hidden)]
    Es(Country),
}

impl Language {
    /// Retrieve the country code for this language dialect.
    pub fn country(&self) -> Country {
        match self {
            Self::__(_) => Country::Any,
            Self::En(country) | Self::Es(country) => *country,
        }
    }
}

// Reads an `language_COUNTRY.Encoding` formatted string into a Language where
// language is a two letter language code and country is a two letter country
// code.
impl<T: AsRef<str>> From<T> for Language {
    // FIXME: Could do less allocation
    fn from(item: T) -> Self {
        let lang = item
            .as_ref()
            .split_terminator('.')
            .next()
            .unwrap_or_default()
            .replace(|x| ['_', '-'].contains(&x), "/");
        Self::__(Box::new(lang))
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::__(code) => f.write_str(code.as_str()),
            Self::En(country) => {
                if *country != Country::Any {
                    f.write_str("en/")?;
                    <Country as Display>::fmt(country, f)
                } else {
                    f.write_str("en")
                }
            }
            Self::Es(country) => {
                if *country != Country::Any {
                    f.write_str("es/")?;
                    <Country as Display>::fmt(country, f)
                } else {
                    f.write_str("es")
                }
            }
        }
    }
}

/// [`Language`] preferences for a user.
///
/// Fields are sorted in order of the user's preference.
///
/// POSIX locale values and GNU nonstandard categories are defined in
/// <https://man7.org/linux/man-pages/man7/locale.7.html>. Windows locale values
/// are defined in <https://learn.microsoft.com/en-us/cpp/c-runtime-library/locale-categories>.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct LanguagePrefs {
    /// Determines general user language preference, should be used in
    /// situations which are not encompassed by other [`LanguagePrefs`].
    pub(crate) fallbacks: Vec<Language>,

    /// Determines collation rules used for sorting and regular expressions,
    /// including character equivalence classes and multicharacter collating
    /// elements.
    pub(crate) collation: Option<Language>,

    /// Determines the interpretation of byte sequences as characters (e.g.,
    /// single versus multibyte characters), character classifications (e.g.,
    /// alphabetic or digit), and the behavior of character classes.
    pub(crate) char_classes: Option<Language>,

    /// Determines the formatting used for monetary-related numeric values,
    /// i.e, the way numbers are usually printed with details such as
    /// decimal point versus decimal comma.
    pub(crate) monetary: Option<Language>,

    /// Determines the language in which messages are
    /// displayed and what an affirmative or negative answer looks
    /// like.
    pub(crate) messages: Option<Language>,

    /// Determines the formatting rules used for nonmonetary numeric values.
    /// For example, the thousands separator and the radix character.
    pub(crate) numeric: Option<Language>,

    /// Determines format and contents of date and time information.
    pub(crate) time: Option<Language>,
}

impl Display for LanguagePrefs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let langs: [(&str, Vec<Language>); 6] = [
            ("Collation", self.collation_langs().collect()),
            ("CharClasses", self.char_class_langs().collect()),
            ("Monetary", self.monetary_langs().collect()),
            ("Messages", self.message_langs().collect()),
            ("Numeric", self.numeric_langs().collect()),
            ("Time", self.time_langs().collect()),
        ];
        for (i, (name, langs)) in langs.iter().enumerate() {
            if i != 0 {
                f.write_str(",")?;
            }
            f.write_str(name)?;
            for (j, lang) in langs.iter().enumerate() {
                if j != 0 {
                    f.write_str(":")?;
                }
                write!(f, "{}", lang)?;
            }
        }

        Ok(())
    }
}

impl LanguagePrefs {
    fn chain_fallbacks<'a>(
        &'a self,
        l: &Option<Language>,
    ) -> impl Iterator<Item = Language> + 'a {
        l.clone().into_iter().chain(self.fallbacks.iter().cloned())
    }

    /// Returns the collation langs of this [`LanguagePrefs`] in order of the
    /// user's preference
    ///
    /// Collation langs are used for sorting and regular expressions,
    /// including character equivalence classes and multicharacter collating
    /// elements.
    pub fn collation_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.collation)
    }

    /// Returns the char class langs of this [`LanguagePrefs`] in order of the
    /// user's preference
    ///
    /// Char class langs determine the interpretation of byte sequences as
    /// characters (e.g., single versus multibyte characters), character
    /// classifications (e.g., alphabetic or digit), and the behavior of
    /// character classes.
    pub fn char_class_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.char_classes)
    }

    /// Returns the monetary langs of this [`LanguagePrefs`] in order of the
    /// user's preference
    ///
    /// Monetary langs determine the formatting used for monetary-related
    /// numeric values, i.e, the way numbers are usually printed with details
    /// such as decimal point versus decimal comma.
    ///
    /// For nonmonetary numeric values, see [`LanguagePrefs::numeric_langs`]
    pub fn monetary_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.monetary)
    }

    /// Returns the messages langs of this [`LanguagePrefs`] in order of the
    /// user's preference
    ///
    /// Message determines the language in which messages are
    /// displayed and what an affirmative or negative answer looks
    /// like.
    pub fn message_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.messages)
    }

    /// Returns the numeric langs of this [`LanguagePrefs`] in order of the
    /// user's preference
    ///
    /// Numeric langs determine the formatting rules used for nonmonetary
    /// numeric values. For example, the thousands separator and the radix
    /// character.
    ///
    /// For monetary formatting, see [`LanguagePrefs::monetary_langs`].
    pub fn numeric_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.numeric)
    }

    /// Returns the time langs of this [`LanguagePrefs`] in order of the user's
    /// preference
    ///
    /// Time langs determine format and contents of date and time information.
    pub fn time_langs(&self) -> impl Iterator<Item = Language> + '_ {
        self.chain_fallbacks(&self.time)
    }
}
