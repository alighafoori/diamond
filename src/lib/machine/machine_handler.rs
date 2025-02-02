// use std::sync::{Arc, Mutex};

use std::collections::HashMap;
use crate::cutils;
use crate::lib::constants::LAUNCH_DATE;
// use crate::lib::constants as cconsts;
use crate::lib::custom_types::{CDateT, QSDicT, TimeBySecT, VString};

//  '  '  '  '  '  '  '  '  '  '  '  '  '  '  '  machine_handler.cpp file
#[derive(Default)]
pub struct CMachine {
    m_clone_id: i8,
    m_should_loop_threads: bool,

    m_is_in_sync_process: bool,
    m_last_sync_status_check: CDateT,

    m_threads_status: QSDicT,
    m_map_thread_code_to_prefix: QSDicT,
    m_is_develop_mod: bool,
    /*
    CDateT m_develop_launch_date = "";

    CMachine(const CMachine&) = delete;

  const static QString stbl_machine_profiles;
  const static QString stbl_machine_neighbors;

  const static QString stb_machine_block_buffer;
  const static QStringList stb_machine_block_buffer_fields;

  static const QString stbl_machine_onchain_contracts;
  static const QStringList stbl_machine_onchain_contracts_fields;

  Config* m_global_configs {};
  */
    pub(crate) m_recorded_blocks_in_db: u32, // TODO: remove this variable(mechanism) after fixing sqlite database lock problem
    /*
      QStringList m_cache_coins_visibility = {}; // TODO: remove this variable(mechanism) after fixing sqlite database lock problem and bloom filter implementation
    QVDRecordsT m_cache_spendable_coins = {}; // TODO: remove this variable(mechanism) after fixing sqlite database lock problem
    QVDRecordsT m_DAG_cached_blocks; // TODO: optimize it ASAP
    QStringList m_DAG_cached_block_hashes = {}; // TODO: optimize it ASAP

    MachineProfile m_profile;

      */
}
/*
pub trait CMachineThreadGaps {
    fn get_coinbase_import_gap(&self) -> TimeBySecT;
    fn get_block_invoke_gap(&self) -> TimeBySecT;
    fn get_nb_coins_import_gap(&mut self) -> TimeBySecT;
}
 */


impl CMachine {
    pub(crate) fn new() -> CMachine {
        CMachine {
            m_clone_id: 0,
            m_should_loop_threads: true,

            m_is_in_sync_process: true,
            m_last_sync_status_check: LAUNCH_DATE.to_string(),

            m_is_develop_mod: false,

            m_threads_status: HashMap::new(),
            m_map_thread_code_to_prefix: HashMap::new(),

            /*
            uint8_t m_clone_id = 0;
            bool m_is_develop_mod = false;
            CDateT m_develop_launch_date = "";

            CMachine(const CMachine&) = delete;

          const static QString stbl_machine_profiles;
          const static QString stbl_machine_neighbors;

          const static QString stb_machine_block_buffer;
          const static QStringList stb_machine_block_buffer_fields;

          static const QString stbl_machine_onchain_contracts;
          static const QStringList stbl_machine_onchain_contracts_fields;

          Config* m_global_configs {};
          */
            m_recorded_blocks_in_db: 0, // TODO: remove this variable(mechanism) after fixing sqlite database lock problem
            /*
              QStringList m_cache_coins_visibility = {}; // TODO: remove this variable(mechanism) after fixing sqlite database lock problem and bloom filter implementation
            QVDRecordsT m_cache_spendable_coins = {}; // TODO: remove this variable(mechanism) after fixing sqlite database lock problem
            QVDRecordsT m_DAG_cached_blocks; // TODO: optimize it ASAP
            QStringList m_DAG_cached_block_hashes = {}; // TODO: optimize it ASAP

            MachineProfile m_profile;

              */
        }
    }

    pub fn init(&self) -> bool
    {
        // let mut m_should_loop_threads = singleton.m_should_loop_threads.try_lock().unwrap();
        // *m_should_loop_threads = true;
        // println!("singleton init m_should_loop_threads: {}", m_should_loop_threads);
        // // CMachine::get_instance().m_is_develop_mod = Mutex::from(is_develop_mod);
        // println!("::::::::::init>>>>>>>>>>>>>> {:?}",CMachine::get_instance().m_should_loop_threads);
        //

        true
    }

    // func name was parseArgs
    pub fn parse_args(&self, args: VString, manual_clone_id: i8)
    {
        // println!("Env args: {:?}", args);

        let mut clone_id: i8 = 0;    // FIXME: this value must be defined by command line
        let mut is_develop_mod: bool = false;


        if args.len() > 1 {
            clone_id = args[1].parse().unwrap();
        }

        if manual_clone_id > 0 {
            clone_id = manual_clone_id;
        }

        if args.len() > 2 {
            is_develop_mod = true;
        }

        self.set_clone_dev(clone_id, is_develop_mod);
    }

    // func name was setCloneDev
    pub fn set_clone_dev(&self, clone_id: i8, _is_develop_mod: bool) -> bool
    {
        // let singleton = CMachine::get_instance();
        // let mut m_clone_id = singleton.m_clone_id.try_lock().unwrap();
        // *m_clone_id = clone_id;
        // println!("singleton init m_clone_id: {}", m_clone_id);
        // println!("::::::::::clone dev >>>>>>>>>>>>>> {:?}",CMachine::get_instance().m_should_loop_threads.try_lock());

        // CMachine::get_instance().m_is_develop_mod = Mutex::from(is_develop_mod);
        true
    }


    //func name was shouldLoopThreads
    pub fn should_loop_threads(&self) -> bool {
        println!("should_ loop_ threads > {:?}", self.m_should_loop_threads);
        self.m_should_loop_threads
    }


    /**
     * if the creationdate of latest recorded block in DAG is older than 2 cycle, so machine must go to synching mode
     * @param {*} args
     */
    //old_name_was isInSyncProcess
    pub fn is_in_sync_process(&self, _force_to_control_based_on_DAG_status: bool) -> bool
    {
        //  put LAST_SYNC_STATUS in CMachine as a static member
        if !self.m_is_in_sync_process {
            return false;
        }

        if cutils::time_diff(self.m_last_sync_status_check.clone(), "".to_string()).as_minutes < 2 {
            return self.m_is_in_sync_process;
        }

        true

        /*
        QString lastSyncStatus = KVHandler::getValue("LAST_SYNC_STATUS");
        if (lastSyncStatus == "")
        {
            IinitLastSyncStatus();
            QString
            lastSyncStatus = KVHandler::getValue("LAST_SYNC_STATUS");
        }
        QJsonObject
        lastSyncStatusObj = CUtils::parseToJsonObj(lastSyncStatus);

        uint64_t
        cycleByMinutes = CMachine::getCycleByMinutes();
        // control if the last status-check is still valid (is younger than 30 minutes?= 24 times in a cycle)
        if (!force_to_control_based_on_DAG_status & &
            (lastSyncStatusObj.value("checkDate").toString() > CUtils::minutesBefore((cycleByMinutes / 24))))
        {
            bool
            is_in_sync = lastSyncStatusObj.value("lastDAGBlockCreationDate").toString() < CUtils::minutesBefore(2 * cycleByMinutes);
            setIsInSyncProcess(is_in_sync, CUtils::getNow());
            return is_in_sync;
        } else {
            // re-check graph info&  update status-check info too
            auto
            [status, blockRecord] = DAG::getLatestBlockRecord();
            if (status)
            CUtils::exiter("No block in DAG exit!!", 111);

            bool
            is_in_sync_process = (blockRecord.m_creation_date < CUtils::minutesBefore(2 * cycleByMinutes));

            lastSyncStatusObj.insert("isInSyncMode", is_in_sync_process? "Y": "N");
            lastSyncStatusObj.insert("checkDate", CUtils::getNow());
            lastSyncStatusObj.insert("lastDAGBlockCreationDate", blockRecord.m_creation_date);
            if (is_in_sync_process)
            lastSyncStatusObj.insert("lastTimeMachineWasInSyncMode", CUtils::getNow());
            KVHandler::upsertKValue("LAST_SYNC_STATUS", CUtils::serializeJson(lastSyncStatusObj));
            setIsInSyncProcess(is_in_sync_process, CUtils::getNow());
            return is_in_sync_process;
        }
        */
    }

    //old_name_was setIsInSyncProcess
    pub fn set_is_in_sync_process(&mut self, status: bool, cDate: &CDateT)
    {
        self.m_is_in_sync_process = status;
        self.m_last_sync_status_check = cDate.clone();
    }

