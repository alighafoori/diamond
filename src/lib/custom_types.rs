use std::collections::HashMap;

pub type CDateT = String;
pub type VString = Vec<String>;
pub type VVString = Vec<Vec<String>>;


/*

pub type CCoinCodeT = String;
pub type CMPAIValueT = u64;
// (+) micro PAI is the smallest unit of accounting for system coins, but normally we use PAI
pub type CMPAISValueT = i64;
// (+-)micro PAI is the smallest unit of accounting for system coins, but normally we use PAI
pub type CBlockHashT = String;
pub type CDocHashT = String;
pub type CAddressT = String;
pub type CDocIndexT = u32;

// customizing document index maximum number
pub type CInputIndexT = u16;

pub type CSigIndexT = u16;
pub type COutputIndexT = u16; // customizing document index maximum number

pub type DPIIndexT = u16;
pub type DNASharePercentT = f64;
pub type DNAShareCountT = f64;

pub type BlockAncestorsCountT = u16;
// TODO: add max ancestor count control for received blocks
pub type BlockLenT = u32;
pub type DocLenT = u32;

pub type CVoteT = i16;  // between -100 0 100

pub type TimeByHoursT = f64;
// time by hours
pub type TimeByMinutesT = u64;
 */
pub type TimeBySecT = u64;

/*
use std::collections::HashMap;

pub type doubleDicT = HashMap<String, f64>      ; // custom dictionary
pub type floatDicT = HashMap<String, f64>       floatDicT; // custom dictionary
pub type UI16DicT = HashMap<String, CDocIndexT>  UI16DicT; // custom dictionary
pub type QHash<QString, uint32_t>    UI32DicT; // custom dictionary
pub type QHash<QString, uint64_t>    UI64DicT; // custom dictionary
*/
pub type QSDicT = HashMap<String, String>;
// custom dictionary
/*
pub type QHash<QString, QStringList> QSLDicT; // custom dictionary
pub type QHash<QString, QSDicT>      QS2DicT; // custom dictionary
*/
pub type QVariant = String    ; // FIXME: implement different QVariant (something like union)!
pub type QVDicT = HashMap<String, QVariant>    ; // custom dictionary
/*
pub type QHash<QString, QJsonObject> QJODicT; // custom dictionary
pub type QHash<QString, QJsonArray>  QJADicT; // custom dictionary
pub type QHash<QString, QVDicT>      QV2DicT;
*/
pub type QVDRecordsT = Vec<QVDicT>;
/*
pub type QVector<QSDicT>        QSDRecordsT;
pub type QVector<QV2DicT>       QV2DRecordsT;
pub type QVector<QJsonObject>   JORecordsT;
pub type QVector<QJsonArray>    JARecordsT;
pub type QVector<ModelClause>   ClausesT;
pub type QVector<OrderModifier> OrderT;

pub type QHash<QString, QVDRecordsT> GRecordsT; // Groupped records
pub type QHash<QString, GRecordsT> G2RecordsT; // Groupped Groupped records


class Coin;
pub type QList<QStringList> CListListT;
pub type QVector<Coin> CoinsT;

 */