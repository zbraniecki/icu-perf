// @generated
# ! [allow (clippy :: octal_escapes)] type DataStruct = < :: icu::plurals :: provider :: CardinalV1Marker as :: icu_provider :: DataMarker > :: Yokeable ; pub fn lookup (locale : & icu_provider :: DataLocale) -> Option < & 'static DataStruct > { static KEYS : [& str ; 89usize] = ["af" , "am" , "ar" , "as" , "az" , "be" , "bg" , "bn" , "bs" , "ca" , "cs" , "cy" , "da" , "de" , "el" , "en" , "es" , "et" , "eu" , "fa" , "fi" , "fil" , "fr" , "ga" , "gd" , "gl" , "gu" , "ha" , "he" , "hi" , "hr" , "hu" , "hy" , "id" , "ig" , "is" , "it" , "ja" , "jv" , "ka" , "kk" , "km" , "kn" , "ko" , "ky" , "lo" , "lt" , "lv" , "mk" , "ml" , "mn" , "mr" , "ms" , "my" , "ne" , "nl" , "nn" , "no" , "or" , "pa" , "pcm" , "pl" , "ps" , "pt" , "ro" , "ru" , "sd" , "si" , "sk" , "sl" , "so" , "sq" , "sr" , "sv" , "sw" , "ta" , "te" , "th" , "tk" , "tr" , "uk" , "und" , "ur" , "uz" , "vi" , "yo" , "yue" , "zh" , "zu"] ; static DATA : [& DataStruct ; 89usize] = [& AF , & AM , & AR , & AM , & AF , & BE , & AF , & AM , & BS , & CA , & CS , & CY , & DA , & DE , & AF , & DE , & ES , & DE , & AF , & AM , & DE , & FIL , & FR , & GA , & GD , & DE , & AM , & AF , & HE , & AM , & BS , & AF , & HY , & ID , & ID , & IS , & CA , & ID , & ID , & AF , & AF , & ID , & AM , & ID , & AF , & ID , & LT , & LV , & MK , & AF , & AF , & AF , & ID , & ID , & AF , & DE , & AF , & AF , & AF , & PA , & AM , & PL , & AF , & PT , & RO , & RU , & AF , & SI , & CS , & SL , & AF , & AF , & BS , & DE , & DE , & AF , & AF , & ID , & AF , & AF , & RU , & ID , & DE , & AF , & ID , & ID , & ID , & ID , & AM] ; KEYS . binary_search_by (| k | locale . strict_cmp (k . as_bytes ()) . reverse ()) . ok () . map (| i | unsafe { * DATA . get_unchecked (i) }) } static AF : DataStruct = include ! ("af.rs.data") ; static AM : DataStruct = include ! ("am.rs.data") ; static AR : DataStruct = include ! ("ar.rs.data") ; static BE : DataStruct = include ! ("be.rs.data") ; static BS : DataStruct = include ! ("bs.rs.data") ; static CA : DataStruct = include ! ("ca.rs.data") ; static CS : DataStruct = include ! ("cs.rs.data") ; static CY : DataStruct = include ! ("cy.rs.data") ; static DA : DataStruct = include ! ("da.rs.data") ; static DE : DataStruct = include ! ("de.rs.data") ; static ES : DataStruct = include ! ("es.rs.data") ; static FIL : DataStruct = include ! ("fil.rs.data") ; static FR : DataStruct = include ! ("fr.rs.data") ; static GA : DataStruct = include ! ("ga.rs.data") ; static GD : DataStruct = include ! ("gd.rs.data") ; static HE : DataStruct = include ! ("he.rs.data") ; static HY : DataStruct = include ! ("hy.rs.data") ; static ID : DataStruct = include ! ("id.rs.data") ; static IS : DataStruct = include ! ("is.rs.data") ; static LT : DataStruct = include ! ("lt.rs.data") ; static LV : DataStruct = include ! ("lv.rs.data") ; static MK : DataStruct = include ! ("mk.rs.data") ; static PA : DataStruct = include ! ("pa.rs.data") ; static PL : DataStruct = include ! ("pl.rs.data") ; static PT : DataStruct = include ! ("pt.rs.data") ; static RO : DataStruct = include ! ("ro.rs.data") ; static RU : DataStruct = include ! ("ru.rs.data") ; static SI : DataStruct = include ! ("si.rs.data") ; static SL : DataStruct = include ! ("sl.rs.data") ;