    //old_name_was reportThreadStatus
    pub fn report_thread_status(&mut self, thread_prefix: &String, thread_code: &String, thread_status: &String)
    {
        self.m_threads_status.insert((thread_prefix.to_string() + &thread_code).clone(), thread_status.clone());
        self.m_map_thread_code_to_prefix.insert(thread_code.clone(), thread_prefix.clone());
    }
}


//
// trait Booting{
//     fn parse_args(self, args: Vec<String>, manual_clone_id: i8);
//     fn set_clone_dev(self, clone_id: i8, is_develop_mod: bool) -> bool;
// }







/*


#ifndef CMACHINE_H
#define CMACHINE_H

class MainWindow;
class CUtils;
class TransientBlockInfo;


#include "lib/transactions/basic_transactions/signature_structure_handler/individual_signature.h"
#include "lib/transactions/basic_transactions/signature_structure_handler/unlock_set.h"
#include "lib/transactions/basic_transactions/signature_structure_handler/unlock_document.h"
#include "lib/transactions/basic_transactions/signature_structure_handler/general_structure.h"

#include "lib/address/address_handler.h"
#include "lib/wallet/wallet.h"
#include "lib/k_v_handler.h"
#include "lib/dag/dag.h"


class MachineTransientBalances
{
public:
  CMPAIValueT m_one_cycle_issued = 0;
  uint64_t m_cycle_counts_from_began = 0;
  uint64_t m_distinct_coinbases_count = 0;
  CMPAIValueT m_total_minted_coins = 0;
  CMPAIValueT m_total_spendable_coins = 0;

  QVDRecordsT m_wallet_spendable_UTXOs {};

  CMPAIValueT m_cb_not_imported_sum = 0;
  QVDRecordsT m_cb_not_imported_processed_outputs = {};
  CMPAIValueT m_cb_not_imported_coinbase_value = 0;

  CMPAIValueT m_normal_not_imported_sum = 0;
  CMPAIValueT m_waited_treasury_incomes = 0;

  CMPAIValueT m_cb_floorished_coins = 0;

  CMPAISValueT m_final_balance = 0;
  CMPAIValueT m_wallet_wealth_value = 0;

  QVDRecordsT m_coins_existance = {};

};


class EmailSettings
{
public:
  QString m_address = "abc@def.gh";
  QString m_password = "";
  QString m_income_IMAP = "993";
  QString m_income_POP3 = "995";
  QString m_incoming_mail_server = "";
  QString m_outgoing_mail_server = "";
  QString m_outgoing_SMTP = "465";
  QString m_fetching_interval_by_minute = "5";  // it depends on smtp server, but less than 5 minute is useless
  QString m_PGP_private_key = "";
  QString m_PGP_public_key = "";

  EmailSettings()
  {}
  void importJson(const QJsonObject& obj);
  QJsonObject exportJson() const;
};

class MPSetting
{
public:
  MPSetting();

  EmailSettings m_public_email;
  EmailSettings m_private_email;

  QString m_machine_alias = "imagineNode";
  UnlockDocument* m_backer_detail{};
  QString m_language = CConsts::DEFAULT_LANG;
  QString m_term_of_services = CConsts::NO;
  QJsonArray m_already_presented_neighbors = {};

  void importJson(const QJsonObject& obj);
  QJsonObject exportJson() const;

};

class MachineProfile
{
public:
  QString m_mp_code = "";
  QString m_mp_name = "";
  QString m_mp_last_modified = "";
  MPSetting m_mp_settings;
  QJsonObject m_this_json;

  // provides a new template profile
  MachineProfile();
  // retrieve profile from DB and sets to object
  MachineProfile(const QString& mp_code);
  // set passed JSon object to itself
  MachineProfile(const QJsonObject& profile);

  void importJson(const QJsonObject& profile);
  QJsonObject exportJson() const;

};

struct CoinsVisibilityRes {
  bool status = false;
  QStringList records = {};
  bool is_visible = false;
};


//  static MainWindow* getMW(){ return get().m_mw; };


  static CMachine& get(){return s_instance;};

  static void parseArgs(int argc, char *argv[], int manual_clone_id = 0);
  static void onAboutToQuit(MainWindow* w){ get().IonAboutToQuit(w); };

  static QString getLaunchDate(){return get().IgetLaunchDate(); };
  static void setLaunchDateAndCloneId(const CDateT& cDate, uint8_t clone_id = 0){return get().IsetLaunchDateAndCloneId(cDate, clone_id); };

  static void emptyDB();

  // machine_handler.cpp
  GenRes initDefaultProfile();
  static bool bootMachine(){return get().IbootMachine();}

  static void setDAGIsInitialized(bool status){get().s_DAG_is_initialized = status; }
  static bool getDAGIsInitialized(){ return get().s_DAG_is_initialized; }

  static std::tuple<bool, QVDRecordsT> cachedSpendableCoins(
    const QString& action = "read",
    const QVDRecordsT& coins = {},
    const CBlockHashT& visible_by = "",
    const CCoinCodeT& the_coin = "") { return get().IcachedSpendableCoins(action, coins, visible_by, the_coin); };

  static std::tuple<bool, QVDRecordsT&> cachedBlocks(
    const QString& action = "read",
    const QVDRecordsT& blocks = {},
    const QString& status = "") { return get().IcachedBlocks(action, blocks, status); };

  static std::tuple<bool, QStringList&> cachedBlockHashes(
    const QString& action = "read",
    const QStringList& block_hashes = {}) { return get().IcachedBlockHashes(action, block_hashes); };

  static CoinsVisibilityRes cachedCoinsVisibility(
    const QString& action = "read",
    const QStringList& entries = {}){ return get().IcachedCoinsVisibility(action, entries); };

  static bool setCloneDev(const uint8_t clone_id, const bool is_develop_mod){return get().IsetCloneDev(clone_id, is_develop_mod);}
  static bool isDevelopMod(){ return get().IisDevelopMod();}
  static uint8_t getAppCloneId(){return get().IgetAppCloneId();}
  static QString getAppCloneIdStr(){return (get().IgetAppCloneId()>0) ? QString::number(get().IgetAppCloneId()) : "";}
  static bool createFolders(){return get().IcreateFolders();}
  static QString getHDPath(){return get().IgetHDPath();}
  static QString getReportsPath(){return get().IgetReportsPath();}
  static QString getInboxPath(){return get().IgetInboxPath();}
  static QString getOutboxPath(){return get().IgetOutboxPath();}
  static QString getLogsPath(){return get().IgetLogsPath();}
  static QString getDAGBackup(){return get().IgetDAGBackup();}

  static bool shouldLoopThreads(){return get().IshouldLoopThreads();}
  static void setShouldLoopThreads(const bool v){return get().IsetShouldLoopThreads(v); }

  static bool databasesAreCreated(){return get().IdatabasesAreCreated();}
  static void setDatabasesAreCreated(const bool v){return get().IsetDatabasesAreCreated(v); }

  static bool canStartLazyLoadings(){return get().IcanStartLazyLoadings();}
  static void setCanStartLazyLoadings(bool v){ get().IsetCanStartLazyLoadings(v);}
  static QString mapThreadCodeToPrefix(const QString& code){ return get().ImapThreadCodeToPrefix(code);}


  static QString getBackerAddress(){return get().IgetBackerAddress();}
  static UnlockDocument* getBackerDetails(){return get().IgetBackerDetails();}
  static MachineProfile getProfile(){return get().IgetProfile();}
  static EmailSettings getPubEmailInfo(){return get().IgetPubEmailInfo();}
  static EmailSettings getPrivEmailInfo(){return get().IgetPrivEmailInfo();}
  static bool setPublicEmailAddress(const QString&  v){return get().IsetPublicEmailAddress(v);}
  static bool setPublicEmailInterval(const QString&  v){return get().IsetPublicEmailInterval(v);}
  static bool setPublicEmailIncomeHost(const QString&  v){return get().IsetPublicEmailIncomeHost(v);}
  static bool setPublicEmailPassword(const QString&  v){return get().IsetPublicEmailPassword(v);}
  static bool setPublicEmailIncomeIMAP(const QString&  v){return get().IsetPublicEmailIncomeIMAP(v);}
  static bool setPublicEmailIncomePOP(const QString&  v){return get().IsetPublicEmailIncomePOP(v);}
  static bool setPublicEmailOutgoingSMTP(const QString&  v){return get().IsetPublicEmailOutgoingSMTP(v);}
  static bool setPublicEmailOutgoingHost(const QString&  v){return get().IsetPublicEmailOutgoingHost(v);}
  static bool setTermOfServices(const QString&  v){return get().IsetTermOfServices(v);}
  static bool saveSettings(){return get().IsaveSettings();}
  static QJsonObject getLastSyncStatus(){return get().IgetLastSyncStatus();};
  static QString getSelectedMProfile(){ return get().IgetSelectedMProfile(); };

  static bool isGUIConnected(){ return get().IisGUIConnected(); }
  static void setIsGUIConnected(const bool status){ get().IsetIsGUIConnected(status); }

  bool loadSelectedProfile();

  static std::tuple<bool, QString, UnlockSet, QStringList> signByMachineKey(
    const QString& signMsg,
    const CSigIndexT& unlockIndex = 0);

  static uint64_t getCycleByMinutes();
  static TimeBySecT getPopEmailGap();
  static TimeBySecT getParsingQGap();
  static TimeBySecT getSendingQGap();
  static TimeBySecT getSendEmailGap();
  static TimeBySecT getInvokeLeavesGap();
  static TimeBySecT getCoinbaseImportGap();
  static TimeBySecT getHardDiskReadingGap();
  static TimeBySecT getAcceptableBlocksGap();
  static TimeBySecT getINamesSettlementGap();
  static TimeBySecT getConcludeTreatmentGap();
  static TimeBySecT getPrerequisitiesRemoverGap();

  static double getMinPollingTimeframeByHour();
  static TimeByHoursT getMinVotingTimeframe();

  static QString getCachePath(QString appCloneId = "");

  static double getMachineServiceInterests(
    const QString& dType,
    const QString& dClass = "",
    const DocLenT& dLen = 0,
    const DocLenT& extra_length = 0,
    const int& supported_P4P_trx_count = 1)
  {
      return get().IgetMachineServiceInterests(
        dType,
        dClass,
        dLen,
        extra_length,
        supported_P4P_trx_count);
  }


  //  -  -  -  -  -  machine_backup.cpp


  //  -  -  -  -  -  machine_services_interests.cpp


  //  -  -  -  -  -  neighbors handler

  static QVDRecordsT getNeighbors(
    const QString& neighbor_type = "",
    const QString& connection_status = "",
    const QString& mp_code = getSelectedMProfile(),
    const QString& n_id = "",
    const QString& n_email = ""){ return get().IgetNeighbors(neighbor_type, connection_status, mp_code, n_id, n_email); };

  static QVDRecordsT getActiveNeighbors(const QString& mp_code = getSelectedMProfile());

  static std::tuple<bool, QString> addANewNeighbor(
    const QString& email,
    const QString& connection_type,
    const QString& public_key = "",
    const QString& mp_code = getSelectedMProfile(),
    const QString& is_active = CConsts::YES,
    const QJsonObject& neighbor_info = QJsonObject(),
    QString creation_date = "");

  static bool handshakeNeighbor(const QString& n_id, const QString& connection_type);

  static std::tuple<bool, bool> parseHandshake(
    const QString& sender_email,
    const QJsonObject& message,
    const QString& connection_type);

  static bool floodEmailToNeighbors(
    const QString& email,
    QString PGP_public_key = ""){ return get().IfloodEmailToNeighbors(email, PGP_public_key); };

  static bool setAlreadyPresentedNeighbors(const QJsonArray& already_presented_neighbors){ return get().IsetAlreadyPresentedNeighbors(already_presented_neighbors); }

  static QJsonArray getAlreadyPresentedNeighbors(){ return get().IgetAlreadyPresentedNeighbors(); }

  static bool deleteNeighbors(
    const QString& n_id,
    const QString& connection_type,
    const QString& mp_code = getSelectedMProfile()){return get().IdeleteNeighbors(n_id, connection_type, mp_code);}

  static std::tuple<bool, bool> parseNiceToMeetYou(
    const QString& sender_email,
    const QJsonObject& message,
    const QString& connection_type);

  //  -  -  -  accounts balances
  static MachineTransientBalances machineBalanceChk();


  //  -  -  -  block buffer part
  static QVDRecordsT searchBufferedDocs(
    const ClausesT& clauses = {},
    const QStringList& fields = stb_machine_block_buffer_fields,
    const OrderT& order = {},
    const uint64_t limit = 0);

  static std::tuple<bool, QString> pushToBlockBuffer(
    const Document* doc,
    const CMPAIValueT dp_cost,
    const QString& mp_code = getSelectedMProfile());

  static std::tuple<bool, QString> broadcastBlock(
    const QString& cost_pay_mode = "normal",
    const QString& create_date_type = "");

  static std::tuple<bool, bool, QString> fetchBufferedTransactions(
    Block* block,
    TransientBlockInfo& transient_block_info);

  static std::tuple<bool, bool, QString> retrieveAndGroupBufferedDocuments(
    Block* block,
    TransientBlockInfo& transient_block_info);

  static bool removeFromBuffer(const ClausesT& clauses);


  //  -  -  -  on-chain contracts part
  static QJsonArray searchInMyOnchainContracts(
    const ClausesT& clauses,
    const QStringList& fields = stbl_machine_onchain_contracts_fields,
    const OrderT order = {},
    const uint64_t limit = 0);





  bool m_machine_is_loaded = false;
  bool m_is_GUI_connected = false;
  QString m_selected_profile = "";



  bool m_db_is_connected = false;
  bool s_DAG_is_initialized;
  bool m_should_loop_threads = true;
  bool m_databases_are_created = false;
  bool m_can_start_lazy_loadings = false;

  bool IbootMachine();
  void IsetLaunchDateAndCloneId(const CDateT& cDate, uint8_t clone_id = 0);
  QString IgetLaunchDate();
  QString IgetBackerAddress();
  UnlockDocument* IgetBackerDetails();
  bool IsetPublicEmailAddress(const QString&  v);
  bool IsetPublicEmailInterval(const QString&  v);
  bool IsetPublicEmailIncomeHost(const QString&  v);
  bool IsetPublicEmailPassword(const QString&  v);
  bool IsetPublicEmailIncomeIMAP(const QString&  v);
  bool IsetPublicEmailIncomePOP(const QString&  v);
  bool IsetPublicEmailOutgoingSMTP(const QString&  v);
  bool IsetPublicEmailOutgoingHost(const QString&  v);
  bool IsetTermOfServices(const QString&  v);
  bool IsaveSettings();
  bool ImaybeAddSeedNeighbors();

  QString IgetSelectedMProfile();

  QJsonObject IgetLastSyncStatus();
  bool IinitLastSyncStatus();


  void IsetIsGUIConnected(const bool status)
  {
    m_is_GUI_connected = status;
  }

  bool IisGUIConnected()
  {
    return m_is_GUI_connected;
  }

  static double IgetMachineServiceInterests(
    const QString& dType,
    const QString& dClass = "",
    const DocLenT& dLen = 0,
    const DocLenT& extra_length = 0,
    const int& supported_P4P_trx_count = 1);

  uint8_t IgetAppCloneId()
  {
    return m_clone_id;
  }

  void IsetShouldLoopThreads(const bool v)
  {
    m_should_loop_threads = v;
  }

  bool IshouldLoopThreads()
  {
    return m_should_loop_threads;
  }

  void IsetDatabasesAreCreated(const bool v)
  {
    m_databases_are_created = v;
  }

  bool IdatabasesAreCreated()
  {
    return m_databases_are_created;
  }

  bool IcanStartLazyLoadings()
  {
    return m_can_start_lazy_loadings;
  }

  void IsetCanStartLazyLoadings(bool v)
  {
    m_can_start_lazy_loadings = v;
  }

  bool IisDevelopMod()
  {
    return m_is_develop_mod;
  }

  bool IsetCloneDev(const uint8_t clone_id, const bool is_develop_mod) //, MainWindow* mw
  {
//    m_mw = mw;  //MainWindow* mw
    m_clone_id = clone_id;
    m_is_develop_mod = is_develop_mod;
    return true;
  }

  bool IcreateFolders();
  QString IgetReportsPath();
  QString IgetHDPath();
  QString IgetInboxPath();
  QString IgetOutboxPath();
  QString IgetLogsPath();
  QString IgetDAGBackup();

  MachineProfile IgetProfile()
  {
    return m_profile;
  }

  EmailSettings IgetPubEmailInfo(){return m_profile.m_mp_settings.m_public_email;}
  EmailSettings IgetPrivEmailInfo(){return m_profile.m_mp_settings.m_private_email;}

  std::tuple<bool, QVDRecordsT&> IcachedBlocks(
    const QString& action = "read",
    const QVDRecordsT& blocks = {},
    const QString& status = "");

  std::tuple<bool, QStringList&> IcachedBlockHashes(
    const QString& action = "read",
    const QStringList& block_hashes = {});

  std::tuple<bool, QVDRecordsT> IcachedSpendableCoins(
    const QString& action,
    const QVDRecordsT& coins = {},
    const CBlockHashT& visible_by = "",
    const CCoinCodeT& the_coin = "");

  CoinsVisibilityRes IcachedCoinsVisibility(
    const QString& action,
    const QStringList& entries);

  //  -  -  -  -  -  neighbors handler
  QVDRecordsT IgetNeighbors(
    const QString& neighbor_type = "",
    const QString& connection_status = "",
    const QString& mp_code = getSelectedMProfile(),
    const QString& n_id = "",
    const QString& n_email = "");

  bool IfloodEmailToNeighbors(
    const QString& email,
    QString PGP_public_key = "");

  QJsonArray IgetAlreadyPresentedNeighbors(){ return m_profile.m_mp_settings.m_already_presented_neighbors; }
  bool IsetAlreadyPresentedNeighbors(const QJsonArray& already_presented_neighbors){ m_profile.m_mp_settings.m_already_presented_neighbors = already_presented_neighbors; return true; }
  bool IdeleteNeighbors(
    const QString& n_id,
    const QString& connection_type,
    const QString& mp_code = getSelectedMProfile());

  void IonAboutToQuit(MainWindow* w);



  QString ImapThreadCodeToPrefix(const QString& code);

};

#endif // CMACHINE_H


#include "stable.h"

#include <mutex>

#include <qdebug.h>

#include "mainwindow.h"
#include "machine_backup.cpp"
#include "machine_neighbor.cpp"
#include "machine_services_interests.cpp"
//#include "threads_gap.cpp"

#include "machine_handler.h"

//  -  -  -  EmailSettings
QJsonObject EmailSettings::exportJson() const
{
  return QJsonObject
  {
    {"m_address", m_address},
    {"m_password", m_password},
    {"m_income_IMAP", m_income_IMAP},
    {"m_income_POP3", m_income_POP3},
    {"m_incoming_mail_server", m_incoming_mail_server},
    {"m_outgoing_mail_server", m_outgoing_mail_server},
    {"m_outgoing_SMTP", m_outgoing_SMTP},
    {"m_fetching_interval_by_minute", m_fetching_interval_by_minute}, // it depends on smtp server, but less than 5 minute is useless
    {"m_PGP_private_key", m_PGP_private_key},
    {"m_PGP_public_key", m_PGP_public_key}
  };
}

void EmailSettings::importJson(const QJsonObject& obj)
{
  m_address = obj.value("m_address").toString();
  m_password = obj.value("m_password").toString();
  m_income_IMAP = obj.value("m_income_IMAP").toString();
  m_income_POP3 = obj.value("m_income_POP3").toString();
  m_incoming_mail_server = obj.value("m_incoming_mail_server").toString();
  m_outgoing_mail_server = obj.value("m_outgoing_mail_server").toString();
  m_outgoing_SMTP = obj.value("m_outgoing_SMTP").toString();
  m_fetching_interval_by_minute = obj.value("m_fetching_interval_by_minute").toString();
  m_PGP_private_key = obj.value("m_PGP_private_key").toString();
  m_PGP_public_key = obj.value("m_PGP_public_key").toString();
}




//  -  -  -  MPSetting
MPSetting::MPSetting()
{

}

void MPSetting::importJson(const QJsonObject& obj)
{
  m_machine_alias = obj.value("m_machine_alias").toString();
  m_language = obj.value("m_language").toString();
  m_term_of_services = obj.value("m_term_of_services").toString();
  m_machine_alias = obj.value("m_machine_alias").toString();
  m_already_presented_neighbors = obj.value("m_already_presented_neighbors").toArray();
  m_backer_detail = new UnlockDocument();
  m_backer_detail->importJson(obj.value("m_backer_detail").toObject());
  m_public_email.importJson(obj.value("m_public_email").toObject());
  m_private_email.importJson(obj.value("m_private_email").toObject());
}

QJsonObject MPSetting::exportJson() const
{
  return QJsonObject
  {
    {"m_machine_alias", m_machine_alias},
    {"m_language", m_language},
    {"m_term_of_services", m_term_of_services},
    {"m_machine_alias", m_machine_alias},
    {"m_backer_detail", m_backer_detail->exportJson()},
    {"m_public_email", m_public_email.exportJson()},
    {"m_private_email", m_private_email.exportJson()},
    {"m_already_presented_neighbors", m_already_presented_neighbors}
  };
}






//  -  -  -  MachineProfile

MachineProfile::MachineProfile()
{

}

MachineProfile::MachineProfile(const QString& mp_code)
{
  QueryRes prf = DbModel::select(
    CMachine::stbl_machine_profiles,
    {"mp_code", "mp_name", "mp_settings"},
    {{"mp_code", mp_code}}
  );
  if (prf.records.size() == 1)
  {
    QJsonObject profile = CUtils::parseToJsonObj(prf.records[0]["mp_settings"]);
    importJson(profile);
  }
}

MachineProfile::MachineProfile(const QJsonObject& profile)
{
  importJson(profile);
}

void MachineProfile::importJson(const QJsonObject& obj)
{
  m_mp_code = obj.value("m_mp_code").toString();
  m_mp_name = obj.value("m_mp_name").toString();
  m_mp_last_modified = obj.value("m_mp_last_modified").toString();
  m_mp_settings.importJson(obj.value("m_mp_settings").toObject());
}

QJsonObject MachineProfile::exportJson() const
{
  return QJsonObject
  {
    {"m_mp_code", m_mp_code},
    {"m_mp_name", m_mp_name},
    {"m_mp_last_modified", m_mp_last_modified},
    {"m_mp_settings", m_mp_settings.exportJson()}
  };
//  return m_this_json;
}





/**
 * the machine can have 1 or more diffrent profile(s)
 * each profile has it's own public/private email&  public/private iPGP key pairs&  it's neightbors set&
 * and it's wallet addresses and
 * machine_onchain_contracts
 * and maybe kvalue
 *
 * adding profile field to all tables
 * machine_tmp_documents, machine_buffer_documents
 * machine_neighbors, machine_wallet_addresses, machine_wallet_funds
 * machine_draft_proposals, machine_used_utxos, machine_ballots, machine_draft_pledges
 *
 *   // the status be booting, synching, ready
  // booting: when nodes starts to connecting to network for first time
  // synching: when latest confirmed blocks are created before 12 hours ago
  // ready: the node has some confirmed blocks created in last 12 hour in his locl DB
  // status: constants.NODE_IS_BOOTING,
  // lastConfirmedBlockDate: IMAGINE LAUNCH DATE,

  // machine email setting
  // each node has 2 email addreess, public&  private to resist against the spamming...
  // TODO: maybe machine have to have ability to have more than one email to comunicate to prevent against any censorship

 */






