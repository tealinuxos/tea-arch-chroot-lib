use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyboard
{
    pub code: String,
    pub name: String,
    pub variant: Vec<Variant>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Variant
{
    pub code: String,
    pub name: String
}

impl Keyboard{
    pub fn list() -> Vec<Keyboard> {

        let mut variants = HashMap::new();

        variants.insert(
            "al".to_string(),
            vec![
                Variant {
                    code: "plisi".to_string(),
                    name: "Albanian (Plisi)".to_string(),
                },
                Variant {
                    code: "veqilharxhi".to_string(),
                    name: "Albanian (Veqilharxhi)".to_string(),
                },
            ],
        );

        variants.insert(
            "am".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Armenian (phonetic)".to_string(),
                },
                Variant {
                    code: "phonetic-alt".to_string(),
                    name: "Armenian (alt. phonetic)".to_string(),
                },
                Variant {
                    code: "eastern".to_string(),
                    name: "Armenian (eastern)".to_string(),
                },
                Variant {
                    code: "eastern-alt".to_string(),
                    name: "Armenian (alt. eastern)".to_string(),
                },
                Variant {
                    code: "western".to_string(),
                    name: "Armenian (western)".to_string(),
                },
            ],
        );

        variants.insert(
            "ara".to_string(),
            vec![
                Variant {
                    code: "digits".to_string(),
                    name: "Arabic (Eastern Arabic numerals)".to_string(),
                },
                Variant {
                    code: "azerty".to_string(),
                    name: "Arabic (AZERTY)".to_string(),
                },
                Variant {
                    code: "azerty_digits".to_string(),
                    name: "Arabic (AZERTY, Eastern Arabic numerals)".to_string(),
                },
                Variant {
                    code: "buckwalter".to_string(),
                    name: "Arabic (Buckwalter)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Arabic (Macintosh)".to_string(),
                },
                Variant {
                    code: "mac-phonetic".to_string(),
                    name: "Arabic (Macintosh, phonetic)".to_string(),
                },
                Variant {
                    code: "olpc".to_string(),
                    name: "Arabic (OLPC)".to_string(),
                },
            ],
        );

        variants.insert(
            "iq".to_string(),
            vec![
                Variant {
                    code: "ku".to_string(),
                    name: "Kurdish (Iraq, Latin Q)".to_string(),
                },
                Variant {
                    code: "ku_alt".to_string(),
                    name: "Kurdish (Iraq, Latin Alt-Q)".to_string(),
                },
                Variant {
                    code: "ku_f".to_string(),
                    name: "Kurdish (Iraq, F)".to_string(),
                },
                Variant {
                    code: "ku_ara".to_string(),
                    name: "Kurdish (Iraq, Arabic-Latin)".to_string(),
                },
            ],
        );

        variants.insert(
            "ma".to_string(),
            vec![
                Variant {
                    code: "tifinagh".to_string(),
                    name: "Berber (Morocco, Tifinagh)".to_string(),
                },
                Variant {
                    code: "tifinagh-alt".to_string(),
                    name: "Berber (Morocco, Tifinagh alt.)".to_string(),
                },
                Variant {
                    code: "tifinagh-alt-phonetic".to_string(),
                    name: "Berber (Morocco, Tifinagh phonetic, alt.)".to_string(),
                },
                Variant {
                    code: "tifinagh-extended".to_string(),
                    name: "Berber (Morocco, Tifinagh extended)".to_string(),
                },
                Variant {
                    code: "tifinagh-phonetic".to_string(),
                    name: "Berber (Morocco, Tifinagh phonetic)".to_string(),
                },
                Variant {
                    code: "tifinagh-extended-phonetic".to_string(),
                    name: "Berber (Morocco, Tifinagh extended phonetic)".to_string(),
                },
                Variant {
                    code: "french".to_string(),
                    name: "French (Morocco)".to_string(),
                },
                Variant {
                    code: "rif".to_string(),
                    name: "Tarifit".to_string(),
                },
            ],
        );

        variants.insert(
            "sy".to_string(),
            vec![
                Variant {
                    code: "syc".to_string(),
                    name: "Syriac".to_string(),
                },
                Variant {
                    code: "syc_phonetic".to_string(),
                    name: "Syriac (phonetic)".to_string(),
                },
                Variant {
                    code: "ku".to_string(),
                    name: "Kurdish (Syria, Latin Q)".to_string(),
                },
                Variant {
                    code: "ku_alt".to_string(),
                    name: "Kurdish (Syria, Latin Alt-Q)".to_string(),
                },
                Variant {
                    code: "ku_f".to_string(),
                    name: "Kurdish (Syria, F)".to_string(),
                },
            ],
        );

        variants.insert(
            "az".to_string(),
            vec![
                Variant {
                    code: "cyrillic".to_string(),
                    name: "Azerbaijani (Cyrillic)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "ml".to_string(),
            vec![
                Variant {
                    code: "fr-oss".to_string(),
                    name: "French (Mali, alt.)".to_string(),
                },
                Variant {
                    code: "us-mac".to_string(),
                    name: "English (Mali, US, Macintosh)".to_string(),
                },
                Variant {
                    code: "us-intl".to_string(),
                    name: "English (Mali, US, intl.)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "bd".to_string(),
            vec![
                Variant {
                    code: "probhat".to_string(),
                    name: "Bangla (Probhat)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "by".to_string(),
            vec![
                Variant {
                    code: "legacy".to_string(),
                    name: "Belarusian (legacy)".to_string(),
                },
                Variant {
                    code: "latin".to_string(),
                    name: "Belarusian (Latin)".to_string(),
                },
                Variant {
                    code: "intl".to_string(),
                    name: "Belarusian (intl.)".to_string(),
                },
                Variant {
                    code: "phonetic".to_string(),
                    name: "Belarusian (phonetic)".to_string(),
                },
                Variant {
                    code: "ru".to_string(),
                    name: "Russian (Belarus)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "be".to_string(),
            vec![
                Variant {
                    code: "oss".to_string(),
                    name: "Belgian (alt.)".to_string(),
                },
                Variant {
                    code: "oss_latin9".to_string(),
                    name: "Belgian (Latin-9 only, alt.)".to_string(),
                },
                Variant {
                    code: "iso-alternate".to_string(),
                    name: "Belgian (ISO, alt.)".to_string(),
                },
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Belgian (no dead keys)".to_string(),
                },
                Variant {
                    code: "wang".to_string(),
                    name: "Belgian (Wang 724 AZERTY)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "dz".to_string(),
            vec![
                Variant {
                    code: "ber".to_string(),
                    name: "Berber (Algeria, Tifinagh)".to_string(),
                },
                Variant {
                    code: "azerty-deadkeys".to_string(),
                    name: "Kabyle (AZERTY, with dead keys)".to_string(),
                },
                Variant {
                    code: "qwerty-gb-deadkeys".to_string(),
                    name: "Kabyle (QWERTY, UK, with dead keys)".to_string(),
                },
                Variant {
                    code: "qwerty-us-deadkeys".to_string(),
                    name: "Kabyle (QWERTY, US, with dead keys)".to_string(),
                },
                Variant {
                    code: "ar".to_string(),
                    name: "Arabic (Algeria)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "az".to_string(),
            vec![
            Variant {
                code: "az".to_string(),
                name: "Azerbaijani".to_string(),
                }
            ]
        );

        variants.insert(
            "ba".to_string(),
            vec![
                Variant {
                    code: "alternatequotes".to_string(),
                    name: "Bosnian (with guillemets)".to_string(),
                },
                Variant {
                    code: "unicode".to_string(),
                    name: "Bosnian (with Bosnian digraphs)".to_string(),
                },
                Variant {
                    code: "unicodeus".to_string(),
                    name: "Bosnian (US, with Bosnian digraphs)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Bosnian (US)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "brai".to_string(),
            vec![
                Variant {
                    code: "left_hand".to_string(),
                    name: "Braille (left-handed)".to_string(),
                },
                Variant {
                    code: "left_hand_invert".to_string(),
                    name: "Braille (left-handed inverted thumb)".to_string(),
                },
                Variant {
                    code: "right_hand".to_string(),
                    name: "Braille (right-handed)".to_string(),
                },
                Variant {
                    code: "right_hand_invert".to_string(),
                    name: "Braille (right-handed inverted thumb)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "bg".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Bulgarian (traditional phonetic)".to_string(),
                },
                Variant {
                    code: "bas_phonetic".to_string(),
                    name: "Bulgarian (new phonetic)".to_string(),
                },
                Variant {
                    code: "bekl".to_string(),
                    name: "Bulgarian (enhanced)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "mm".to_string(),
            vec![
                Variant {
                    code: "zawgyi".to_string(),
                    name: "Burmese (Zawgyi)".to_string(),
                },
                Variant {
                    code: "mnw".to_string(),
                    name: "Mon".to_string(),
                },
                Variant {
                    code: "mnw-a1".to_string(),
                    name: "Mon (A1)".to_string(),
                },
                Variant {
                    code: "shn".to_string(),
                    name: "Shan".to_string(),
                },
                Variant {
                    code: "zgt".to_string(),
                    name: "Shan (Zawgyi)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "cn".to_string(),
            vec![
                Variant {
                    code: "altgr-pinyin".to_string(),
                    name: "Hanyu Pinyin Letters (with AltGr dead keys)".to_string(),
                },
                Variant {
                    code: "mon_trad".to_string(),
                    name: "Mongolian (Bichig)".to_string(),
                },
                Variant {
                    code: "mon_trad_todo".to_string(),
                    name: "Mongolian (Todo)".to_string(),
                },
                Variant {
                    code: "mon_trad_xibe".to_string(),
                    name: "Mongolian (Xibe)".to_string(),
                },
                Variant {
                    code: "mon_trad_manchu".to_string(),
                    name: "Mongolian (Manchu)".to_string(),
                },
                Variant {
                    code: "mon_trad_galik".to_string(),
                    name: "Mongolian (Galik)".to_string(),
                },
                Variant {
                    code: "mon_todo_galik".to_string(),
                    name: "Mongolian (Todo Galik)".to_string(),
                },
                Variant {
                    code: "mon_manchu_galik".to_string(),
                    name: "Mongolian (Manchu Galik)".to_string(),
                },
                Variant {
                    code: "tib".to_string(),
                    name: "Tibetan".to_string(),
                },
                Variant {
                    code: "tib_asciinum".to_string(),
                    name: "Tibetan (with ASCII numerals)".to_string(),
                },
                Variant {
                    code: "ug".to_string(),
                    name: "Uyghur".to_string(),
                },
            ],
        );

        variants.insert(
            "hr".to_string(),
            vec![
                Variant {
                    code: "alternatequotes".to_string(),
                    name: "Croatian (with guillemets)".to_string(),
                },
                Variant {
                    code: "unicode".to_string(),
                    name: "Croatian (with Croatian digraphs)".to_string(),
                },
                Variant {
                    code: "unicodeus".to_string(),
                    name: "Croatian (US, with Croatian digraphs)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Croatian (US)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "cz".to_string(),
            vec![
                Variant {
                    code: "bksl".to_string(),
                    name: "Czech (extra backslash)".to_string(),
                },
                Variant {
                    code: "qwerty".to_string(),
                    name: "Czech (QWERTY)".to_string(),
                },
                Variant {
                    code: "qwerty_bksl".to_string(),
                    name: "Czech (QWERTY, extra backslash)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Czech (QWERTZ, Windows)".to_string(),
                },
                Variant {
                    code: "winkeys-qwerty".to_string(),
                    name: "Czech (QWERTY, Windows)".to_string(),
                },
                Variant {
                    code: "qwerty-mac".to_string(),
                    name: "Czech (QWERTY, Macintosh)".to_string(),
                },
                Variant {
                    code: "ucw".to_string(),
                    name: "Czech (UCW, only accented letters)".to_string(),
                },
                Variant {
                    code: "dvorak-ucw".to_string(),
                    name: "Czech (US, Dvorak, UCW support)".to_string(),
                },
                Variant {
                    code: "rus".to_string(),
                    name: "Russian (Czechia, phonetic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "dk".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Danish (no dead keys)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Danish (Windows)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Danish (Macintosh)".to_string(),
                },
                Variant {
                    code: "mac_nodeadkeys".to_string(),
                    name: "Danish (Macintosh, no dead keys)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Danish (Dvorak)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "us".to_string(),
            vec![
                Variant {
                    code: "euro".to_string(),
                    name: "English (US, euro on 5)".to_string(),
                },
                Variant {
                    code: "intl".to_string(),
                    name: "English (US, intl., with dead keys)".to_string(),
                },
                Variant {
                    code: "alt-intl".to_string(),
                    name: "English (US, alt. intl.)".to_string(),
                },
                Variant {
                    code: "altgr-intl".to_string(),
                    name: "English (intl., with AltGr dead keys)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "English (Macintosh)".to_string(),
                },
                Variant {
                    code: "colemak".to_string(),
                    name: "English (Colemak)".to_string(),
                },
                Variant {
                    code: "colemak_dh".to_string(),
                    name: "English (Colemak-DH)".to_string(),
                },
                Variant {
                    code: "colemak_dh_wide".to_string(),
                    name: "English (Colemak-DH Wide)".to_string(),
                },
                Variant {
                    code: "colemak_dh_ortho".to_string(),
                    name: "English (Colemak-DH Ortholinear)".to_string(),
                },
                Variant {
                    code: "colemak_dh_iso".to_string(),
                    name: "English (Colemak-DH ISO)".to_string(),
                },
                Variant {
                    code: "colemak_dh_wide_iso".to_string(),
                    name: "English (Colemak-DH Wide ISO)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "English (Dvorak)".to_string(),
                },
                Variant {
                    code: "dvorak-intl".to_string(),
                    name: "English (Dvorak, intl., with dead keys)".to_string(),
                },
                Variant {
                    code: "dvorak-alt-intl".to_string(),
                    name: "English (Dvorak, alt. intl.)".to_string(),
                },
                Variant {
                    code: "dvorak-l".to_string(),
                    name: "English (Dvorak, left-handed)".to_string(),
                },
                Variant {
                    code: "dvorak-r".to_string(),
                    name: "English (Dvorak, right-handed)".to_string(),
                },
                Variant {
                    code: "dvorak-classic".to_string(),
                    name: "English (classic Dvorak)".to_string(),
                },
                Variant {
                    code: "dvp".to_string(),
                    name: "English (programmer Dvorak)".to_string(),
                },
                Variant {
                    code: "dvorak-mac".to_string(),
                    name: "English (Dvorak, Macintosh)".to_string(),
                },
                Variant {
                    code: "norman".to_string(),
                    name: "English (Norman)".to_string(),
                },
                Variant {
                    code: "symbolic".to_string(),
                    name: "English (US, Symbolic)".to_string(),
                },
                Variant {
                    code: "workman".to_string(),
                    name: "English (Workman)".to_string(),
                },
                Variant {
                    code: "workman-intl".to_string(),
                    name: "English (Workman, intl., with dead keys)".to_string(),
                },
                Variant {
                    code: "olpc2".to_string(),
                    name: "English (the divide/multiply toggle the layout)".to_string(),
                },
                Variant {
                    code: "chr".to_string(),
                    name: "English (US, Cherokee)".to_string(),
                },
                Variant {
                    code: "haw".to_string(),
                    name: "English (US, Hawaiian)".to_string(),
                },
                Variant {
                    code: "rus".to_string(),
                    name: "Russian (US, phonetic)".to_string(),
                },
                Variant {
                    code: "hbs".to_string(),
                    name: "Serbo-Croatian (US)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "fi".to_string(),
            vec![
                Variant {
                    code: "winkeys".to_string(),
                    name: "Finnish (Windows)".to_string(),
                },
                Variant {
                    code: "classic".to_string(),
                    name: "Finnish (classic)".to_string(),
                },
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Finnish (classic, no dead keys)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Finnish (Macintosh)".to_string(),
                },
                Variant {
                    code: "smi".to_string(),
                    name: "Northern Saami (Finland)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "fr".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "French (no dead keys)".to_string(),
                },
                Variant {
                    code: "oss".to_string(),
                    name: "French (alt.)".to_string(),
                },
                Variant {
                    code: "oss_nodeadkeys".to_string(),
                    name: "French (alt., no dead keys)".to_string(),
                },
                Variant {
                    code: "oss_latin9".to_string(),
                    name: "French (alt., Latin-9 only)".to_string(),
                },
                Variant {
                    code: "latin9".to_string(),
                    name: "French (legacy, alt.)".to_string(),
                },
                Variant {
                    code: "latin9_nodeadkeys".to_string(),
                    name: "French (legacy, alt., no dead keys)".to_string(),
                },
                Variant {
                    code: "azerty".to_string(),
                    name: "French (AZERTY)".to_string(),
                },
                Variant {
                    code: "afnor".to_string(),
                    name: "French (AZERTY, AFNOR)".to_string(),
                },
                Variant {
                    code: "bepo".to_string(),
                    name: "French (BEPO)".to_string(),
                },
                Variant {
                    code: "bepo_latin9".to_string(),
                    name: "French (BEPO, Latin-9 only)".to_string(),
                },
                Variant {
                    code: "bepo_afnor".to_string(),
                    name: "French (BEPO, AFNOR)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "French (Dvorak)".to_string(),
                },
                Variant {
                    code: "ergol".to_string(),
                    name: "French (Ergo‑L)".to_string(),
                },
                Variant {
                    code: "ergol_iso".to_string(),
                    name: "French (Ergo‑L, ISO variant)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "French (Macintosh)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "French (US)".to_string(),
                },
                Variant {
                    code: "bre".to_string(),
                    name: "Breton (France)".to_string(),
                },
                Variant {
                    code: "oci".to_string(),
                    name: "Occitan".to_string(),
                },
                Variant {
                    code: "geo".to_string(),
                    name: "Georgian (France, AZERTY Tskapo)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "ge".to_string(),
            vec![
                Variant {
                    code: "ergonomic".to_string(),
                    name: "Georgian (ergonomic)".to_string(),
                },
                Variant {
                    code: "mess".to_string(),
                    name: "Georgian (MESS)".to_string(),
                },
                Variant {
                    code: "os".to_string(),
                    name: "Ossetian (Georgia)".to_string(),
                },
                Variant {
                    code: "ru".to_string(),
                    name: "Russian (Georgia)".to_string(),
                },
            ],
        );
    
        variants.insert(
            "de".to_string(),
            vec![
                Variant {
                    code: "deadacute".to_string(),
                    name: "German (dead acute)".to_string(),
                },
                Variant {
                    code: "deadgraveacute".to_string(),
                    name: "German (dead grave acute)".to_string(),
                },
                Variant {
                    code: "deadtilde".to_string(),
                    name: "German (dead tilde)".to_string(),
                },
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "German (no dead keys)".to_string(),
                },
                Variant {
                    code: "e1".to_string(),
                    name: "German (E1)".to_string(),
                },
                Variant {
                    code: "e2".to_string(),
                    name: "German (E2)".to_string(),
                },
                Variant {
                    code: "T3".to_string(),
                    name: "German (T3)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "German (US)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "German (Dvorak)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "German (Macintosh)".to_string(),
                },
                Variant {
                    code: "mac_nodeadkeys".to_string(),
                    name: "German (Macintosh, no dead keys)".to_string(),
                },
                Variant {
                    code: "neo".to_string(),
                    name: "German (Neo 2)".to_string(),
                },
                Variant {
                    code: "qwerty".to_string(),
                    name: "German (QWERTY)".to_string(),
                },
                Variant {
                    code: "dsb".to_string(),
                    name: "Lower Sorbian".to_string(),
                },
                Variant {
                    code: "dsb_qwertz".to_string(),
                    name: "Lower Sorbian (QWERTZ)".to_string(),
                },
                Variant {
                    code: "ro".to_string(),
                    name: "Romanian (Germany)".to_string(),
                },
                Variant {
                    code: "ro_nodeadkeys".to_string(),
                    name: "Romanian (Germany, no dead keys)".to_string(),
                },
                Variant {
                    code: "ru".to_string(),
                    name: "Russian (Germany, phonetic)".to_string(),
                },
                Variant {
                    code: "tr".to_string(),
                    name: "Turkish (Germany)".to_string(),
                },
            ],
        );

        variants.insert(
            "at".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "German (Austria, no dead keys)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "German (Austria, Macintosh)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ch".to_string(),
            vec![
                Variant {
                    code: "de_nodeadkeys".to_string(),
                    name: "German (Switzerland, no dead keys)".to_string(),
                },
                Variant {
                    code: "de_mac".to_string(),
                    name: "German (Switzerland, Macintosh)".to_string(),
                },
                Variant {
                    code: "legacy".to_string(),
                    name: "German (Switzerland, legacy)".to_string(),
                },
                Variant {
                    code: "fr".to_string(),
                    name: "French (Switzerland)".to_string(),
                },
                Variant {
                    code: "fr_nodeadkeys".to_string(),
                    name: "French (Switzerland, no dead keys)".to_string(),
                },
                Variant {
                    code: "fr_mac".to_string(),
                    name: "French (Switzerland, Macintosh)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "gr".to_string(),
            vec![
                Variant {
                    code: "simple".to_string(),
                    name: "Greek (simple)".to_string(),
                },
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Greek (no dead keys)".to_string(),
                },
                Variant {
                    code: "polytonic".to_string(),
                    name: "Greek (polytonic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "il".to_string(),
            vec![
                Variant {
                    code: "si2".to_string(),
                    name: "Hebrew (SI-1452-2)".to_string(),
                },
                Variant {
                    code: "lyx".to_string(),
                    name: "Hebrew (lyx)".to_string(),
                },
                Variant {
                    code: "phonetic".to_string(),
                    name: "Hebrew (phonetic)".to_string(),
                },
                Variant {
                    code: "biblical".to_string(),
                    name: "Hebrew (Biblical, Tiro)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "hu".to_string(),
            vec![
                Variant {
                    code: "standard".to_string(),
                    name: "Hungarian (standard)".to_string(),
                },
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Hungarian (no dead keys)".to_string(),
                },
                Variant {
                    code: "qwerty".to_string(),
                    name: "Hungarian (QWERTY)".to_string(),
                },
                Variant {
                    code: "101_qwertz_comma_dead".to_string(),
                    name: "Hungarian (QWERTZ, 101-key, comma, dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwertz_comma_nodead".to_string(),
                    name: "Hungarian (QWERTZ, 101-key, comma, no dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwertz_dot_dead".to_string(),
                    name: "Hungarian (QWERTZ, 101-key, dot, dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwertz_dot_nodead".to_string(),
                    name: "Hungarian (QWERTZ, 101-key, dot, no dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwerty_comma_dead".to_string(),
                    name: "Hungarian (QWERTY, 101-key, comma, dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwerty_comma_nodead".to_string(),
                    name: "Hungarian (QWERTY, 101-key, comma, no dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwerty_dot_dead".to_string(),
                    name: "Hungarian (QWERTY, 101-key, dot, dead keys)".to_string(),
                },
                Variant {
                    code: "101_qwerty_dot_nodead".to_string(),
                    name: "Hungarian (QWERTY, 101-key, dot, no dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwertz_comma_dead".to_string(),
                    name: "Hungarian (QWERTZ, 102-key, comma, dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwertz_comma_nodead".to_string(),
                    name: "Hungarian (QWERTZ, 102-key, comma, no dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwertz_dot_dead".to_string(),
                    name: "Hungarian (QWERTZ, 102-key, dot, dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwertz_dot_nodead".to_string(),
                    name: "Hungarian (QWERTZ, 102-key, dot, no dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwerty_comma_dead".to_string(),
                    name: "Hungarian (QWERTY, 102-key, comma, dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwerty_comma_nodead".to_string(),
                    name: "Hungarian (QWERTY, 102-key, comma, no dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwerty_dot_dead".to_string(),
                    name: "Hungarian (QWERTY, 102-key, dot, dead keys)".to_string(),
                },
                Variant {
                    code: "102_qwerty_dot_nodead".to_string(),
                    name: "Hungarian (QWERTY, 102-key, dot, no dead keys)".to_string(),
                },
            ],
        );

        variants.insert(
            "is".to_string(),
            vec![
                Variant {
                    code: "mac_legacy".to_string(),
                    name: "Icelandic (Macintosh, legacy)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Icelandic (Macintosh)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Icelandic (Dvorak)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "in".to_string(),
            vec![
                Variant {
                    code: "asm-kagapa".to_string(),
                    name: "Assamese (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "ben".to_string(),
                    name: "Bangla (India)".to_string(),
                },
                Variant {
                    code: "ben_probhat".to_string(),
                    name: "Bangla (India, Probhat)".to_string(),
                },
                Variant {
                    code: "ben_baishakhi".to_string(),
                    name: "Bangla (India, Baishakhi)".to_string(),
                },
                Variant {
                    code: "ben_bornona".to_string(),
                    name: "Bangla (India, Bornona)".to_string(),
                },
                Variant {
                    code: "ben-kagapa".to_string(),
                    name: "Bangla (India, KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "ben_gitanjali".to_string(),
                    name: "Bangla (India, Gitanjali)".to_string(),
                },
                Variant {
                    code: "ben_inscript".to_string(),
                    name: "Bangla (India, Baishakhi InScript)".to_string(),
                },
                Variant {
                    code: "eng".to_string(),
                    name: "English (India, with rupee)".to_string(),
                },
                Variant {
                    code: "guj".to_string(),
                    name: "Gujarati".to_string(),
                },
                Variant {
                    code: "guj-kagapa".to_string(),
                    name: "Gujarati (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "bolnagri".to_string(),
                    name: "Hindi (Bolnagri)".to_string(),
                },
                Variant {
                    code: "hin-wx".to_string(),
                    name: "Hindi (Wx)".to_string(),
                },
                Variant {
                    code: "hin-kagapa".to_string(),
                    name: "Hindi (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "kan".to_string(),
                    name: "Kannada".to_string(),
                },
                Variant {
                    code: "kan-kagapa".to_string(),
                    name: "Kannada (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "mal".to_string(),
                    name: "Malayalam".to_string(),
                },
                Variant {
                    code: "mal_lalitha".to_string(),
                    name: "Malayalam (Lalitha)".to_string(),
                },
                Variant {
                    code: "mal_enhanced".to_string(),
                    name: "Malayalam (enhanced InScript, with rupee)".to_string(),
                },
                Variant {
                    code: "mal_poorna".to_string(),
                    name: "Malayalam (Poorna, extended InScript)".to_string(),
                },
                Variant {
                    code: "mni".to_string(),
                    name: "Manipuri (Meitei)".to_string(),
                },
                Variant {
                    code: "mar-kagapa".to_string(),
                    name: "Marathi (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "marathi".to_string(),
                    name: "Marathi (enhanced InScript)".to_string(),
                },
                Variant {
                    code: "ori".to_string(),
                    name: "Oriya".to_string(),
                },
                Variant {
                    code: "ori-bolnagri".to_string(),
                    name: "Oriya (Bolnagri)".to_string(),
                },
                Variant {
                    code: "ori-wx".to_string(),
                    name: "Oriya (Wx)".to_string(),
                },
                Variant {
                    code: "guru".to_string(),
                    name: "Punjabi (Gurmukhi)".to_string(),
                },
                Variant {
                    code: "jhelum".to_string(),
                    name: "Punjabi (Gurmukhi Jhelum)".to_string(),
                },
                Variant {
                    code: "san-kagapa".to_string(),
                    name: "Sanskrit (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "sat".to_string(),
                    name: "Santali (Ol Chiki)".to_string(),
                },
                Variant {
                    code: "tamilnet".to_string(),
                    name: "Tamil (TamilNet '99)".to_string(),
                },
                Variant {
                    code: "tamilnet_tamilnumbers".to_string(),
                    name: "Tamil (TamilNet '99 with Tamil numerals)".to_string(),
                },
                Variant {
                    code: "tamilnet_TAB".to_string(),
                    name: "Tamil (TamilNet '99, TAB encoding)".to_string(),
                },
                Variant {
                    code: "tamilnet_TSCII".to_string(),
                    name: "Tamil (TamilNet '99, TSCII encoding)".to_string(),
                },
                Variant {
                    code: "tam".to_string(),
                    name: "Tamil (InScript, with Arabic numerals)".to_string(),
                },
                Variant {
                    code: "tam_tamilnumbers".to_string(),
                    name: "Tamil (InScript, with Tamil numerals)".to_string(),
                },
                Variant {
                    code: "tel".to_string(),
                    name: "Telugu".to_string(),
                },
                Variant {
                    code: "tel-kagapa".to_string(),
                    name: "Telugu (KaGaPa, phonetic)".to_string(),
                },
                Variant {
                    code: "tel-sarala".to_string(),
                    name: "Telugu (Sarala)".to_string(),
                },
                Variant {
                    code: "urd-phonetic".to_string(),
                    name: "Urdu (phonetic)".to_string(),
                },
                Variant {
                    code: "urd-phonetic3".to_string(),
                    name: "Urdu (alt. phonetic)".to_string(),
                },
                Variant {
                    code: "urd-winkeys".to_string(),
                    name: "Urdu (Windows)".to_string(),
                },
                Variant {
                    code: "iipa".to_string(),
                    name: "Indic IPA".to_string(),
                },
            ],
        );
        
        variants.insert(
            "id".to_string(),
            vec![
                Variant {
                    code: "melayu-phonetic".to_string(),
                    name: "Indonesian (Arab Melayu, phonetic)".to_string(),
                },
                Variant {
                    code: "melayu-phoneticx".to_string(),
                    name: "Indonesian (Arab Melayu, extended phonetic)".to_string(),
                },
                Variant {
                    code: "pegon-phonetic".to_string(),
                    name: "Indonesian (Arab Pegon, phonetic)".to_string(),
                },
                Variant {
                    code: "javanese".to_string(),
                    name: "Javanese".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ie".to_string(),
            vec![
                Variant {
                    code: "UnicodeExpert".to_string(),
                    name: "Irish (UnicodeExpert)".to_string(),
                },
                Variant {
                    code: "CloGaelach".to_string(),
                    name: "Irish (CloGaelach)".to_string(),
                },
                Variant {
                    code: "ogam".to_string(),
                    name: "Ogham".to_string(),
                },
                Variant {
                    code: "ogam_is434".to_string(),
                    name: "Ogham (IS434)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "it".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Italian (no dead keys)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Italian (Windows)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Italian (Macintosh)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Italian (US)".to_string(),
                },
                Variant {
                    code: "ibm".to_string(),
                    name: "Italian (IBM 142)".to_string(),
                },
                Variant {
                    code: "fur".to_string(),
                    name: "Friulian (Italy)".to_string(),
                },
                Variant {
                    code: "scn".to_string(),
                    name: "Sicilian".to_string(),
                },
                Variant {
                    code: "geo".to_string(),
                    name: "Georgian (Italy)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "jp".to_string(),
            vec![
                Variant {
                    code: "kana".to_string(),
                    name: "Japanese (Kana)".to_string(),
                },
                Variant {
                    code: "kana86".to_string(),
                    name: "Japanese (Kana 86)".to_string(),
                },
                Variant {
                    code: "OADG109A".to_string(),
                    name: "Japanese (OADG 109A)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Japanese (Macintosh)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Japanese (Dvorak)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "kz".to_string(),
            vec![
                Variant {
                    code: "kazrus".to_string(),
                    name: "Kazakh (with Russian)".to_string(),
                },
                Variant {
                    code: "ext".to_string(),
                    name: "Kazakh (extended)".to_string(),
                },
                Variant {
                    code: "latin".to_string(),
                    name: "Kazakh (Latin)".to_string(),
                },
                Variant {
                    code: "ruskaz".to_string(),
                    name: "Russian (Kazakhstan, with Kazakh)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "kr".to_string(),
            vec![
                Variant {
                    code: "kr104".to_string(),
                    name: "Korean (101/104-key compatible)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "kg".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Kyrgyz (phonetic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "la".to_string(),
            vec![
                Variant {
                    code: "stea".to_string(),
                    name: "Lao (STEA)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "lv".to_string(),
            vec![
                Variant {
                    code: "apostrophe".to_string(),
                    name: "Latvian (apostrophe)".to_string(),
                },
                Variant {
                    code: "tilde".to_string(),
                    name: "Latvian (tilde)".to_string(),
                },
                Variant {
                    code: "fkey".to_string(),
                    name: "Latvian (F)".to_string(),
                },
                Variant {
                    code: "modern".to_string(),
                    name: "Latvian (Modern Latin)".to_string(),
                },
                Variant {
                    code: "modern-cyr".to_string(),
                    name: "Latvian (Modern Cyrillic)".to_string(),
                },
                Variant {
                    code: "ergonomic".to_string(),
                    name: "Latvian (ergonomic, ŪGJRMV)".to_string(),
                },
                Variant {
                    code: "adapted".to_string(),
                    name: "Latvian (adapted)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "lt".to_string(),
            vec![
                Variant {
                    code: "std".to_string(),
                    name: "Lithuanian (standard)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Lithuanian (US)".to_string(),
                },
                Variant {
                    code: "ibm".to_string(),
                    name: "Lithuanian (IBM)".to_string(),
                },
                Variant {
                    code: "lekp".to_string(),
                    name: "Lithuanian (LEKP)".to_string(),
                },
                Variant {
                    code: "lekpa".to_string(),
                    name: "Lithuanian (LEKPa)".to_string(),
                },
                Variant {
                    code: "ratise".to_string(),
                    name: "Lithuanian (Ratise)".to_string(),
                },
                Variant {
                    code: "sgs".to_string(),
                    name: "Samogitian".to_string(),
                },
            ],
        );
        
        variants.insert(
            "mk".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Macedonian (no dead keys)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "my".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Malay (Jawi, phonetic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "mt".to_string(),
            vec![
                Variant {
                    code: "us".to_string(),
                    name: "Maltese (US)".to_string(),
                },
                Variant {
                    code: "alt-us".to_string(),
                    name: "Maltese (US, with AltGr overrides)".to_string(),
                },
                Variant {
                    code: "alt-gb".to_string(),
                    name: "Maltese (UK, with AltGr overrides)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "md".to_string(),
            vec![
                Variant {
                    code: "gag".to_string(),
                    name: "Gagauz (Moldova)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "me".to_string(),
            vec![
                Variant {
                    code: "cyrillic".to_string(),
                    name: "Montenegrin (Cyrillic)".to_string(),
                },
                Variant {
                    code: "cyrillicyz".to_string(),
                    name: "Montenegrin (Cyrillic, ZE and ZHE swapped)".to_string(),
                },
                Variant {
                    code: "cyrillicalternatequotes".to_string(),
                    name: "Montenegrin (Cyrillic, with guillemets)".to_string(),
                },
                Variant {
                    code: "latinunicode".to_string(),
                    name: "Montenegrin (Latin, Unicode)".to_string(),
                },
                Variant {
                    code: "latinyz".to_string(),
                    name: "Montenegrin (Latin, QWERTY)".to_string(),
                },
                Variant {
                    code: "latinunicodeyz".to_string(),
                    name: "Montenegrin (Latin, Unicode, QWERTY)".to_string(),
                },
                Variant {
                    code: "latinalternatequotes".to_string(),
                    name: "Montenegrin (Latin, with guillemets)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "no".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Norwegian (no dead keys)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Norwegian (Windows)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Norwegian (Macintosh)".to_string(),
                },
                Variant {
                    code: "mac_nodeadkeys".to_string(),
                    name: "Norwegian (Macintosh, no dead keys)".to_string(),
                },
                Variant {
                    code: "colemak".to_string(),
                    name: "Norwegian (Colemak)".to_string(),
                },
                Variant {
                    code: "colemak_dh".to_string(),
                    name: "Norwegian (Colemak-DH)".to_string(),
                },
                Variant {
                    code: "colemak_dh_wide".to_string(),
                    name: "Norwegian (Colemak-DH Wide)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Norwegian (Dvorak)".to_string(),
                },
                Variant {
                    code: "smi".to_string(),
                    name: "Northern Saami (Norway)".to_string(),
                },
                Variant {
                    code: "smi_nodeadkeys".to_string(),
                    name: "Northern Saami (Norway, no dead keys)".to_string(),
                },
            ],
        );

        variants.insert(
            "ir".to_string(),
            vec![
                Variant {
                    code: "pes_keypad".to_string(),
                    name: "Persian (with Persian keypad)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Persian (Windows)".to_string(),
                },
                Variant {
                    code: "azb".to_string(),
                    name: "Azerbaijani (Iran)".to_string(),
                },
                Variant {
                    code: "ku".to_string(),
                    name: "Kurdish (Iran, Latin Q)".to_string(),
                },
                Variant {
                    code: "ku_alt".to_string(),
                    name: "Kurdish (Iran, Latin Alt-Q)".to_string(),
                },
                Variant {
                    code: "ku_f".to_string(),
                    name: "Kurdish (Iran, F)".to_string(),
                },
                Variant {
                    code: "ku_ara".to_string(),
                    name: "Kurdish (Iran, Arabic-Latin)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "pl".to_string(),
            vec![
                Variant {
                    code: "legacy".to_string(),
                    name: "Polish (legacy)".to_string(),
                },
                Variant {
                    code: "qwertz".to_string(),
                    name: "Polish (QWERTZ)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Polish (Dvorak)".to_string(),
                },
                Variant {
                    code: "dvorak_quotes".to_string(),
                    name: "Polish (Dvorak, with Polish quotes on quotemark key)".to_string(),
                },
                Variant {
                    code: "dvorak_altquotes".to_string(),
                    name: "Polish (Dvorak, with Polish quotes on key 1)".to_string(),
                },
                Variant {
                    code: "dvp".to_string(),
                    name: "Polish (programmer Dvorak)".to_string(),
                },
                Variant {
                    code: "csb".to_string(),
                    name: "Kashubian".to_string(),
                },
                Variant {
                    code: "szl".to_string(),
                    name: "Silesian".to_string(),
                },
                Variant {
                    code: "ru_phonetic_dvorak".to_string(),
                    name: "Russian (Poland, phonetic Dvorak)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "pt".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Portuguese (no dead keys)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Portuguese (Macintosh)".to_string(),
                },
                Variant {
                    code: "mac_nodeadkeys".to_string(),
                    name: "Portuguese (Macintosh, no dead keys)".to_string(),
                },
                Variant {
                    code: "nativo".to_string(),
                    name: "Portuguese (Nativo)".to_string(),
                },
                Variant {
                    code: "nativo-us".to_string(),
                    name: "Portuguese (Nativo for US keyboards)".to_string(),
                },
                Variant {
                    code: "nativo-epo".to_string(),
                    name: "Esperanto (Portugal, Nativo)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "br".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Portuguese (Brazil, no dead keys)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Portuguese (Brazil, Dvorak)".to_string(),
                },
                Variant {
                    code: "nativo".to_string(),
                    name: "Portuguese (Brazil, Nativo)".to_string(),
                },
                Variant {
                    code: "nativo-us".to_string(),
                    name: "Portuguese (Brazil, Nativo for US keyboards)".to_string(),
                },
                Variant {
                    code: "thinkpad".to_string(),
                    name: "Portuguese (Brazil, IBM/Lenovo ThinkPad)".to_string(),
                },
                Variant {
                    code: "nativo-epo".to_string(),
                    name: "Esperanto (Brazil, Nativo)".to_string(),
                },
                Variant {
                    code: "rus".to_string(),
                    name: "Russian (Brazil, phonetic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ro".to_string(),
            vec![
                Variant {
                    code: "std".to_string(),
                    name: "Romanian (standard)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Romanian (Windows)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ru".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Russian (phonetic)".to_string(),
                },
                Variant {
                    code: "phonetic_winkeys".to_string(),
                    name: "Russian (phonetic, Windows)".to_string(),
                },
                Variant {
                    code: "phonetic_YAZHERTY".to_string(),
                    name: "Russian (phonetic, YAZHERTY)".to_string(),
                },
                Variant {
                    code: "phonetic_azerty".to_string(),
                    name: "Russian (phonetic, AZERTY)".to_string(),
                },
                Variant {
                    code: "phonetic_dvorak".to_string(),
                    name: "Russian (phonetic, Dvorak)".to_string(),
                },
                Variant {
                    code: "typewriter".to_string(),
                    name: "Russian (typewriter)".to_string(),
                },
                Variant {
                    code: "ruchey_ru".to_string(),
                    name: "Russian (engineering, RU)".to_string(),
                },
                Variant {
                    code: "ruchey_en".to_string(),
                    name: "Russian (engineering, EN)".to_string(),
                },
                Variant {
                    code: "legacy".to_string(),
                    name: "Russian (legacy)".to_string(),
                },
                Variant {
                    code: "typewriter-legacy".to_string(),
                    name: "Russian (typewriter, legacy)".to_string(),
                },
                Variant {
                    code: "dos".to_string(),
                    name: "Russian (DOS)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Russian (Macintosh)".to_string(),
                },
                Variant {
                    code: "ab".to_string(),
                    name: "Abkhazian (Russia)".to_string(),
                },
                Variant {
                    code: "bak".to_string(),
                    name: "Bashkirian".to_string(),
                },
                Variant {
                    code: "cv".to_string(),
                    name: "Chuvash".to_string(),
                },
                Variant {
                    code: "cv_latin".to_string(),
                    name: "Chuvash (Latin)".to_string(),
                },
                Variant {
                    code: "xal".to_string(),
                    name: "Kalmyk".to_string(),
                },
                Variant {
                    code: "kom".to_string(),
                    name: "Komi".to_string(),
                },
                Variant {
                    code: "chm".to_string(),
                    name: "Mari".to_string(),
                },
                Variant {
                    code: "os_legacy".to_string(),
                    name: "Ossetian (legacy)".to_string(),
                },
                Variant {
                    code: "os_winkeys".to_string(),
                    name: "Ossetian (Windows)".to_string(),
                },
                Variant {
                    code: "srp".to_string(),
                    name: "Serbian (Russia)".to_string(),
                },
                Variant {
                    code: "tt".to_string(),
                    name: "Tatar".to_string(),
                },
                Variant {
                    code: "udm".to_string(),
                    name: "Udmurt".to_string(),
                },
                Variant {
                    code: "sah".to_string(),
                    name: "Yakut".to_string(),
                },
            ],
        );
        
        variants.insert(
            "rs".to_string(),
            vec![
                Variant {
                    code: "alternatequotes".to_string(),
                    name: "Serbian (Cyrillic, with guillemets)".to_string(),
                },
                Variant {
                    code: "yz".to_string(),
                    name: "Serbian (Cyrillic, ZE and ZHE swapped)".to_string(),
                },
                Variant {
                    code: "latin".to_string(),
                    name: "Serbian (Latin)".to_string(),
                },
                Variant {
                    code: "latinalternatequotes".to_string(),
                    name: "Serbian (Latin, with guillemets)".to_string(),
                },
                Variant {
                    code: "latinunicode".to_string(),
                    name: "Serbian (Latin, Unicode)".to_string(),
                },
                Variant {
                    code: "latinyz".to_string(),
                    name: "Serbian (Latin, QWERTY)".to_string(),
                },
                Variant {
                    code: "latinunicodeyz".to_string(),
                    name: "Serbian (Latin, Unicode, QWERTY)".to_string(),
                },
                Variant {
                    code: "rue".to_string(),
                    name: "Pannonian Rusyn".to_string(),
                },
            ],
        );
        
        variants.insert(
            "lk".to_string(),
            vec![
                Variant {
                    code: "us".to_string(),
                    name: "Sinhala (US)".to_string(),
                },
                Variant {
                    code: "tam_unicode".to_string(),
                    name: "Tamil (Sri Lanka, TamilNet '99)".to_string(),
                },
                Variant {
                    code: "tam_TAB".to_string(),
                    name: "Tamil (Sri Lanka, TamilNet '99, TAB encoding)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "sk".to_string(),
            vec![
                Variant {
                    code: "bksl".to_string(),
                    name: "Slovak (extra backslash)".to_string(),
                },
                Variant {
                    code: "qwerty".to_string(),
                    name: "Slovak (QWERTY)".to_string(),
                },
                Variant {
                    code: "qwerty_bksl".to_string(),
                    name: "Slovak (QWERTY, extra backslash)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "si".to_string(),
            vec![
                Variant {
                    code: "alternatequotes".to_string(),
                    name: "Slovenian (with guillemets)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Slovenian (US)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "es".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Spanish (no dead keys)".to_string(),
                },
                Variant {
                    code: "deadtilde".to_string(),
                    name: "Spanish (dead tilde)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Spanish (Windows)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Spanish (Dvorak)".to_string(),
                },
                Variant {
                    code: "ast".to_string(),
                    name: "Asturian (Spain, with bottom-dot H and L)".to_string(),
                },
                Variant {
                    code: "cat".to_string(),
                    name: "Catalan (Spain, with middle-dot L)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "latam".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Spanish (Latin American, no dead keys)".to_string(),
                },
                Variant {
                    code: "deadtilde".to_string(),
                    name: "Spanish (Latin American, dead tilde)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Spanish (Latin American, Dvorak)".to_string(),
                },
                Variant {
                    code: "colemak".to_string(),
                    name: "Spanish (Latin American, Colemak)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ke".to_string(),
            vec![
                Variant {
                    code: "kik".to_string(),
                    name: "Kikuyu".to_string(),
                },
            ],
        );
        
        variants.insert(
            "se".to_string(),
            vec![
                Variant {
                    code: "nodeadkeys".to_string(),
                    name: "Swedish (no dead keys)".to_string(),
                },
                Variant {
                    code: "dvorak".to_string(),
                    name: "Swedish (Dvorak)".to_string(),
                },
                Variant {
                    code: "us_dvorak".to_string(),
                    name: "Swedish (Dvorak, intl.)".to_string(),
                },
                Variant {
                    code: "svdvorak".to_string(),
                    name: "Swedish (Svdvorak)".to_string(),
                },
                Variant {
                    code: "mac".to_string(),
                    name: "Swedish (Macintosh)".to_string(),
                },
                Variant {
                    code: "us".to_string(),
                    name: "Swedish (US)".to_string(),
                },
                Variant {
                    code: "swl".to_string(),
                    name: "Swedish Sign Language".to_string(),
                },
                Variant {
                    code: "smi".to_string(),
                    name: "Northern Saami (Sweden)".to_string(),
                },
                Variant {
                    code: "rus".to_string(),
                    name: "Russian (Sweden, phonetic)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "tw".to_string(),
            vec![
                Variant {
                    code: "indigenous".to_string(),
                    name: "Taiwanese (indigenous)".to_string(),
                },
                Variant {
                    code: "saisiyat".to_string(),
                    name: "Saisiyat (Taiwan)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "tj".to_string(),
            vec![
                Variant {
                    code: "legacy".to_string(),
                    name: "Tajik (legacy)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "th".to_string(),
            vec![
                Variant {
                    code: "tis".to_string(),
                    name: "Thai (TIS-820.2538)".to_string(),
                },
                Variant {
                    code: "pat".to_string(),
                    name: "Thai (Pattachote)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "tm".to_string(),
            vec![
                Variant {
                    code: "alt".to_string(),
                    name: "Turkmen (Alt-Q)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "tr".to_string(),
            vec![
                Variant {
                    code: "f".to_string(),
                    name: "Turkish (F)".to_string(),
                },
                Variant {
                    code: "e".to_string(),
                    name: "Turkish (E)".to_string(),
                },
                Variant {
                    code: "alt".to_string(),
                    name: "Turkish (Alt-Q)".to_string(),
                },
                Variant {
                    code: "intl".to_string(),
                    name: "Turkish (intl., with dead keys)".to_string(),
                },
                Variant {
                    code: "ku".to_string(),
                    name: "Kurdish (Turkey, Latin Q)".to_string(),
                },
                Variant {
                    code: "ku_f".to_string(),
                    name: "Kurdish (Turkey, F)".to_string(),
                },
                Variant {
                    code: "ku_alt".to_string(),
                    name: "Kurdish (Turkey, Latin Alt-Q)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "ua".to_string(),
            vec![
                Variant {
                    code: "phonetic".to_string(),
                    name: "Ukrainian (phonetic)".to_string(),
                },
                Variant {
                    code: "typewriter".to_string(),
                    name: "Ukrainian (typewriter)".to_string(),
                },
                Variant {
                    code: "winkeys".to_string(),
                    name: "Ukrainian (Windows)".to_string(),
                },
                Variant {
                    code: "macOS".to_string(),
                    name: "Ukrainian (macOS)".to_string(),
                },
                Variant {
                    code: "legacy".to_string(),
                    name: "Ukrainian (legacy)".to_string(),
                },
                Variant {
                    code: "homophonic".to_string(),
                    name: "Ukrainian (homophonic)".to_string(),
                },
                Variant {
                    code: "crh".to_string(),
                    name: "Crimean Tatar (Turkish Q)".to_string(),
                },
                Variant {
                    code: "crh_f".to_string(),
                    name: "Crimean Tatar (Turkish F)".to_string(),
                },
                Variant {
                    code: "crh_alt".to_string(),
                    name: "Crimean Tatar (Turkish Alt-Q)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "pk".to_string(),
            vec![
                Variant {
                    code: "urd-crulp".to_string(),
                    name: "Urdu (Pakistan, CRULP)".to_string(),
                },
                Variant {
                    code: "urd-nla".to_string(),
                    name: "Urdu (Pakistan, NLA)".to_string(),
                },
                Variant {
                    code: "ara".to_string(),
                    name: "Arabic (Pakistan)".to_string(),
                },
                Variant {
                    code: "snd".to_string(),
                    name: "Sindhi".to_string(),
                },
            ],
        );
        
        variants.insert(
            "uz".to_string(),
            vec![
                Variant {
                    code: "latin".to_string(),
                    name: "Uzbek (Latin)".to_string(),
                },
            ],
        );
        
        variants.insert(
            "vn".to_string(),
            vec![
                Variant {
                    code: "us".to_string(),
                    name: "Vietnamese (US)".to_string(),
                },
                Variant {
                    code: "tcvn".to_string(),
                    name: "Vietnamese (TCVN6064)".to_string(),
                },
            ],
        );
        
    
        let keyboard = vec![
            Keyboard {
                code: "al".to_string(),
                name: "Albanian".to_string(),
                variant: variants.get("al").unwrap().to_vec(),
            },
            Keyboard {
                code: "am".to_string(),
                name: "Armenian".to_string(),
                variant: variants.get("am").unwrap().to_vec(),
            },
            Keyboard {
                code: "ara".to_string(),
                name: "Arabic".to_string(),
                variant: variants.get("ara").unwrap().to_vec(),
            },
            Keyboard {
                code: "iq".to_string(),
                name: "Arabic (Iraq)".to_string(),
                variant: variants.get("iq").unwrap().to_vec(),
            },
            Keyboard {
                code: "ma".to_string(),
                name: "Arabic (Morocco)".to_string(),
                variant: variants.get("ma").unwrap().to_vec(),
            },
            Keyboard {
                code: "sy".to_string(),
                name: "Arabic (Syria)".to_string(),
                variant: variants.get("sy").unwrap().to_vec(),
            },
            Keyboard {
                code: "az".to_string(),
                name: "Azerbaijani".to_string(),
                variant: variants.get("az").unwrap().to_vec(),
            },
            Keyboard {
                code: "ml".to_string(),
                name: "Bambara".to_string(),
                variant: variants.get("ml").unwrap().to_vec(),
            },
            Keyboard {
                code: "bd".to_string(),
                name: "Bangla".to_string(),
                variant: variants.get("bd").unwrap().to_vec(),
            },
            Keyboard {
                code: "by".to_string(),
                name: "Belarusian".to_string(),
                variant: variants.get("by").unwrap().to_vec(),
            },
            Keyboard {
                code: "be".to_string(),
                name: "Belgian".to_string(),
                variant: variants.get("be").unwrap().to_vec(),
            },
            Keyboard {
                code: "dz".to_string(),
                name: "Berber (Algeria, Latin)".to_string(),
                variant: variants.get("dz").unwrap().to_vec(),
            },
            Keyboard {
                code: "ba".to_string(),
                name: "Bosnian".to_string(),
                variant: variants.get("ba").unwrap().to_vec(),
            },
            Keyboard {
                code: "brai".to_string(),
                name: "Braille".to_string(),
                variant: variants.get("brai").unwrap().to_vec(),
            },
            Keyboard {
                code: "bg".to_string(),
                name: "Bulgarian".to_string(),
                variant: variants.get("bg").unwrap().to_vec(),
            },
            Keyboard {
                code: "mm".to_string(),
                name: "Burmese".to_string(),
                variant: variants.get("mm").unwrap().to_vec(),
            },
            Keyboard {
                code: "cn".to_string(),
                name: "Chinese".to_string(),
                variant: variants.get("cn").unwrap().to_vec(),
            },
            Keyboard {
                code: "hr".to_string(),
                name: "Croatian".to_string(),
                variant: variants.get("hr").unwrap().to_vec(),
            },
            Keyboard {
                code: "cz".to_string(),
                name: "Czech".to_string(),
                variant: variants.get("cz").unwrap().to_vec(),
            },
            Keyboard {
                code: "dk".to_string(),
                name: "Danish".to_string(),
                variant: variants.get("dk").unwrap().to_vec(),
            },
            Keyboard {
                code: "us".to_string(),
                name: "English (US)".to_string(),
                variant: variants.get("us").unwrap().to_vec(),
            },
            Keyboard {
                code: "fi".to_string(),
                name: "Finnish".to_string(),
                variant: variants.get("fi").unwrap().to_vec(),
            },
            Keyboard {
                code: "fr".to_string(),
                name: "French".to_string(),
                variant: variants.get("fr").unwrap().to_vec(),
            },
            Keyboard {
                code: "ge".to_string(),
                name: "Georgian".to_string(),
                variant: variants.get("ge").unwrap().to_vec(),
            },
            Keyboard {
                code: "de".to_string(),
                name: "German".to_string(),
                variant: variants.get("de").unwrap().to_vec(),
            },
            Keyboard {
                code: "at".to_string(),
                name: "German (Austria)".to_string(),
                variant: variants.get("at").unwrap().to_vec(),
            },
            Keyboard {
                code: "ch".to_string(),
                name: "German (Switzerland)".to_string(),
                variant: variants.get("ch").unwrap().to_vec(),
            },
            Keyboard {
                code: "gr".to_string(),
                name: "Greek".to_string(),
                variant: variants.get("gr").unwrap().to_vec(),
            },
            Keyboard {
                code: "il".to_string(),
                name: "Hebrew".to_string(),
                variant: variants.get("il").unwrap().to_vec(),
            },
            Keyboard {
                code: "hu".to_string(),
                name: "Hungarian".to_string(),
                variant: variants.get("hu").unwrap().to_vec(),
            },
            Keyboard {
                code: "is".to_string(),
                name: "Icelandic".to_string(),
                variant: variants.get("is").unwrap().to_vec(),
            },
            Keyboard {
                code: "in".to_string(),
                name: "Indian".to_string(),
                variant: variants.get("in").unwrap().to_vec(),
            },
            Keyboard {
                code: "id".to_string(),
                name: "Indonesian (Latin)".to_string(),
                variant: variants.get("id").unwrap().to_vec(),
            },
            Keyboard {
                code: "ie".to_string(),
                name: "Irish".to_string(),
                variant: variants.get("ie").unwrap().to_vec(),
            },
            Keyboard {
                code: "it".to_string(),
                name: "Italian".to_string(),
                variant: variants.get("it").unwrap().to_vec(),
            },
            Keyboard {
                code: "jp".to_string(),
                name: "Japanese".to_string(),
                variant: variants.get("jp").unwrap().to_vec(),
            },
            Keyboard {
                code: "kz".to_string(),
                name: "Kazakh".to_string(),
                variant: variants.get("kz").unwrap().to_vec(),
            },
            Keyboard {
                code: "kr".to_string(),
                name: "Korean".to_string(),
                variant: variants.get("kr").unwrap().to_vec(),
            },
            Keyboard {
                code: "kg".to_string(),
                name: "Kyrgyz".to_string(),
                variant: variants.get("kg").unwrap().to_vec(),
            },
            Keyboard {
                code: "la".to_string(),
                name: "Lao".to_string(),
                variant: variants.get("la").unwrap().to_vec(),
            },
            Keyboard {
                code: "lv".to_string(),
                name: "Latvian".to_string(),
                variant: variants.get("lv").unwrap().to_vec(),
            },
            Keyboard {
                code: "lt".to_string(),
                name: "Lithuanian".to_string(),
                variant: variants.get("lt").unwrap().to_vec(),
            },
            Keyboard {
                code: "mk".to_string(),
                name: "Macedonian".to_string(),
                variant: variants.get("mk").unwrap().to_vec(),
            },
            Keyboard {
                code: "my".to_string(),
                name: "Malay (Jawi, Arabic Keyboard)".to_string(),
                variant: variants.get("my").unwrap().to_vec(),
            },
            Keyboard {
                code: "mt".to_string(),
                name: "Maltese".to_string(),
                variant: variants.get("mt").unwrap().to_vec(),
            },
            Keyboard {
                code: "md".to_string(),
                name: "Moldavian".to_string(),
                variant: variants.get("md").unwrap().to_vec(),
            },
            Keyboard {
                code: "me".to_string(),
                name: "Montenegrin".to_string(),
                variant: variants.get("me").unwrap().to_vec(),
            },
            Keyboard {
                code: "no".to_string(),
                name: "Norwegian".to_string(),
                variant: variants.get("no").unwrap().to_vec(),
            },
            Keyboard {
                code: "ir".to_string(),
                name: "Persian".to_string(),
                variant: variants.get("ir").unwrap().to_vec(),
            },
            Keyboard {
                code: "pl".to_string(),
                name: "Polish".to_string(),
                variant: variants.get("pl").unwrap().to_vec(),
            },
            Keyboard {
                code: "pt".to_string(),
                name: "Portuguese".to_string(),
                variant: variants.get("pt").unwrap().to_vec(),
            },
            Keyboard {
                code: "br".to_string(),
                name: "Brazilian Portuguese".to_string(),
                variant: variants.get("br").unwrap().to_vec(),
            },
            Keyboard {
                code: "ro".to_string(),
                name: "Romanian".to_string(),
                variant: variants.get("ro").unwrap().to_vec(),
            },
            Keyboard {
                code: "ru".to_string(),
                name: "Russian".to_string(),
                variant: variants.get("ru").unwrap().to_vec(),
            },
            Keyboard {
                code: "rs".to_string(),
                name: "Serbian".to_string(),
                variant: variants.get("rs").unwrap().to_vec(),
            },
            Keyboard {
                code: "lk".to_string(),
                name: "Sinhala".to_string(),
                variant: variants.get("lk").unwrap().to_vec(),
            },
            Keyboard {
                code: "sk".to_string(),
                name: "Slovak".to_string(),
                variant: variants.get("sk").unwrap().to_vec(),
            },
            Keyboard {
                code: "si".to_string(),
                name: "Slovenian".to_string(),
                variant: variants.get("si").unwrap().to_vec(),
            },
            Keyboard {
                code: "es".to_string(),
                name: "Spanish".to_string(),
                variant: variants.get("es").unwrap().to_vec(),
            },
            Keyboard {
                code: "latam".to_string(),
                name: "Latin American Spanish".to_string(),
                variant: variants.get("latam").unwrap().to_vec(),
            },
            Keyboard {
                code: "ke".to_string(),
                name: "Kikuyu".to_string(),
                variant: variants.get("ke").unwrap().to_vec(),
            },
            Keyboard {
                code: "se".to_string(),
                name: "Swedish".to_string(),
                variant: variants.get("se").unwrap().to_vec(),
            },
            Keyboard {
                code: "tw".to_string(),
                name: "Taiwanese".to_string(),
                variant: variants.get("tw").unwrap().to_vec(),
            },
            Keyboard {
                code: "tj".to_string(),
                name: "Tajik".to_string(),
                variant: variants.get("tj").unwrap().to_vec(),
            },
            Keyboard {
                code: "th".to_string(),
                name: "Thai".to_string(),
                variant: variants.get("th").unwrap().to_vec(),
            },
            Keyboard {
                code: "tm".to_string(),
                name: "Turkmen".to_string(),
                variant: variants.get("tm").unwrap().to_vec(),
            },
            Keyboard {
                code: "tr".to_string(),
                name: "Turkish".to_string(),
                variant: variants.get("tr").unwrap().to_vec(),
            },
            Keyboard {
                code: "ua".to_string(),
                name: "Ukrainian".to_string(),
                variant: variants.get("ua").unwrap().to_vec(),
            },
            Keyboard {
                code: "pk".to_string(),
                name: "Urdu".to_string(),
                variant: variants.get("pk").unwrap().to_vec(),
            },
            Keyboard {
                code: "uz".to_string(),
                name: "Uzbek".to_string(),
                variant: variants.get("uz").unwrap().to_vec(),
            },
            Keyboard {
                code: "vn".to_string(),
                name: "Vietnamese".to_string(),
                variant: variants.get("vn").unwrap().to_vec(),
            },            
        ];

        keyboard
    }
}