// -  -  -  -  -  -  CMachine  -  -  -  -

std::mutex mutex_cached_blocks; //m_DAG_cached_blocks
std::mutex mutex_cached_block_hashes;
std::mutex mutex_cached_spendable_coins;
std::mutex mutex_cached_coins_visibility;

CMachine CMachine::s_instance;
const QString CMachine::stbl_machine_profiles = "c_machine_profiles";
const QString CMachine::stbl_machine_neighbors = "c_machine_neighbors";

//CMachine& CMachine::get()
//{
//  return s_instance;
//}

QString CMachine::IgetLaunchDate()
{
  if (CConsts::LAUNCH_DATE != "")
    return CConsts::LAUNCH_DATE;

  return m_develop_launch_date;
}

void CMachine::IsetLaunchDateAndCloneId(const CDateT& cDate, uint8_t clone_id)
{
  m_develop_launch_date = cDate;
  if (clone_id != 0)
    m_clone_id = clone_id;
}

GenRes CMachine::initDefaultProfile()
{

  m_profile = MachineProfile(CConsts::DEFAULT);
  if (m_profile.m_mp_code == CConsts::DEFAULT)
    return { true, "The Default profile Already initialized" };

  // initializing default valuies and save it

  m_profile.m_mp_code = CConsts::DEFAULT;
  m_profile.m_mp_name = CConsts::DEFAULT;
  m_profile.m_mp_last_modified = CUtils::getNow();

  {
    // initializing email PGP pair keys (it uses native node.js crypto lib. TODO: probably depricated and must refactor to use new one)
    auto[status, private_PGP, public_PGP] = CCrypto::nativeGenerateKeyPair();
      if (!status)
        return {false, "Couldn't creat RSA key pairs (for private channel)"};
    m_profile.m_mp_settings.m_private_email.m_PGP_private_key = private_PGP;
    m_profile.m_mp_settings.m_private_email.m_PGP_public_key = public_PGP;
  }
  {
    // initializing email PGP pair keys (it uses native node.js crypto lib. TODO: probably depricated and must refactor to use new one)
    auto[status, private_PGP, public_PGP] = CCrypto::nativeGenerateKeyPair();
      if (!status)
        return {false, "Couldn't creat RSA key pairs (for public channel)"};
    m_profile.m_mp_settings.m_public_email.m_PGP_private_key = private_PGP;
    m_profile.m_mp_settings.m_public_email.m_PGP_public_key = public_PGP;
  }


  auto[status, unlock_doc] = CAddress::createANewAddress(
    CConsts::SIGNATURE_TYPES::Strict,
    "2/3",
    "0.0.1");
  if (!status)
    return {false, "Couldn't creat ECDSA key pairs (for public channel)"};

  m_profile.m_mp_settings.m_backer_detail = new UnlockDocument();
  m_profile.m_mp_settings.m_backer_detail = &unlock_doc;
  m_profile.m_mp_settings.m_machine_alias = "node-" + CUtils::hash6c(CCrypto::keccak256(unlock_doc.m_account_address));

  {
    // this block existed ONLY for test and development environment
    QString AAA_PGP_private_key = "-----BEGIN PRIVATE KEY-----\n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\nmNI4cf8hgrmIb2AgIyZ9LJMhSDaF+g==\n-----END PRIVATE KEY-----";
    QString AAA_PGP_public_key = "-----BEGIN PUBLIC KEY-----\nMIICIDANBgkqhkiG9w0BAQEFAAOCAg0AMIICCAKCAgEAjSeu+Qny40lpzFfDy1XN\nWYyNHAldVUwH2I++PwV+xN9LE0JIbg3kh4BMHB5zN468qmlvE8QvTYbA2NNkHDYH\ngbi+cszEZveERJK8i34ZqqrgFh8+nBiRExVFjmtJMqxjUpGUIURlw4U0/EC3fM5O\nuQjX4z992F0L0vs0gWgFd3o3mapoh+NPTeyJ+b6IZ8JUm0Mw/i0hLGyBoK5wCY1Z\npqBuMmnK+st8/QG2UxS1TifiEy/zSFDdmvJ14B2CdckAY3PEtzCPYnm7Q1NpM9vs\nDdjrkOtiaREJaqvfuEw9g9IlECb5wkmXUNpWMfgd31ecD766SK/YP8dqR5zVW+Vh\ndp8FtRZGiGyWbOglyARKh4GC37LoJZJk8uTvp1FmEnUZK6nSSDmDzNAq8M8849zH\nMGXpoRd2A04nmuL5JAA2WpkW7WQqbvpmunXHRrOKt9cFmE6zuslm8TiRTigjD69Y\nOuSl48x7o/3LLJw4dbgPlmYi5Ezf2ZncllFSbIPxXSP5t95cqOrxTA7/Ue8/VJnu\nD3GJym8lXPFn8zHMjVH9ujxpm69+0XqSsBnkiQtcv0qeaPEBMobbABBcvMumvUAb\ndU1oMVr4Md+5Oapn7M03w+ZNIqofZtQFbmEiWajAYTixhYQHSqLXdp1YXgVIcfBa\nIyo+tB2Zu7WF6b4mp0tD0tUCARE=\n-----END PUBLIC KEY-----";

    QString BBB_PGP_private_key = "-----BEGIN PRIVATE KEY-----\nMIIJPwIBADANBgkqhkiG9w0BAQEFAASCCSkwggklAgEAAoICAQDOu5d2Gh1c94ex\noyA1LDpQ3ixFUZd5BGuLw8ngQUYq5NxUXr/ZlbL4j9UceirVj/Xm+b9EVH9B+K31\nMiCL6nZ4LD12MzuOWsq9Nl+z68ArH6onnrHWC7QKNr5GR1sl2WKpUoAtl9jT6NZp\nyj7Mf564Tyo+NTKBSghLOaw11xms02LZ4snTI0xVrjHnLRjTLC6Em9vHAx+91HEy\n7LRhnBwLyLmWQI8I8qOv07NH6MLvB5Qz878eZ+ok4WFeIIpe+NdoFl0S3lapTzqU\nxESWT2leHKCU6Ws97/f2fUGGzTC7gwNuFytc+Pyl8SbGmWFB9pHf97PHBXFjQwR9\n8UaUyBfrRHCgSBHsFfUFm/arCnsoF/uBhgl45VgKPF1sphEEt04x+pDetdu2mWOK\nhrX3vldm7dsAfQHKEoo9kqpUCkvewDU+bu9aNLxcRQ5wuAsrFh6qOtl5N6zRVbfT\nL+0eeRQ4dPTNXxJinC5LeaBCZuK+u8IuF0BgTV7wcbO1vZuEE8exCAGepGd80MfK\nsSsxAcF/BdPv243+jKPgJF6gyp+CbSf8YfZmKMpv3gtHYwwd5OtPE6Hesj3i3QcK\npXEHsqyHYfkf3KdnphS0zQVBAiPNSBT9tNC4BAeo4FOKJIIoUas9/SJRxjS23+lf\nwdAw/zbMtlcospc1aBF9MBeJlM7NVQIBEQKCAgAeZuGRXjF+nN8/xSpiLCaxihWR\nuSzdFzz99yU3kSDoMLb9WTpUtCHZQlQLt5zjK8JHnTK3OZo+aFXRPBPYVy+KJJ+g\ncPIrhdKFPLO4k5xCk7cj8bC9mE8urbKR3VErNo6CT+WsWhhbZgFp6Qk8MOKiojrr\nB9K4qQE4PS/pzM8R4tnUv3gIdiHQXWGxDilMOzQEcUX3npO6CKc8Md5KlvUQyrHh\nY9jMnCchYuWosUnX23etSX388Sn2XWEkbjJ3YNRiIWgKTd+RXnmOWRklKcu7BDW7\ni7zyhSv+mfMMS1n9dSYmxywGJJ2f7sHwB38+aAZks3xR+UVha7zlWDAG0iGiVWgn\nEfnE02rocfOWaIgVNJjqWzeohcpY8EBmJfYNOIB7GRkD0vv8pP8lMY5kb1f7Ea+P\n3XJrkDxJkNEDbJSmEXu+BIYKkhDg3uxwWmo8MCrhDEVYqwJT8Z+AEPmDj3z/R9E9\nFzRDC/+HHtjJ0qXLoQu6wPoR3lWzgVa3WJcLTnQx6GrFM1Respw5Ew/pr90UedQM\nU7zvH/Hflw7I0q95y91mW8u+LiEI/tApud0cGOtOJjEfD8rXAjP5VeScD0UfgL0E\n1//4PrR37UwgVPTnhn42XoKfu/Wdi9PCGHEUdK+W77CZOAhPhPBbGlrOF05ktw7J\n9O817qC6bXlBQenFPwKCAQEA3P+c5Jz8Oc+hMeNB5jJj1oxmJ+Sa+iQ8GuLnJCtE\n84RFPs1ybDK0b04h0xs4kt5IQrueTrFf3b2EdsPUpdGq+vN2Ip1LX/sg6SdN7jW+\n5SGjHhnNK5HQDb81rkDZ1ftvgeMn5l/6C22H1LHiL5r3aFMEuanqyTrNW2v+Ulmb\nUd8zbAYEpZdS8hrEpSpgj6M8qxrwVQ+CPUwBphC3wQ2WJBgh09xr87wYf9mmogzO\nAB534Z+yT9zcmjK/2Q1NqOtWJCgHmyRW9jrzhAZwSUX94/4gLYhuG9jlNNw8OlDY\nbE8tmrE0hRPVYugiPe67u80KN2gHqiEazlNbAjHBU/CUawKCAQEA73mTT59PC05q\n/M43KkUA28Wui3cww9PXv8SbsvYyqfna3eYfFV3lyBs4ldJ6nLxzZg8pf6zB+Zkr\nQyYBtVQw5/hVxyj6JrTb1mwmyWlsgwymYugd56TGdojQQXH5OcbN14LS2uigpZDG\nbMXQnuBepewzcIuVnEPE2foL3aU1eTjD1fJdCCwMU1OB8MfOkwu3HkpFTRNVdh5U\nexyTcOiwEfsihEogLpQaMxCFBFL3O1mQlLQl9v22mj+rUR37O4TBa/73UYNFa9te\nlPkoEMLcN+/SN9ALttvHjj2/qmORmLIOBI0LXbUrKDhrnXf72CF8CIqQhuhFdkX7\nRXoMgKiVPwKCAQAm/+6CskqgykmfZFbsYz7LgjAlKFeVjex9Nxm7FrHQnt8LFTJP\nVD31hkI0UBkK2+6iXVgsAS8JA1OcfOlKcEtZdkIGG8IB4QXOyrNmRbhGjXcjbfcH\nsFHkTuta/GKtSn0W69ndXDsvMXJStfq9G1jWLMSZPBpfvxUuQDvwaip33BgiHy3/\nGrRI14wdJZiR0YMtQP08L+nOlPE7bFypmPxguPbpJuXft8gWj9IcmNkPFG+CKz2V\nn3I5VD/5IHcdzy1RrLYMUbT+RqNxpsiFZrRVaRS8vbkT+RljrmT7O3F8hnF1ps0I\nbOlrzpyhhHt7folVElu0nG4kaRAPcjEs7jhPAoIBABwsa68DrvJFdf+fykE1S2Um\nUMUdFMu+kdpTXZyVb19Kkjg5MNVWV0S36IoYwyF/lRsQ17Sq6aTk1+nIPG+vjUh3\nkZ71wxOczpGyXuqE35bybe2EuDlere/T3EPvSn9EkK/xRfui5bkgF1gXRbhWobkq\n2OAQa/RENUbSH4N82R1R+Ov+ZUxBatygaaPbRXq2FYsXy+rzNzsSoIb0TZTQFLbS\nQEvMfEG3EiQgD6Yn4NnOTT6ryDss6E5h1+ts8GFa6ZQ8HRimCCrOg5kOQPLpv44c\nNtljxSSSU7ZhnhQLtsariS22PZKNyNeOKsc7Ss4iDpeX1MSTy+/L/3GV41puL60C\nggEAa2qQ5OS9YzWWmX4PXDt8vS5yLS0gDIwYzbSIHza4NTUOxH06ZfpcJoT/JA+D\nPK3ND/9F4cq24NV8E/aJ0tbqTdNDlokspXGyMgzptG3Ddo8Wh9xpUasZDhrUNXPT\nW1mlzTTrXlhYSnRNak6YYTDQY528JP1GaaL7RL7KcgkOGmSwQU64WVHaQ3MJnmhN\nVUwJuqQsFUTMy30h/w1GInpuLNh5YIWf7/V0hoNCCTGH8mBeYnFxrBGNTCfpeq5Z\nRF/FOP+NjVh4KGz/SFnlmSDnek7zaNoFJ7OhGwJtTrFCjSevRgjbK3XUvXtHEgDn\nbxC8SYNfb397H1VNQUzlPIW6oA==\n-----END PRIVATE KEY-----";
    QString BBB_PGP_public_key = "-----BEGIN PUBLIC KEY-----\nMIICIDANBgkqhkiG9w0BAQEFAAOCAg0AMIICCAKCAgEAzruXdhodXPeHsaMgNSw6\nUN4sRVGXeQRri8PJ4EFGKuTcVF6/2ZWy+I/VHHoq1Y/15vm/RFR/Qfit9TIgi+p2\neCw9djM7jlrKvTZfs+vAKx+qJ56x1gu0Cja+RkdbJdliqVKALZfY0+jWaco+zH+e\nuE8qPjUygUoISzmsNdcZrNNi2eLJ0yNMVa4x5y0Y0ywuhJvbxwMfvdRxMuy0YZwc\nC8i5lkCPCPKjr9OzR+jC7weUM/O/HmfqJOFhXiCKXvjXaBZdEt5WqU86lMRElk9p\nXhyglOlrPe/39n1Bhs0wu4MDbhcrXPj8pfEmxplhQfaR3/ezxwVxY0MEffFGlMgX\n60RwoEgR7BX1BZv2qwp7KBf7gYYJeOVYCjxdbKYRBLdOMfqQ3rXbtpljioa1975X\nZu3bAH0ByhKKPZKqVApL3sA1Pm7vWjS8XEUOcLgLKxYeqjrZeTes0VW30y/tHnkU\nOHT0zV8SYpwuS3mgQmbivrvCLhdAYE1e8HGztb2bhBPHsQgBnqRnfNDHyrErMQHB\nfwXT79uN/oyj4CReoMqfgm0n/GH2ZijKb94LR2MMHeTrTxOh3rI94t0HCqVxB7Ks\nh2H5H9ynZ6YUtM0FQQIjzUgU/bTQuAQHqOBTiiSCKFGrPf0iUcY0tt/pX8HQMP82\nzLZXKLKXNWgRfTAXiZTOzVUCARE=\n-----END PUBLIC KEY-----";

    if (m_clone_id == 1)
    {
      m_profile.m_mp_settings.m_machine_alias = "node-AAA";
      m_profile.m_mp_settings.m_public_email.m_address = "AAA@AAA.AAA";
      m_profile.m_mp_settings.m_public_email.m_PGP_private_key = AAA_PGP_private_key;
      m_profile.m_mp_settings.m_public_email.m_PGP_public_key = AAA_PGP_public_key;
      addANewNeighbor(
        "BBB@BBB.BBB",
        CConsts::PUBLIC,
        BBB_PGP_public_key,
        CConsts::DEFAULT);
    }
    if (m_clone_id == 2)
    {
      m_profile.m_mp_settings.m_machine_alias = "node-BBB";
      m_profile.m_mp_settings.m_public_email.m_address = "BBB@BBB.BBB";
      m_profile.m_mp_settings.m_public_email.m_PGP_private_key = BBB_PGP_private_key;
      m_profile.m_mp_settings.m_public_email.m_PGP_public_key = BBB_PGP_public_key;
      addANewNeighbor(
        "AAA@AAA.AAA",
        CConsts::PUBLIC,
        AAA_PGP_public_key,
        CConsts::DEFAULT);
    }
  }



  IsaveSettings();

  // set selected profile=default
  DbModel::upsert(
    "c_kvalue",
    "kv_key",
    "SELECTED_PROFILE",
    QVDicT
    {
      {"kv_value", m_profile.m_mp_code},
      {"kv_last_modified", m_profile.m_mp_last_modified}
    }
  );

  // add backer address to wallet as well
  Wallet::insertAddress( WalletAddress (
    m_profile.m_mp_settings.m_backer_detail,
    CConsts::DEFAULT,   // mp code
    "Backer Address (" +
    m_profile.m_mp_settings.m_backer_detail->m_unlock_sets[0].m_signature_type + " " +
    m_profile.m_mp_settings.m_backer_detail->m_unlock_sets[0].m_signature_ver + ")"));

  ImaybeAddSeedNeighbors();

  return { true, "The Default profile initialized" };
}

void CMachine::emptyDB()
{
  QSqlQuery* query;
  if (CConsts::DATABASAE_AGENT == "psql")
  {
    query = new QSqlQuery(*DbHandler::getPSQLDB("db_comen_general"));

  }else{
    // QSqlQuery query(DbHandler::getDB(database_name));
  }

  for (std::string a_table: psql_tables_list)
  {
    QString query_string = "DELETE FROM " + QString::fromStdString(a_table);
    CLog::log("cleaning tables: " + query_string, "sql", "info");

    bool queryRes = query->exec(query_string);
    if (query->lastError().type() != QSqlError::NoError || !queryRes)
    {
      qDebug() << query->lastError().text();
    }
  }
}


bool CMachine::IbootMachine()
{
  srand (time(NULL));

  //  if (![1, 4].includes(appCloneId))
  //    machine.neighborHandler.addSeedNeighbors()
  //  //TODO: start remove it BEFORERELEASE
  //  if (appCloneId == 1) {
  //    let issetted = model.sRead({
  //        table: 'i_kvalue',
  //        fields: ['kv_value'],
  //        query: [
  //            ['kv_key', 'setSampleMachines']
  //        ]
  //    });
  //    if (issetted.length > 0)
  //        return { err: false, msg: 'sample machines already setted' };
  //    sampleMachines.setSampleMachines();
  //    db.setAppCloneId(1);
  //  }
  //  //TODO: end remove it BEFORERELEASE
  //  // initialize document & content
  //  let initRes = initContentSetting.doInit();
  //  if (initRes.err != false)
  //    return initRes;
  //  let doesSafelyInitialized = initContentSetting.doesSafelyInitialized();
  //  if (doesSafelyInitialized.err != false) {
  //    utils.exiter(`doesSafelyInitialized ${doesSafelyInitialized}`, 609);
  //  }
  //  return { err: false, msg: 'The machine fully initilized' };

  int clone_id = CMachine::getAppCloneId();
  CLog::log(QString("Booting App(%1) ").arg(clone_id));


  m_global_configs = new Config {};
  m_global_configs->load();

  m_last_sync_status_check = getLaunchDate();


  // control DataBase
  if (!m_db_is_connected)
  {
    m_db_is_connected = true; // just a temporary lock to provide enough time to initialize db(if it is necessary)
    auto[status, msg] = DbHandler::initDb();
    m_db_is_connected = status;
    if (!status)
    {
      CLog::log(msg);
      CUtils::exiter("Coudn't establish database connections!", 11);
    }
  }

  // check if flag MACHINE_AND_DAG_ARE_SAFELY_INITIALIZED is setted
  QString is_inited = KVHandler::getValue("MACHINE_AND_DAG_ARE_SAFELY_INITIALIZED");
  if (is_inited != CConsts::YES)
  {
    emptyDB();  // machine didn't initialized successfully, so empty all tables and try from zero
    KVHandler::setValue("MACHINE_AND_DAG_ARE_SAFELY_INITIALIZED", CConsts::NO);
  }

  GenRes res = initDefaultProfile();
  if (res.status != true)
      return false;

  // load machine settings
  loadSelectedProfile();

  m_machine_is_loaded = true;
  s_DAG_is_initialized = true;


  {
    // remove this block after improving db and fixing database block problem
    auto[status, coins] = cachedSpendableCoins("read");
    if (!status)
    {
      CLog::log("couldn't read from cached Spendable Coins!", "app", "fatal");
    }

    if (coins.size() < 500)
    {
      QueryRes exist_coins = DbModel::select(
        "c_trx_utxos",
        {"ut_coin", "ut_creation_date", "ut_ref_creation_date", "ut_visible_by", "ut_o_address", "ut_o_value"});
      cachedSpendableCoins("assign", exist_coins.records);
    }
  }

  return true;
}

void CMachine::parseArgs(int argc, char *argv[], int manual_clone_id)
{
  qDebug() << argc;
  for (int param_inx = 0; param_inx < argc; param_inx++)
    qDebug() << param_inx << " " << argv[param_inx];

  uint8_t clone_id = 0;    // FIXME: this value must be defined by command line
  if (argc > 1)
    clone_id = atoi(argv[1]);

  if (manual_clone_id > 0)
    clone_id = manual_clone_id;

  bool is_develop_mod = false;
  if (argc > 2)
    is_develop_mod = true;

//  clone_id = 1;
  qDebug() << " clone_id " << clone_id;

  setCloneDev(clone_id, is_develop_mod);

}

QString CMachine::IgetHDPath()
{
  QString hd_files = "hd_files"; // CConsts::HD_FILES;
  if (getAppCloneId() > 0)
    hd_files += QString::number(getAppCloneId());
  return hd_files;
}

QString CMachine::IgetReportsPath()
{
  return getHDPath() + "/reports";
}

QString CMachine::IgetInboxPath()
{
  return getHDPath() + "/inbox";
}

QString CMachine::IgetOutboxPath()
{
  return getHDPath() + "/outbox";
}

QString CMachine::IgetLogsPath()
{
  return getHDPath();
}

QString CMachine::IgetDAGBackup()
{
  return getHDPath() + "/dag_backup";
}

bool CMachine::IcreateFolders()
{
  if (CConsts::HD_ROOT_PATHE != "")
    if (!QDir(CConsts::HD_ROOT_PATHE).exists())
      QDir().mkdir(CConsts::HD_ROOT_PATHE);

  if (!QDir(getHDPath()).exists())
    QDir().mkdir(getHDPath());

  if (!QDir(getReportsPath()).exists())
    QDir().mkdir(getReportsPath());

  if (!QDir(getInboxPath()).exists())
    QDir().mkdir(getInboxPath());

  if (!QDir(getOutboxPath()).exists())
    QDir().mkdir(getOutboxPath());

  if (!QDir(getDAGBackup()).exists())
    QDir().mkdir(getDAGBackup());

  QString cache_path = getCachePath();
  if (getAppCloneId() > 0)
  if (!QDir(cache_path).exists())
    QDir().mkdir(cache_path);


  return true;
}

QString CMachine::IgetBackerAddress()
{
  return m_profile.m_mp_settings.m_backer_detail->m_account_address;
}

UnlockDocument* CMachine::IgetBackerDetails()
{
  return m_profile.m_mp_settings.m_backer_detail;
}

bool CMachine::loadSelectedProfile()
{
  QueryRes selected_prof = DbModel::select(
    "c_kvalue",
    QStringList {"kv_value"},     // fields
    {ModelClause("kv_key", "SELECTED_PROFILE")}
  );
  if (selected_prof.records.size() != 1) {
      return false;
  }


  MachineProfile mp(selected_prof.records[0]["kv_value"].toString());
  m_profile = mp;
  return true;
}

QString CMachine::IgetSelectedMProfile()
{
  if (m_selected_profile != "")
    return m_selected_profile;

  QueryRes res = DbModel::select(
    "c_kvalue",
    {"kv_value"},
    {{"kv_key", "SELECTED_PROFILE"}}
  );
  // console.log('resresresresres', res);
  if (res.records.size() != 1) {
    CLog::log("NoooOOOOOOOOOOOOOOooooooooooooo profile!", "app", "fatal");
    exit(345);
  }
  m_selected_profile = res.records[0].value("kv_value").toString();
  return m_selected_profile;
}

// - - - - - - cycle functions - - - - -
uint64_t CMachine::getCycleByMinutes()
{
  if (CConsts::TIME_GAIN == 1){
    return CConsts::STANDARD_CYCLE_BY_MINUTES;
  }
  return CConsts::TIME_GAIN;
}

TimeByHoursT CMachine::getMinVotingTimeframe()
{
  TimeByHoursT voting_timeframe = (CMachine::getCycleByMinutes() * 2) / 60;   // at least 2 cycle for voting
  if (voting_timeframe == static_cast<uint64_t>(voting_timeframe))
    return voting_timeframe;
  return CUtils::customFloorFloat(static_cast<double>(voting_timeframe), 2);
}

bool CMachine::IsetPublicEmailAddress(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_address = v;
  return true;
}

bool CMachine::IsetPublicEmailInterval(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_fetching_interval_by_minute = v;
  return true;
}

bool CMachine::IsetPublicEmailIncomeHost(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_incoming_mail_server = v;
  return true;
}

bool CMachine::IsetPublicEmailPassword(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_password = v;
  return true;
}

bool CMachine::IsetPublicEmailIncomeIMAP(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_income_IMAP = v;
  return true;
}

bool CMachine::IsetPublicEmailIncomePOP(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_income_POP3 = v;
  return true;
}

bool CMachine::IsetPublicEmailOutgoingSMTP(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_outgoing_SMTP = v;
  return true;
}

bool CMachine::IsetPublicEmailOutgoingHost(const QString&  v)
{
  m_profile.m_mp_settings.m_public_email.m_outgoing_mail_server = v;
  return true;
}

bool CMachine::IsetTermOfServices(const QString&  v)
{
  m_profile.m_mp_settings.m_term_of_services = v;
  return true;
}

bool CMachine::IsaveSettings()
{
  QString serialized_setting = CUtils::serializeJson(m_profile.exportJson());
  QVDicT values {
//    {"mp_code", m_profile.m_mp_code},
    {"mp_name", m_profile.m_mp_name},
    {"mp_settings", serialized_setting},
    {"mp_last_modified", m_profile.m_mp_last_modified}
  };
  DbModel::upsert(
      CMachine::stbl_machine_profiles,
      "mp_code",
      m_profile.m_mp_code,
      values,
      true);

  return true;
}


bool CMachine::ImaybeAddSeedNeighbors()
{
  addANewNeighbor("matbit@secmail.pro", CConsts::PUBLIC);
  return true;
}


QJsonObject CMachine::IgetLastSyncStatus()
{
  QString lastSyncStatus = KVHandler::getValue("LAST_SYNC_STATUS");
  if (lastSyncStatus == "")
  {
    get().IinitLastSyncStatus();
    lastSyncStatus = KVHandler::getValue("LAST_SYNC_STATUS");
  }
  return CUtils::parseToJsonObj(lastSyncStatus);
}

bool CMachine::IinitLastSyncStatus()
{
    QJsonObject lastSyncStatus {
      {"isInSyncMode", "Unknown"},
      {"lastTimeMachineWasInSyncMode",
        CUtils::minutesBefore(CMachine::getCycleByMinutes() * 2)},
      {"checkDate", CUtils::minutesBefore(CMachine::getCycleByMinutes())},
      {"lastDAGBlockCreationDate", "Unknown"}
    };
    return KVHandler::upsertKValue(
      "LAST_SYNC_STATUS",
      CUtils::serializeJson(lastSyncStatus));
}


/**
 * @brief CMachine::signByMachineKey
 * @param sign_message
 * @param unlock_index
 * @return {status, signer address, unlock set, signatures}
 */
std::tuple<bool, QString, UnlockSet, QStringList> CMachine::signByMachineKey(
  const QString& sign_message,
  const CSigIndexT& unlock_index)
{
  QString signer = getBackerAddress();
  auto[sign_status, res_msg, sign_signatures, sign_unlock_set] = Wallet::signByAnAddress(
    signer,
    sign_message,
    unlock_index);
  if (!sign_status)
  {
    return {false, "", {}, {}};
  }

  UnlockSet uSet {};
  uSet.importJson(sign_unlock_set);
  return {true, signer, uSet, sign_signatures};

}

QString CMachine::getCachePath(QString appCloneId)
{
  if (appCloneId == "")
    appCloneId = getAppCloneIdStr();
  return CConsts::HD_FILES + appCloneId + "/" + "cache_files";
}

void CMachine::IonAboutToQuit(MainWindow* w)
{
  setShouldLoopThreads(false);
  bool any_thread_is_runing = true;
  int i = 0;

  for (QString a_thread: m_threads_status.keys())
    if (m_threads_status[a_thread] == CConsts::THREAD_STATE::SLEEPING)
      CLog::log("Gracefully stopped sleeping thread(" + a_thread + ")");

  while (any_thread_is_runing && (i < 3000))
  {
    i++;
    std::this_thread::sleep_for(std::chrono::seconds(1));
    any_thread_is_runing = false;
    for (QString a_thread: m_threads_status.keys())
      if (m_threads_status[a_thread] == CConsts::THREAD_STATE::RUNNING)
      {
        if ((i > 10) && (i%5==0))
          CLog::log("The thread (" + a_thread + ") still is running!");

        any_thread_is_runing = true;
      }
  }

  if (w)
    w->saveConfigurationParameters();

  CLog::log("Preparing to shout down...");
  m_global_configs->save();

  delete m_global_configs;

  DbHandler::closeConnections(); //TODO: use delete &DbHandler::get();

  m_cache_coins_visibility = QStringList {};

  CLog::log("Gracefully shouted down");
}


QString CMachine::ImapThreadCodeToPrefix(const QString& code)
{
  if (m_map_thread_code_to_prefix.keys().contains(code))
    return m_map_thread_code_to_prefix.value(code);
  return "Un-assigned thread(" + code + ")!";
}

//bool CMachine::IaddToCachedBlocks(const QVDicT& block)
//{
//  try {
//    // using a local lock_guard to lock mtx guarantees unlocking on destruction / exception:
//    std::lock_guard<std::mutex> lck ();
//    m_DAG_cached_blocks.push_back(block);
//    return true;
//  }
//  catch (std::logic_error&) {
//    std::cout << "[exception caught]\n";
//    return false;
//  }
//}

std::tuple<bool, QVDRecordsT&> CMachine::IcachedBlocks(
  const QString& action,
  const QVDRecordsT& blocks,
  const QString& status)
{
  try {
    // using a local lock_guard to lock mtx guarantees unlocking on destruction / exception:
    std::lock_guard<std::mutex> lck (mutex_cached_blocks);

    if (action == "assign")
    {
      m_DAG_cached_blocks = blocks;
    }

    if (action == "append")
    {
      for (QVDicT a_block: blocks)
        m_DAG_cached_blocks.push_back(a_block);
    }

    if (action == "update")
    {
      for (QVDicT a_block: blocks)
        for (int i = 0 ; i < m_DAG_cached_blocks.size(); i++)
          if (m_DAG_cached_blocks[i].value("b_hash").toString() == a_block.value("b_hash").toString())
            m_DAG_cached_blocks[i]["b_utxo_imported"] = status;
    }

    return {true, m_DAG_cached_blocks};

  }
  catch (std::logic_error&)
  {
    QString thread_code = QString::number((quint64)QThread::currentThread(), 16);
    CLog::log("Failed in cached blocks action(" + action + ") Thread(" + thread_code + " / " + mapThreadCodeToPrefix(thread_code)+ ")");
    std::cout << "[exception caught]\n";
    return {false, m_DAG_cached_blocks};
  }
}

std::tuple<bool, QStringList&> CMachine::IcachedBlockHashes(
  const QString& action,
  const QStringList& block_hashes)
{
  try {
    // using a local lock_guard to lock mtx guarantees unlocking on destruction / exception:
    std::lock_guard<std::mutex> lck (mutex_cached_block_hashes);

    if (action == "assign")
    {
      m_DAG_cached_block_hashes = block_hashes;
    }

    if (action == "append")
    {
      for (QString a_hash: block_hashes)
        m_DAG_cached_block_hashes.push_back(a_hash);
    }

    return {true, m_DAG_cached_block_hashes};

  }
  catch (std::logic_error&)
  {
    QString thread_code = QString::number((quint64)QThread::currentThread(), 16);
    CLog::log("Failed in cached block hashes action(" + action + ") Thread(" + thread_code + " / " + mapThreadCodeToPrefix(thread_code)+ ")");
    std::cout << "[exception caught]\n";
    return {false, m_DAG_cached_block_hashes};
  }
}

std::tuple<bool, QVDRecordsT> CMachine::IcachedSpendableCoins(
  const QString& action,
  const QVDRecordsT& coins,
  const CBlockHashT& visible_by,
  const CCoinCodeT& the_coin)
{
  try {
    // using a local lock_guard to lock mtx guarantees unlocking on destruction / exception:
    std::lock_guard<std::mutex> lck (mutex_cached_spendable_coins);

    if (action == "assign")
    {
      m_cache_spendable_coins = coins;
    }

    if (action == "append")
    {
      for (QVDicT coin: coins)
        m_cache_spendable_coins.push_back(coin);
    }

    if (action == "remove")
    {
      QVDRecordsT remined_coins = {};
      if ((visible_by != "") || (the_coin != ""))
      {
        for (QVDicT a_coin: m_cache_spendable_coins)
        {
          if ((visible_by != "") && (the_coin != ""))
          {
            if ((a_coin.value("ut_visible_by").toString() != visible_by) || (a_coin.value("ut_coin").toString() != the_coin))
              remined_coins.push_back(a_coin);

          }
          else if (visible_by != "")
          {
            if (a_coin.value("ut_visible_by").toString() != visible_by)
              remined_coins.push_back(a_coin);
          }
          else if (the_coin != "")
          {
            if (a_coin.value("ut_coin").toString() != the_coin)
              remined_coins.push_back(a_coin);
          }
        }

        m_cache_spendable_coins = remined_coins;
      }
    }

    return {true, m_cache_spendable_coins};

  }
  catch (std::logic_error&)
  {
    QString thread_code = QString::number((quint64)QThread::currentThread(), 16);
    CLog::log("Failed in cached spendable coins action(" + action + ") Thread(" + thread_code + " / " + mapThreadCodeToPrefix(thread_code)+ ")");
    std::cout << "[exception caught]\n";
    return {false, m_cache_spendable_coins};
  }
}

CoinsVisibilityRes CMachine::IcachedCoinsVisibility(
  const QString& action,
  const QStringList& entries)
{
  try {
    // using a local lock_guard to lock mtx guarantees unlocking on destruction / exception:
    std::lock_guard<std::mutex> lck (mutex_cached_coins_visibility);

    bool contains = true;

    if (action == "assign")
    {
      m_cache_coins_visibility = entries;
    }

    if (action == "append")
    {
      for (QString a_visiblity: entries)
        m_cache_coins_visibility.push_back(a_visiblity);
    }

    if (action == "contains")
    {
      contains = m_cache_coins_visibility.contains(entries[0]);
    }

    return CoinsVisibilityRes {true, m_cache_coins_visibility, contains};

  }
  catch (std::logic_error&)
  {
    QString thread_code = QString::number((quint64)QThread::currentThread(), 16);
    CLog::log("Failed in cached spendable coins action(" + action + ") Thread(" + thread_code + " / " + mapThreadCodeToPrefix(thread_code)+ ")");
    std::cout << "[exception caught]\n";
    return CoinsVisibilityRes {false, m_cache_coins_visibility, false};
  }
}


double CMachine::getMinPollingTimeframeByHour()
{
  return (getCycleByMinutes() * 2.0) / 60.0;
}

 */