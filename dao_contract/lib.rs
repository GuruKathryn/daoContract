#![cfg_attr(not(feature = "std"), no_std,no_main)]
#![feature(min_specialization)]

use ink::env::Environment;
// use ink_lang as ink;


// rust 
// ink comtract 

#[ink::chain_extension]
pub trait FetchRandom {
type ErrorCode = RandomReadErr;


#[ink(extension = 1)]
fn fetch_random(subject: [u8; 32]) -> [u8; 32];
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum RandomReadErr {
FailGetRandomSource,
}

impl ink::env::chain_extension::FromStatusCode for RandomReadErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetRandomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = FetchRandom;
}

// impl ink::env::chain_extension::FromStatusCode for RandomReadErr {
// fn from_status_code(status_code: u32) -> Result<(), Self> {
//    match status_code {
//        0 => Ok(()),
//        1 => Err(Self::FailGetRandomSource),
//        _ => panic!("encountered unknown status code"),
//    }
// }
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
// pub enum CustomEnvironment {}

// impl Environment for CustomEnvironment {
// const MAX_EVENT_TOPICS: usize =
//    <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

// type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
// type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
// type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
// type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
// type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

// type ChainExtension = FetchRandom;
// }




#[openbrush::contract(env=crate::CustomEnvironment)]
mod dao_contract {

    use super::*;
    use ink::prelude::string::{ ToString}; 
    use openbrush::contracts::psp22::Data;
    // use ink::primitives::AccountId;
    // 
    use openbrush::traits::{String};  // Vec 
    // use dao_governance_token::dao_governance_token::{DaoGovernanceToken, DaoGovernanceTokenRef};
    // use dao_governance_token::dao_governance_token::DaoGovernanceTokenRef;
    use ink::prelude::{vec,vec::Vec};
    // use ink::storage::traits::StorageLayout;
    use ink::storage::Mapping;
    use rand_chacha::ChaChaRng;
    // use ink::storage::traits::{PackedLayout, SpreadLayout};
    use ink::env::{ *};
    use scale::{
        Decode,
        Encode,
    };
    use fixed::{types::extra::{U3,U4}, FixedI128};
    // use scale_info::Registry;
 
    type ProjectId = u16;
    type TaskId = u16;
    type MemberId = u16;
    type DaoId = u16;
    type Fix = FixedI128<U4>;
    const min_deposit_balance: Balance = 1_000_000_000;
    const SECONDS_PER_DAY: u64 = 86400;
    pub type ResultTransaction<T> = core::result::Result<T, Error>;
    // pub type ResultOwner<T> = core::result::Result<T, OwnableError>;
    pub type Result<T> = core::result::Result<T, Error>;
    // pub type ResultTransfer<T> = core::result::Result<T, ink_env::Error>;
    pub type ExtensionResult<T,E>  =  core::result::Result<T, E>;
    // / Defines the storage of your contract.
    // / Add new fields to the below struct in order
    // / to add new static storage fields to your contract.
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// 
    /// effieciency message and testing 
    /// 
    pub type Periodicity = u8; 
    pub type Achieved_goals = u8; 
    #[ink(storage)]
    pub struct FelideaDao {

         ///dao authority
         dao_admin:AccountId,
         ///daoID
         dao_id:[u8;16],
        //  dao info 
         dao_info:DaoInfo,
        //  token address => token info
         token_list_for_address: Mapping<AccountId, TokenInfo>,
         ///track the id's of member
         next_member_id: u16,
         ///AccountId => MemberInfo
         dao_member_list:Mapping<AccountId, MemberInfo>,
         /// ( DAO address , member_id ) => MemberInfo
         member_infoes_from_id: Mapping< MemberId, MemberInfo>,
         ///memberid => address 
         memberid_to_address:Mapping<MemberId, AccountId>,
         ///projectId => ProjectInfo
         dao_project_list:Mapping<ProjectId, ProjectInfo>,         
         ///daoAddress => ProjectId
         next_project_id: u16,    
         /// TaskId => TicketInfo
         dao_task_list:Mapping<TaskId, TicketInfo>,     
          ///(daoAddress,ProjectId) => taskId
         next_ticket_id: u16,
        ///member address => stake
        staking_data: Mapping<AccountId, Stake>,
        ///dao_stake 
        dao_staking_data:Mapping<DaoId,StakeForDao>,
        ///interest rate
        total_stake: u128, // interest rate per cent
        ///stake duration
        staking_duration: Timestamp,
        ///nextdaoID
        // next_dao_id:u16,
        ///dao_Address => DaoInfo
        dao_list:Mapping<AccountId,DaoInfo>,
        ///next dao id
        next_dao_id:DaoId,
        ///dao_list as next dao id 
        dao_list_from_id:Mapping<DaoId,DaoInfo>
        //token 
        // token:DaoGovernanceTokenRef 
        // keep track of periodicty and achived goals 
        // ticket_periodicity_achived_goals:Mappint<Periodicity,Achieved_goals>       
    }



    #[derive(scale::Encode, scale::Decode, Clone, Default,   PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
     
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct DaoInfo {
        dao_name: String,
        description: String,
        website:Option<String>,
        profile:Option<String>,
        dao_address:Option<AccountId>,
        dao_admin:Option<AccountId>,
        dao_id:Option<DaoId>
    }
    // add a dao ID => random  hash generation 
    // map dao address with dao INFO 

    #[derive(scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo,    ink::storage::traits::StorageLayout )
    )]
    pub struct TokenInfo {
        token_type: TokenType,
        token_address: AccountId,
    }


    #[derive(scale::Encode, scale::Decode, Clone, Default)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        PartialEq,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum TokenType {
        #[default]
        GovernanceToken,
        Psp22,
        Psp34,
    }



    #[derive(scale::Encode, scale::Decode, Clone, Default,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct MemberInfo {
        name: String,
        member_id: MemberId,
        member_status:MemberStatus,
        member_efficiency:u128,
        member_role:MemberRole,
        start_time:String,
        end_time:Option<String>,
        task_list:Vec<TaskId>,
        project_list:Vec<ProjectId>,
        periodicity:u8,
        achived_goals:u8,
        avalability:Option<u16>
    }


    #[derive(scale::Encode, scale::Decode, Clone, Default,PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum MemberStatus {
        Active,
        Inactive,
        Terminated,
        #[default]
        None

    }


    #[derive(scale::Encode, scale::Decode, Clone, Default,PartialEq )]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum MemberRole {
        Creator,
        Recruiter,
        Supporter,
        Auditor,
        Marketer,
        Seller,
        Advisor,
        #[default]
        None

    }


    #[derive(scale::Encode, scale::Decode, Clone,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        Eq,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct ProjectInfo {
        project_id:ProjectId,
        name:String,
        creator:AccountId,
        project_status:ProjectStatus,
        assigned_to:AccountId,
        start_time:String,
        end_time:Option<String>,
        description:String,
        task_list:Vec<TaskId>,
        sprint:Sprint
    }
    
    #[derive(scale::Encode, scale::Decode, Clone, Default,PartialEq,Eq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum ProjectStatus {
        Active,
        Inactive,
        Completed,
        Incompelte,
        #[default]
        None
    }

    #[derive(scale::Encode, scale::Decode, Clone, Default,Eq,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
       
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct Sprint {
        project_id: ProjectId,
        start_date:String, 
        end_date:String, 
        action:u8,    
    }

    #[derive(scale::Encode, scale::Decode, Clone,Eq,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
        
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]

    pub struct TicketInfo {
        ticket_id:TaskId,
        project_id:u16,
        name:String,
        creator:AccountId,
        ticket_description:String,
        ticket_status:TaskStatus,
        assigned_to:AccountId,
        task_type:TaskType,
        start_time:String,
        end_time:Option<String>,
        review:Option<ReviewStatus>,
        total_time_logged_in:Option<u16>,   
        confidence_assigned_to:u8
    }
      
    #[derive(scale::Encode, scale::Decode, Clone, Default,Eq,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
       
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum TaskStatus {
        ToDO,
        InProgress,
        ReadyToPR,
        BackToCw, 
        DevVerfied,
        LiveDeployed,
        Closed,
        Completed,
        #[default]
        None
    }

    #[derive(scale::Encode, scale::Decode, Clone, Default,Eq,PartialEq)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
     
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum TaskType {
        Bug,
        Feature,
        #[default]
        None
    }

    #[derive(scale::Encode, scale::Decode, Clone,  PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
      
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct ReviewStatus {
        pub records: ReviewRecord,
        
    }

    #[derive(scale::Encode, scale::Decode, Clone,  PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
      
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct ReviewRecord {
        pub who: AccountId,
        pub meta: Vec<u8>,
        pub option: ReviewOpinion,
    }


    #[derive(scale::Encode, scale::Decode, Clone, Default,  PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
      
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub enum ReviewOpinion {
        /// Agree.
        YES,
        /// Reject.
        NO,
        #[default]
        None
    }

    #[derive(scale::Encode, scale::Decode, Clone,    PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
    
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct Stake {
        staked_amount: Balance,
        deposit_time: String,
        release_time: Option<String>,
    }



    #[derive(scale::Encode, scale::Decode, Clone,    PartialEq,
        Eq,)]
    #[cfg_attr(
    feature = "std",
    derive(
        Debug,
    
        scale_info::TypeInfo,
        ink::storage::traits::StorageLayout    // <----- ! derive as so
        )
    )]
    pub struct StakeForDao {
        staked_amount: Balance,
        deposit_time: String,
        release_time: Option<String>,
        dao_id:DaoId
        
    }



    #[derive(scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ReadWriteErrorCode {
          InvalidKey,
          CannotWriteToKey,
          CannotReadFromKey,
    }


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// The Token Does Not Exists.
        TheTokenDoesNotExist,
        /// Invalid Operation.
        InvalidOperation,
        ///Not a Member
        NotAMember,
        /// Distribution is failure.
        DistributionIsFailure,
        /// Changing Token Status Is Failure.
        ChangingTokenStatusIsFailure,
        /// Withdrawing is Failure.
        WithdrawingIsFailure,
        /// Wrong Csv Data
        WrongCsvData,
        /// Tranfering Contract Balance is Failure
        TransferingContractBalanceIsFailure,
        /// Tranfering Contract Balance is Failure
        ThisFunctionCanBeCalledFromDaoManager,
         /// Not first member
         NotFirstMember,
         /// Target member does not exist.
         MemberDoesNotExist,
         /// Target member already exists.
         MemberAlreadyExists,
         /// Electoral Commissioner Data is mismatched.
         ElectoralCommissionerDataMismatch,
         /// Only Member does.
         OnlyMemberDoes,
         /// Only Electoral Commissioner
         OnlyElectoralCommissioner,
         /// Only Proposal Manager Address call this function.
         OnlyFromProposalManagerAddress,
         /// Csv Convert Failure
         CsvConvertFailure,
         /// Invalid Electoral Commissioner Count
         InvalidElectoralCommissionerCount,
         /// Invalid Delete Member Count
         InvalidDeleteMemberCount,
         /// At least one election commissioner
         AtLeastOneElectionCommissioner,
         /// Possible bug
         PossibleBug,
         ///Not a creator
         NotACreator,
         ///project does not exist
         ProjectDoesNotExist,
         ///Task does not exist
         TaskDoesNotExist,
         ///Not a Task Authority
         NotaTaskAuthority,
         ///Account does not exists
         AccountNotExist,
         ///Not Allowed
         NotAllowed,
         /// Only the owner can calim the refund
         InvalidRefundRequest, 
         /// Prevents multiple stake for same account, one person  one stake         
         AccountAlreadyExists, 
         ///deposits not sufficient
         DepositNotSufficient,
         ///self staking not allowed
         SelfStakingNotAllowed,
         ///can not reddem before the peroid 
         RedeemDurationNotReached,
         ///indufficient funds
         InsufficientContractBalance,
         ///transfer failed 
         TransferFailed,
         ///Not a Admin
         NotAAdmin,
         ///total ownership can not be 
         InvalidOwnershipPercentage,
         //add stake failed
         AddStakeFailed,
         ///Already staked
         AlreadyStaked,
         ///NotStaked
         NotStaked,
         ///DAO already added
         DaoAlreadyAdded,
         ///Dao does not exists
         DaoDoesNotExist,
         ///Cant log time more than 40 hrs on one task unless its authorised
         CanNotLogMoreTimeWithOutApproveMent,
         /// Ticket is closed
         TicketisClosed,
         ///Efficiency can not be calcilated by dividing to zero
         EfficiencyPeriodShouldNotBeZero,
         ///Ticket alreadyClosed
         TicketAlreadyClosed,
         //Dao Not Added
         DaoNotAdded,
         ///Dao NOT Staked
         DaoNotStaked

    }   

    //// events
    #[ink(event)]
    pub struct MemberAdded {
        dao_address:AccountId,
        member: AccountId,
        member_id: u16,
    }
    #[ink(event)]
    pub struct GovernanceTokenAdded {
        dao_address:AccountId,
        token_address: AccountId,
    }
    #[ink(event)]
    pub struct Transferred {
        from:Option<AccountId>,
        to: AccountId,
        value:Balance
    }
    #[ink(event)]
    pub struct MemberTerminated {
        dao_address:AccountId,
        member_address: AccountId,
        start_time:String,
        end_time:Option<String>,
    }

    #[ink(event)]
    pub struct MemberRoleUpdated {
        member_address: AccountId,
        new_role: MemberRole,
    }
    #[ink(event)]
    pub struct ProjectCreated {
        dao_address:AccountId,
        creator: AccountId,
        project_id: ProjectId,
        assigned_to:AccountId,
        start_time:String
    }
    #[ink(event)]
    pub struct ProjectStatusUpdated {
        project_id:ProjectId,
        status: ProjectStatus,
    }
    #[ink(event)]
    pub struct ProjectCompleted {
        project_id:ProjectId,
        start_time: String,
        end_time:Option<String>
    }

    #[ink(event)]
    pub struct TaskCreated {
        ticket_id:TaskId,
        project_id:ProjectId,
        creator:AccountId,
        assigned_to:AccountId,
        task_type:TaskType,
        start_time: String,
    }
   // Events to be propagated in response to some activities
   #[ink(event)]
   pub struct RedeemSuccessful {
       staker: AccountId,
       stake: Stake,
      
   }

   #[ink(event)]
   pub struct WithdrawSuccessful {
       staker: AccountId,
       stake: Stake,
    
   }

   #[ink(event)]
   pub struct DepositSuccessful {
       staker: AccountId,
       stake: Stake,
   }
      
   #[ink(event)]
   pub struct SprintAdded {
    project_id: ProjectId,
    start_date: String,
    end_date: String,
   }

   #[ink(event)]
   pub struct SprintUpdated {
    project_id: ProjectId,
    start_date: String,
    end_date: String,
   }

   

    impl FelideaDao {
        
        ///DAO's constructor 
       #[ink(constructor)]
       pub fn new( dao_name:String,website:Option<String>, profile:Option<String>,description:String,admin:AccountId) -> Self {

        // let admin = self.env().caller();
        // let total_balance = Self::env().balance();
        // let salt = version.to_le_bytes();
        // let app = DaoGovernanceTokenRef::new(None,None,6,1000000)
        //     .endowment(total_balance / 4)
        //     .code_hash(app_code_hash)
        //     .salt_bytes(salt)
        //     .instantiate();
        //    let r=Self::get_random_id();
           Self { 
           dao_id:[0;16],
           dao_admin:admin,
           dao_info: DaoInfo {
               dao_name: dao_name,
               description: description,
               website:website,
               profile:profile,
               dao_address:None,
               dao_admin:Some(admin),
               dao_id:Some(0)
           },
           token_list_for_address: Mapping::default(),
           next_member_id:0,
           dao_member_list: Mapping::default(),
           member_infoes_from_id: Mapping::default(),
           memberid_to_address:Mapping::default(),
           dao_project_list: Mapping::default(),
           next_project_id:0,
           dao_task_list: Mapping::default(),
           next_ticket_id:0,
           staking_data:Mapping::default(),
           dao_staking_data:Mapping::default(),
           total_stake: u128::default(),
           staking_duration: Timestamp::default(),
           dao_list:Mapping::default(),
           next_dao_id:0,
           dao_list_from_id:Mapping::default()
        //    token:app
       }
       }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(false ,0,"s".to_string(),None)
        // }

        /// make the dao owner the 100% owner of this dao 
        /// 
        #[ink(message)]
        pub fn add_stake_for_owner(&mut self) -> Result<()>{
            let get_admin = self.dao_admin.clone(); 
            if self.staking_data.contains(&get_admin){
                return Err(Error::AlreadyStaked);
            }   
            let dao_address = self.env().account_id();
            self.inline_add_member(dao_address, "admin".into(), get_admin);

            if self
            .dao_member_list
            .get(&(get_admin))
            == None 
            {
               //  if self
               //  .dao_list
               //  .get(&(caller))
               //  == None
               //  {
               //  ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
               //  return Err(Error::NotAMember);
               //  }
            ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
            return Err(Error::NotAMember);
            }
            let amount:Balance = 1; 
            self.total_stake = self.total_stake+amount;
            let time_now =  Self::env().block_timestamp();
            let mut stake = Stake{
                staked_amount:amount, 
                deposit_time:time_now.to_string().into(),
                release_time:None, 
            };
            self.staking_data.insert(&get_admin, &stake);
            Self::env().emit_event(DepositSuccessful {
                staker: get_admin,
                stake: stake,
            });

            Ok(()) 
        }



         #[ink(message)] 
         pub fn set_dao_id(&mut self) -> [u8;16] {
            let random_seed = self.get_random_id(); 
            // let value: u8 = u8::from_ne_bytes(random_seed);
            let mut result = [0; 16];
            for i in 0..16 {
                result[i] = random_seed[i];
            }
            self.dao_id = result ;
            self.dao_id
         }

         #[ink(message)]   
         pub fn get_dao_id(&self) -> [u8;16] {
           
            let random_seed = self.get_random_id(); 
            // let value: u8 = u8::from_ne_bytes(random_seed);
            let mut result = [0; 16];
            for i in 0..16 {
                result[i] = random_seed[i];
            }
            // self.dao_id = result ;
            self.dao_id
         }

        fn get_random_id(&self,) -> [u8;32] {
            // let random_seed = self.env().random(user_account.as_ref());
            let self_address = self.env().account_id();
            let random_seed = Self::env().code_hash(&self_address).unwrap() ;
            let mut seed_converted: [u8; 32] = Default::default();
            seed_converted.copy_from_slice(random_seed.as_ref());
            seed_converted
        }

         ///get_Dao_info    
         #[ink(message)]
         pub fn get_dao_info(&self) -> DaoInfo {
             self.dao_info.clone()
         }


          ///get_Dao_info    
          #[ink(message)]
          pub fn update_dao_info(&mut self,new_dao_info:DaoInfo){
              self.dao_info = new_dao_info
          }
         
         ///add_dao_token
         #[ink(message)]
         pub fn add_dao_token(
             &mut self,
             dao_address: AccountId,
             token_type: TokenType,
             token_address: AccountId,
           
         ) -> Result<()> {
             if !self._is_calling_from_dao_manager() {
                 return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
             }
 
             let token_info = TokenInfo {
                 token_type: token_type,
                 token_address: token_address,
             };
             self.token_list_for_address.insert(&token_address, &token_info.clone());
             Self::env().emit_event(GovernanceTokenAdded {
                 dao_address: dao_address,
                 token_address: token_address,
             });
             Ok(())
         }
         
         ///get_token_list
         #[ink(message)]
         pub fn get_token_list(&self) -> Vec<TokenInfo> {
             let mut result: Vec<TokenInfo> = Vec::new();
             let con =  self.env().account_id() ;
                 match self.token_list_for_address.get(&con) {
                     Some(value) => result.push(value),
                     None => (),
                 }
             result
         }
         
         ///get_admin
         #[ink(message)]
         pub fn get_admin(&self)->AccountId {
            self.dao_admin.clone()
         }
         
         /// check id admin
         #[ink(message)]
         pub fn is_admin(&self,admin:AccountId)->bool {
           if self.dao_admin ==admin{
             return true
           }else{
             return false
           }
         }
 
         ///get contract balance
         #[ink(message)]
         pub fn get_contract_balance(&self) -> Balance {
             self.env().balance()
         }
 

         #[inline]
         fn _is_calling_from_dao_manager(&self) -> bool {
             self.env().caller() == self.dao_admin
         }

         /// add  member.
        #[ink(message)]
        pub fn add_member(
            &mut self,
            dao_address: AccountId,
            member_address: AccountId,
            name: String,
        ) -> ResultTransaction<()> {
            let caller = self.env().caller();
            if caller != self.dao_admin {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }
            if self
            .dao_member_list
            .get(&(member_address))
            != None
        {
            ink::env::debug_println!("########## MEMBER EXISTS ALREADY Error.");
            return Err(Error::MemberAlreadyExists);
        }
            self.inline_add_member(dao_address, name, member_address);
            Ok(())
        }


        /// delete the member
        #[ink(message)]
        pub fn delete_member(&mut self, _dao_address: AccountId, member_address: AccountId, ) -> ResultTransaction<()> {
            if !self._is_calling_from_dao_manager() {
                return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            }
            self.inline_delete_member(_dao_address, member_address)
        }



        /// inline delete the member.
        #[inline]
        fn inline_delete_member(
        &mut self,
        dao_address: AccountId,
        member_address: AccountId,
        ) -> ResultTransaction<()> {
        let member_info = match self.dao_member_list.get(&(member_address)) {
        Some(value) => value,
        None => {
            ink::env::debug_println!("MemberDoesNotExist Error.");
            return Err(Error::MemberDoesNotExist);
        },
        };
        let next_member_id = member_info.member_id;
        self.member_infoes_from_id
        .remove(&next_member_id);
        self.dao_member_list.remove(&(member_address));

            Ok(())
        }


        // #[inline]
        // fn _is_calling_from_dao_manager(&self) -> bool {
        //     self.env().caller() == self.dao_admin
        // }
        
        
        /// add dao as a member 
        #[ink(message)]    
        pub fn add_dao_as_member(&mut self, admin:AccountId, dao_info:DaoInfo) -> Result<()> {

            match self.dao_list.get(&admin) {
                Some(data) => {
                    return Err(Error::DaoAlreadyAdded)
                },
                None => {
                    let mut dao_info = dao_info; 
                       dao_info.dao_id = Some(self.next_dao_id)   ;                  
                  _=   self.dao_list.insert(&admin,&dao_info)  ;  
                    _= self.dao_list_from_id.insert(&self.next_dao_id,&dao_info);
                    self.next_dao_id = self.next_dao_id +1;

                }
            }
            Ok(())
        }

        
        
        /// get_dao_list
        #[ink(message)]
        pub fn get_dao_list(&self) -> Vec<DaoInfo> {
            let mut dao_list:Vec<DaoInfo> = Vec::new();
            for i in 0..self.next_dao_id {
                let dao_info = match self.dao_list_from_id.get(&i) {
                    Some(value) => value,
                    None => continue,
                };
                dao_list.push(dao_info.clone());
            }
            dao_list
        }


        #[inline]
        fn inline_add_member(
            &mut self,
            dao_address: AccountId,
            name: String,
            member_address: AccountId,
        ) {
            
            //calculate the start time 
            let mut task_list:Vec<TaskId> =Vec::new(); 
            let mut project_list:Vec<ProjectId> =Vec::new(); 
            let time_now =  Self::env().block_timestamp();
            let member_info = MemberInfo {
                name: name,
                member_id: self.next_member_id,
                member_status:MemberStatus::Active,
                member_efficiency:0,
                member_role:MemberRole::None,
                start_time:time_now.to_string().into(),
                end_time:None,
                task_list,
                project_list,
                periodicity:0,
                achived_goals:0,
                avalability:Some(0)
            };

            self.member_infoes_from_id
            .insert(&self.next_member_id, &member_info.clone());
            self.memberid_to_address
            .insert(&self.next_member_id,&member_address);
            self.dao_member_list
                .insert(&member_address, &member_info.clone());
            self.next_member_id = self.next_member_id + 1;
           
            Self::env().emit_event(MemberAdded {
                dao_address: dao_address,
                member: member_address,
                member_id: self.next_member_id,
            });
        }



            /// terminate the member
            #[ink(message)]
            pub fn terminate_member(&mut self, _dao_address: AccountId, member_address: AccountId, ) -> ResultTransaction<()> {
                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
                self.inline_terminate_member(_dao_address, member_address)
            }

            /// inline terminate the member.
            #[inline]
            fn inline_terminate_member(
            &mut self,
            dao_address: AccountId,
            member_address: AccountId,
            ) -> ResultTransaction<()> {
            let mut member_info = match self.dao_member_list.get(&(member_address)) {
            Some(value) => value,
            None => {
                ink::env::debug_println!("MemberDoesNotExist Error.");
                return Err(Error::MemberDoesNotExist);
            },
            };
            let next_member_id = member_info.member_id;

            let mut member_info_ = match self.member_infoes_from_id.get(&next_member_id) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
                };
                
            self.dao_member_list
                .remove(&member_address);
            self.member_infoes_from_id
            .remove(&next_member_id);
            let time_now =  Self::env().block_timestamp();
            member_info.member_status = MemberStatus::Terminated;
            member_info.end_time = Some(time_now.to_string().into());


               self.dao_member_list
                .insert(&member_address, &member_info.clone());
               self.member_infoes_from_id
            .insert(&next_member_id, &member_info.clone());

            Self::env().emit_event(MemberTerminated {
                dao_address: dao_address,
                member_address: member_address,
                start_time: member_info.start_time,
                end_time: member_info.end_time,

            });
                Ok(())
            }

        #[inline]
        fn _convert_timestamp_to_date(&self, timestamp:Timestamp)  {
        let res =  Timestamp::from(timestamp);
        }

         /// get member info by id 
        #[ink(message)]
        pub fn get_member_info(&self,memberId:MemberId) -> ResultTransaction<MemberInfo>  {
            match self.member_infoes_from_id.get(&(memberId)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::MemberDoesNotExist)
                ,
            }
        }

     /// get member info 
     #[ink(message)]
     pub fn get_member_info_by_address(&self,member_address:AccountId) -> ResultTransaction<MemberInfo>  {
         match self.dao_member_list.get(&(member_address)) {
             Some(_value) => return  Ok(_value),
             None =>return Err(Error::MemberDoesNotExist)
             ,
         }
     }


        /// set_dao_admin 
        #[ink(message)]
        pub fn change_dao_admin(&mut self,member_address:AccountId) -> Result<()>  {
            let caller = self.env().caller();
            if caller != self.dao_admin {
                return Err(Error::NotAAdmin)
            }
            self.dao_admin = member_address;
            Ok(())
        }



        /// check the caller is the member of dao
        #[ink(message)]
        pub fn is_member(&self) -> bool {
            let caller = self.env().caller();
            match self.dao_member_list.get(&(caller)) {
                Some(_value) => true,
                None => false,
            }
        }
        

        #[inline]
        pub fn _is_member(&self, member_address:AccountId)->bool{
            match self.dao_member_list.get(&(member_address)) {
                Some(_value) => true,
                None => false,
            }
        }


        /// get member list.
        #[ink(message)]
        pub fn get_member_list(&self,) -> Vec<MemberInfo> {
            let mut member_list: Vec<MemberInfo> = Vec::new();
            for i in 0..self.next_member_id {
                let member_info = match self.member_infoes_from_id.get(&i) {
                    Some(value) => value,
                    None => continue,
                };
                member_list.push(member_info.clone());
            }
            member_list
        }
        


           /// update memmbers role
           #[ink(message)]
           pub fn update_member_role(&mut self,member_id:MemberId,role:MemberRole) -> Result<()> {
                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
             let mut member_info =  match self.member_infoes_from_id.get(&(member_id)) {
                   Some(_value) => _value,
                   None => return Err(Error::MemberDoesNotExist),
               };
               let next_member_id = member_info.member_id;
               let mut member_info_ = match self.member_infoes_from_id.get(&next_member_id) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
                };
               member_info.member_role = role;
               let member_address = match self.memberid_to_address.get(&member_id){
                Some(value) => value,
                None => {
                    ink::env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
                };

               self.dao_member_list
                .insert(&member_address, &member_info.clone());
                 self.member_infoes_from_id
                .insert(&next_member_id, &member_info.clone());
                Self::env().emit_event(MemberRoleUpdated {
                    member_address: member_address,
                    new_role: member_info.member_role,
                });
               Ok(())
           }

           ///get contract balance
           #[ink(message)]
           pub fn get_contract_address(&self) -> AccountId{

                self.env().account_id()
           }    

           
           /// create_project
           #[ink(message)]
           pub fn create_project(&mut self,name:String,dao_address: AccountId,assigned_to:AccountId,project_description:String) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(assigned_to) {
                    return Err(Error::NotAMember);
                }
                // let mut next_project_id = match self.next_project_ids.get(&dao_address) {
                //     Some(value) => value,
                //     None => 0,
                // };
                //calculate the start time 
                let mut task_list:Vec<TaskId> = Vec::new();
                let time_now =  Self::env().block_timestamp();
                // Calculate the timestamp of the seventh day after the original timestamp
                let seven_days_in_milliseconds = 7 * 24 * 60 * 60 * 1000;
                let timestamp_seventh_day = time_now + seven_days_in_milliseconds;
                let new_sprint = Sprint{
                    project_id:self.next_project_id, 
                    start_date:time_now.to_string().into(),
                    end_date:  timestamp_seventh_day.to_string().into() , 
                    action:0
                };
                let project_info = ProjectInfo {
                    name: name,
                    project_id:self.next_project_id,
                    creator:creator,
                    project_status:ProjectStatus::Active,
                    assigned_to:assigned_to,
                    start_time:time_now.to_string().into(),
                    end_time:None,
                    description:project_description,
                    task_list:task_list,
                    sprint:new_sprint
                };
                let mut  member_info = match self.dao_member_list.get(&(assigned_to)) {
                    Some(value) => value,
                    None => {
                        ink::env::debug_println!("MemberDoesNotExist Error.");
                        return Err(Error::MemberDoesNotExist);
                    },
                };
                member_info.project_list.push(self.next_project_id);
                // self.project_infoes_from_id
                // .insert(&(dao_address, self.next_project_id), &project_info.clone());
                self.dao_project_list
                .insert(&self.next_project_id, &project_info.clone());
              self.next_project_id = self.next_project_id + 1;
              self.dao_member_list.insert(&assigned_to,&member_info);
              self.member_infoes_from_id.insert(&member_info.member_id,&member_info);

                
                Self::env().emit_event(ProjectCreated {
                    dao_address: dao_address,
                    creator: creator,
                    project_id: project_info.project_id,
                    assigned_to:project_info.assigned_to,
                    start_time:project_info.start_time
                });
               Ok(())
           }
           
            /// get_project_info
           #[ink(message)]
           pub fn get_project_info(&self,project_id:ProjectId) -> ResultTransaction<ProjectInfo> {
    
            match self.dao_project_list.get(&(project_id)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::ProjectDoesNotExist)
                ,
            }
           }
           
            /// update project status
            #[ink(message)]
            pub fn update_project_status(&mut self,dao_address:AccountId,project_id:ProjectId,status:ProjectStatus) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }

            
            let project_id = project_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };
                if status == ProjectStatus::Completed {
                let time_now =  Self::env().block_timestamp();
                project_info.end_time = Some(time_now.to_string().into());
                }
                 project_info.project_status = status;

                 self.dao_project_list
                .insert(&project_id, &project_info.clone());
                //  self.project_infoes_from_id
                // .insert(&(dao_address, project_id), &project_info.clone());
                Self::env().emit_event(ProjectStatusUpdated {
                    project_id: project_id,
                    status:  project_info.project_status,
                });
                Ok(())

            }
            ///read runtime 
            #[ink(message)]
            pub fn read(&self, key:[u8; 32]) -> ExtensionResult<[u8; 32], RandomReadErr> {
                self.env()
                    .extension()
                    .fetch_random(key)
            }



            ///set_the_goal
            #[ink(message)]
            pub fn set_the_confidence(&mut self,ticket_id:TaskId,confidence_level:u8) -> Result<()> {
                let assigned_to = self.env().caller();

                let mut task_info =  match self.dao_task_list.get(&(ticket_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };

                if task_info.assigned_to!=assigned_to{
                    return Err(Error::NotACreator)
                }

                task_info.confidence_assigned_to = confidence_level;
                // project_info.sprint.action = confidence_level;

                self.dao_task_list
                    .insert(&ticket_id, &task_info.clone());
                Ok(())
            }

            /// close project
            #[ink(message)]
            pub fn close_project(&mut self,project_id:ProjectId) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = project_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };

            project_info.project_status = ProjectStatus::Completed;
            let time_now =  Self::env().block_timestamp();
            project_info.end_time = Some(time_now.to_string().into());

            self.dao_project_list
           .insert(&project_id, &project_info.clone());
        //     self.project_infoes_from_id
        //    .insert(&(dao_address, project_id), &project_info.clone());

            Self::env().emit_event(ProjectCompleted {
                project_id: project_id,
                start_time: project_info.start_time,
                end_time: project_info.end_time,
            });
                //TODO
            Ok(())

           }
              
        /// get project list.
        #[ink(message)]
        pub fn get_project_list(&self) -> Vec<ProjectInfo> {
            let mut project_list: Vec<ProjectInfo> = Vec::new();
            // let next_project_id = match self.next_project_ids.get(&dao_address) {
            //     Some(value) => value,
            //     None => return project_list,
            // };
            for i in 0..self.next_project_id {
                let project_info = match self.dao_project_list.get(&i) {
                    Some(value) => value,
                    None => continue,
                };
                project_list.push(project_info.clone());
            }
            project_list
        }   

        /// update sprint
         #[ink(message)]   
         pub fn update_sprint(&mut self, project_id:ProjectId, start_date:String, end_date:String) -> Result<()> {
            let creator = self.env().caller();
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };
            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = project_info.project_id.clone();
            // let end_date = project_info.sprint.end_date.clone();
            ///one day after 
            let one_day_in_milliseconds = 24 * 60 * 60 * 1000;
            let seven_days_in_milliseconds = 7 * 24 * 60 * 60 * 1000;
            // let number: u32 = u32::from_le_bytes(end_date.as_slice().try_into().expect("Invalid byte length"));
            // let s = Self::convert_bytes_to_number(end_date);
            // let integer_value: i32 = end_date.parse().unwrap();
            // let start_date = number + one_day_in_milliseconds; 
            // let new_end_date = start_date + seven_days_in_milliseconds ;
            // project_info.sprint.start_date = start_date.to_string().into() ;
            // project_info.sprint.end_date = new_end_date.to_string().into() ;
            project_info.sprint.start_date = start_date; 
            project_info.sprint.start_date = end_date; 
            self.dao_project_list
            .insert(&project_id, &project_info.clone());
            Self::env().emit_event(SprintUpdated {
                project_id: project_id,
                start_date: project_info.sprint.start_date,
                end_date: project_info.sprint.end_date,
            });
            Ok(())
         }

         
         pub fn convert_bytes_to_number(bytes: Vec<u8>) -> u32 {
            let bytes_array: [u8; 4] = bytes.as_slice().try_into().expect("Invalid byte length");
        
            let number: u32 = Decode::decode(&mut &bytes_array[..])
                .expect("Failed to decode bytes to number");
        
            number
        }


         /// update sprint
         #[ink(message)]   
         pub fn get_sprint(&self, project_id:ProjectId,) -> Result<Sprint> {
                  
            let creator = self.env().caller();
                
            let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if project_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = project_info.project_id;
            
            
            Ok(project_info.sprint.clone())
         }
         
           /// create_ticket
           #[ink(message)]
           pub fn create_ticket(&mut self,name:String,assigned_to:AccountId,ticket_type:TaskType,project_id:ProjectId,ticket_description:String) -> Result<()> {
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(assigned_to) {
                    return Err(Error::NotAMember);
                }
                let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::ProjectDoesNotExist),
                };
                // let mut next_ticket_id = match self.next_ticket_ids.get(&(dao_address,project_id)) {
                //     Some(value) => value,
                //     None => 0,
                // };
                
                //calculate the start time 
                let time_now =  Self::env().block_timestamp();
                let task_info = TicketInfo {
                    name: name,
                    ticket_id:self.next_ticket_id,
                    project_id:project_id,
                    creator:creator,
                    ticket_description:ticket_description,
                    ticket_status:TaskStatus::ToDO,
                    assigned_to:assigned_to,
                    start_time:time_now.to_string().into(),
                    task_type:ticket_type,
                    end_time:None,
                    review:None,
                    total_time_logged_in:None,
                    confidence_assigned_to:0
                };
                project_info.task_list.push(self.next_ticket_id);

                let mut  member_info = match self.dao_member_list.get(&(assigned_to)) {
                    Some(value) => value,
                    None => {
                        ink::env::debug_println!("MemberDoesNotExist Error.");
                        return Err(Error::MemberDoesNotExist);
                    },
                };
                member_info.task_list.push(self.next_ticket_id);
                self.dao_task_list
                .insert(&self.next_ticket_id, &task_info.clone());
                self.next_ticket_id = self.next_ticket_id + 1;
                self.dao_member_list.insert(&assigned_to,&member_info);
                self.member_infoes_from_id.insert(&member_info.member_id,&member_info);

                // self.next_ticket_ids.insert(&(dao_address,project_id), &next_ticket_id);
                //TODO
                // self.project_infoes_from_id
                // .insert(&(dao_address, project_info.project_id), &project_info.clone());
                self.dao_project_list
                .insert(&project_info.project_id, &project_info.clone());
            
                Self::env().emit_event(TaskCreated {
                    ticket_id: task_info.ticket_id,
                    project_id:task_info.project_id,
                    creator: task_info.creator,
                    assigned_to:task_info.assigned_to,
                    task_type:task_info.task_type,
                    start_time:task_info.start_time
                });
               Ok(())
           }

           /// get members ticket list 
           #[ink(message)]
           pub fn get_members_ticket_list(&self,member_id: MemberId) -> Result<Vec<TicketInfo>> {
            let mut task_list: Vec<TicketInfo> = Vec::new();

            let mut  member_info = match self.member_infoes_from_id.get(&(member_id)) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("MemberDoesNotExist Error.");
                    return Err(Error::MemberDoesNotExist);
                },
            };
            let member_task_list  = member_info.task_list.clone() ;
            let length = member_task_list.len(); 
            for i in 0..length {
                let index = member_task_list[i];
                let task_info = match self.dao_task_list.get(&index) {
                    Some(value) => value,
                    None => continue,
                };
                task_list.push(task_info.clone());
            } 
            Ok(task_list)
        }

        /// get members ticket list 
        #[ink(message)]
        pub fn get_members_project_list(&self,member_id: MemberId) -> Result<Vec<ProjectInfo>> {
        let mut project_list: Vec<ProjectInfo> = Vec::new();

        let mut  member_info = match self.member_infoes_from_id.get(&(member_id)) {
            Some(value) => value,
            None => {
                ink::env::debug_println!("MemberDoesNotExist Error.");
                return Err(Error::MemberDoesNotExist);
            },
        };
        let member_project_list  = member_info.project_list.clone() ;
        let length = member_project_list.len(); 
        for i in 0..length {
            let index = member_project_list[i];
            let project_info = match self.dao_project_list.get(&index) {
                Some(value) => value,
                None => continue,
            };
            project_list.push(project_info.clone());
          }  
        Ok(project_list)
    }
        
        /// get time logged data 
        #[ink(message)]
        pub fn get_time_logged_data_for_ticket(&self,ticket_id:TaskId) -> Result<Option<u16>> {
            let mut task_info = match self.dao_task_list.get(&(ticket_id)) {
                Some(_value) => _value,
                None => return Err(Error::TaskDoesNotExist),
            };

            Ok(task_info.total_time_logged_in.clone())
        }




           /// create_task
           #[ink(message)]
           pub fn create_review(&mut self,reviewer:AccountId,ticket_id:TaskId,project_id:ProjectId,review_discription:Vec<u8>,opinion:ReviewOpinion) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                if !self._is_member(reviewer) {
                    return Err(Error::NotAMember);
                }
                let mut project_info =  match self.dao_project_list.get(&(project_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::ProjectDoesNotExist),
                };
                
                let mut task_info = match self.dao_task_list.get(&(ticket_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };

                let  review = ReviewRecord{
                    who:reviewer,
                    meta:review_discription,
                    option:opinion
                };    

                let  review_status = ReviewStatus{
                    records:review
                };

                task_info.review = Some(review_status);

                
                self.dao_task_list
                .insert(&ticket_id, &task_info.clone());

                // self.next_ticket_ids.insert(&(dao_address,project_id), &ticket_id);
       
               Ok(())
           }


           /// update_ticket_status
           #[ink(message)]
           pub fn update_ticket_status(&mut self,ticket_id:TaskId,ticket_status:TaskStatus) -> Result<()> {
    
                let creator = self.env().caller();
                if !self._is_member(creator) {
                    return Err(Error::NotAMember);
                }
                
                //change the assigend_to's periodicity 

                let mut task_info = match self.dao_task_list.get(&(ticket_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };
                
                let member_address = task_info.assigned_to;

                let mut get_assigned_to_member_info =  match self.dao_member_list.get(&(member_address)) {
                    Some(_value) => _value,
                    None =>return Err(Error::MemberDoesNotExist)
                    ,
                };

                let mut periodicity_of_assigned_member= get_assigned_to_member_info.periodicity; 
                let mut achived_goals_of_assigned_member = get_assigned_to_member_info.achived_goals;

                if  ticket_status == TaskStatus::Closed {
                    if task_info.ticket_status == TaskStatus::Closed{
                        return Err(Error::TicketAlreadyClosed);
                    }
                    //// ERROR:: Periodicty should increase irrespectinve of the fact that ticket is closed or not
                    /// thats where the sprint or time tracking comes in between
                    /// think about the nil option 
                    if get_assigned_to_member_info.periodicity <4 && get_assigned_to_member_info.achived_goals <4 {
                        get_assigned_to_member_info.periodicity =   get_assigned_to_member_info.periodicity+1;
                        get_assigned_to_member_info.achived_goals = get_assigned_to_member_info.achived_goals+1;
                    }
                    
                    if get_assigned_to_member_info.periodicity == 4 && get_assigned_to_member_info.achived_goals ==4 {
                        get_assigned_to_member_info.periodicity =  0;
                        get_assigned_to_member_info.achived_goals = 0;
                    }
                    get_assigned_to_member_info.avalability = Some(0); 
                    self.dao_member_list
                    .insert(&member_address, &get_assigned_to_member_info.clone());
                    self.member_infoes_from_id
                    .insert(&get_assigned_to_member_info.member_id, &get_assigned_to_member_info.clone());
                    // Self::env().emit_event(MemberRoleUpdated {
                    //     member_address: member_address,
                    //     new_role: member_info.member_role,
                    // });
                    
                }                 


                

                if task_info.creator == creator || task_info.assigned_to == creator  {
                    if ticket_status == TaskStatus::Completed {
                        let time_now =  Self::env().block_timestamp();
                        task_info.end_time = Some(time_now.to_string().into());
                    }
                    task_info.ticket_status  = ticket_status;
                    let project_id = task_info.project_id;
    
                    
                    self.dao_task_list
                    .insert(&ticket_id, &task_info.clone());

                    
                    return Ok(()) ;

                }

                 return Err(Error::NotaTaskAuthority)

           }
           
            /// get_task_info
            #[ink(message)]
            pub fn get_ticket_info(&mut self,ticket_id:TaskId) -> ResultTransaction<TicketInfo> {

            match self.dao_task_list.get(&(ticket_id)) {
                Some(_value) => return  Ok(_value),
                None =>return Err(Error::TaskDoesNotExist)
                ,
            }
            }         


            /// close project
            #[ink(message)]
            pub fn close_ticket(&mut self,dao_address:AccountId,ticket_id:TaskId) -> Result<()> {
                
            let creator = self.env().caller();
                
            let mut task_info =  match self.dao_task_list.get(&(ticket_id)) {
                Some(_value) => _value,
                None => return Err(Error::ProjectDoesNotExist),
            };

            if task_info.creator!=creator{
            return Err(Error::NotACreator)
            }
            let project_id = task_info.project_id;
            let mut project_info_ = match self.dao_project_list.get(&(project_id)) {
                Some(value) => value,
                None => {
                    ink::env::debug_println!("ProjectDoesNotExist Error.");
                    return Err(Error::ProjectDoesNotExist);
                },
                };

            task_info.ticket_status = TaskStatus::Completed;
            let time_now =  Self::env().block_timestamp();
            task_info.end_time = Some(time_now.to_string().into());
                
            
            self.dao_task_list
                .insert(&ticket_id, &task_info.clone());                
                //TODO
            Ok(())
            }
             


            /// get project list.
            #[ink(message)]
            pub fn get_ticket_list(&self, ) -> Vec<TicketInfo> {
                let mut task_list: Vec<TicketInfo> = Vec::new();
                // let next_project_id = match self.next_ticket_ids.get(&(dao_address,project_id)) {
                //     Some(value) => value,
                //     None => return task_list,
                // };
                for i in 0..self.next_ticket_id {
                    let project_info = match self.dao_task_list.get(&(i)) {
                        Some(value) => value,
                        None => continue,
                    };
                    task_list.push(project_info.clone());
                }
                task_list
            }


            #[ink(message)]
            pub fn distribute_governance_token(&mut self,token_address: AccountId, to_address:AccountId, amount:u128) -> Result<()> {
                if !self._is_calling_from_dao_manager() {
                    return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
                }
                // let mut instance: DaoGovernanceTokenRef = ink::env::call::FromAccountId::from_account_id(token_address);
                // let mut instance = DaoGovernanceTokenRef::new(Some("name".as_bytes().to_vec()),Some("name".as_bytes().to_vec()),5,1000000) .endowment(total_balance / 4)
                // .code_hash(accumulator_code_hash)
                // .salt_bytes(salt)
                // .instantiate();
            //    match DaoGovernanceTokenRef::distribute_token(to_address ,100) {
            //            Ok(()) =>{ 
                           
            //             //    Self::env().emit_event(Transferred {
            //             //        from: None,
            //             //        to: to_address,
            //             //        value: amount,
            //             //    });
            //             ink::env::debug_println!("okayhu ");
       
            //                return Ok(())
                       
            //            },
            //            Err(_e) => return Err(Error::DistributionIsFailure),
            //        }
                Ok(())
            }


            ///send funds make transfer 
        //     #[ink(message, payable)]
        //    pub fn deposit(&mut self, to: AccountId)-> Result<()>{
        //         let balance = Self::env().transferred_value();
        //         Self::env().transfer(to, balance)?;
        //         Ok(())
        //     }

            // send funds only owner can access
            #[ink(message)]
            pub fn send_funds(&self,value:Balance) -> Result<()> {
                
                self.env()
                    .transfer(self.env().caller(), value)
                    .unwrap_or_else(|err| () );
                // panic!("transfer failed: {:?}", err)
                Ok(())
            }


            #[ink(message)]
            pub fn get_availability_efficiency_data(&mut self,member_address: AccountId) -> Result<Vec<(u16, u16)>>  {
                self.get_time_logged_data(member_address)
            }

           

            // #[ink(message)]
            // pub fn get_quality(&mut self) {

            // }


            //   #[inline]
            // pub fn distribute_reward(&mut self, token_address: AccountId, to_address:AccountId, amount:u128) -> Result<()> {
                    
            //     let creator = self.env().account_id();

            //     if !self._is_calling_from_dao_manager() {
            //         return Err(Error::ThisFunctionCanBeCalledFromDaoManager);
            //     }
    
            //     let token_info: TokenInfo = match self.token_list_for_address.get(&token_address) {
            //         Some(value) => value,
            //         None => return Err(Error::TheTokenDoesNotExist),
            //     };
                    
            //     let mut instance: DaoGovernanceTokenRef = ink::env::call::FromAccountId::from_account_id(token_address);
                
            //     match instance.distribute_token(to_address ,amount) {
            //         Ok(()) => return Ok(()),
            //         Err(_e) => return Err(Error::DistributionIsFailure),
            //     }
                
            // }   
                 

            /// record the time.
            #[ink(message)]
            pub fn log_time(&mut self,ticket_id:TaskId,hours:u16, minutes:u16) -> Result<()> {
                let caller = self.env().caller();
                let mut task_info = match self.dao_task_list.get(&(ticket_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };
                if task_info.ticket_status == TaskStatus::Closed {
                    return Err(Error::TicketisClosed)
                }
                if  task_info.assigned_to != caller {
                    return Err(Error::NotaTaskAuthority);
                }


                let total_minutes = task_info.total_time_logged_in.unwrap_or(0) + (hours * 60) + minutes;
                let total_hours = total_minutes / 60;
                  if total_minutes > 2400 {
                   return Err(Error::CanNotLogMoreTimeWithOutApproveMent);
             }
                // task_info.total_time_logged_in = Some(total_hours);
                // task_info.total_time_logged_in = Some(total_hours);
                            task_info.total_time_logged_in  = match task_info.total_time_logged_in{
                                    Some(data) =>{
                                        if data > 40 || data+total_hours > 40 {
                                      return Err(Error::CanNotLogMoreTimeWithOutApproveMent);
                                        }
                                        Some(data+total_minutes)
                                    },
                                    None =>{
                                        Some(0+total_minutes)
                                    }
                            };

                            let member_address = task_info.assigned_to;

                            let mut get_assigned_to_member_info =  match self.dao_member_list.get(&(member_address)) {
                                Some(_value) => _value,
                                None =>return Err(Error::MemberDoesNotExist)
                                ,
                            };

                            get_assigned_to_member_info.avalability = task_info.total_time_logged_in;
                            self.dao_member_list
                            .insert(&member_address, &get_assigned_to_member_info.clone());
                            self.member_infoes_from_id
                            .insert(&get_assigned_to_member_info.member_id, &get_assigned_to_member_info.clone());    

                self.dao_task_list
                .insert(&ticket_id, &task_info.clone());

                Ok(())

            }                
            

            /// record the time.
            #[ink(message)]
            pub fn log_extra_time(&mut self,ticket_id:TaskId,hours:u16, minutes:u16) -> Result<()> {
                let caller = self.env().caller();
                let mut task_info = match self.dao_task_list.get(&(ticket_id)) {
                    Some(_value) => _value,
                    None => return Err(Error::TaskDoesNotExist),
                };
                if task_info.ticket_status == TaskStatus::Closed {
                    return Err(Error::TicketisClosed)
                }
                ///only admin can approve extra time log 
                if  self.dao_admin != caller {
                    return Err(Error::NotAAdmin);
                }
                let total_minutes = task_info.total_time_logged_in.unwrap_or(0) + (hours * 60) + minutes;

                task_info.total_time_logged_in  = match task_info.total_time_logged_in{
                        Some(data) =>{
                        
                            Some(data+total_minutes)
                        },
                        None =>{
                            Some(0+total_minutes)
                        }
                };

                
                self.dao_task_list
                .insert(&ticket_id, &task_info.clone());

                Ok(())

            }                
            

            #[inline]
            pub fn get_time_logged_data(&self, member_address:AccountId) -> Result<Vec<(u16, u16)>> {
                //TODO

                //get member 
                let member = match self.dao_member_list.get(&member_address) {
                     Some(mem) =>  mem , 
                     None => return Err(Error::NotAMember),
                };

                // (id,time)
                let mut task_data :Vec<(u16, u16)> = Vec::new() ;
                let task_list = member.task_list; 
                for i in &task_list {

                    let mut tasks = match  self.dao_task_list.get(&i) { 
                        Some(task) => task, 
                        None => continue,
                    };
                    let time_logged_in = match tasks.total_time_logged_in{
                        Some(time ) => time, 
                        None => 0,
                    }; 
                    task_data.push((*i, time_logged_in)) ; 

                }
               Ok(task_data)
                
            }   



            // #[ink(message)]
            // pub fn set_stake_data(&mut self, interest_rate: u8, staking_duration:Timestamp) -> Result<()> {
            //         let caller = self.env().caller();
            //         if caller != self.dao_admin {
            //             return Err(Error::NotAAdmin)
            //         } 
            //         self.total_stake = interest_rate;
            //         self.staking_duration = staking_duration;
            //    Ok(())
                
            // }  

             ///get stake for account 
             #[ink(message)]
             pub fn get_stake_for_account(&self, account_id: AccountId) -> Option<Stake> {
                 self.staking_data.get(&account_id)
             }
     


             ///early withdraw
             #[ink(message)]
             pub fn early_withdraw(&mut self) -> Result<Stake> {
                 // Early withdraw don't get any interest
                 let account = self.env().caller();
                 let stake = self.get_account_if_exists(&account)?;
                 let total_amount = stake.staked_amount;
     
                 self.transfer_balance(account.clone(), total_amount)?;
                 self.env().emit_event(WithdrawSuccessful {
                     staker: account,
                     stake: stake.clone(),
                 });
     
     
                 self.staking_data.remove(&account);
                 Ok(stake)
             }
     
             
             pub fn check_not_self(&self, account: &AccountId) -> bool {
                self.env().account_id() != *account
            }
            pub fn get_timestamp(&self) -> Timestamp {
                ink::env::block_timestamp::<ink::env::DefaultEnvironment>()
            }
    
            /// For testing purpose
            #[ink(message)]
            pub fn read_timestamp(&self) -> Option<Timestamp> {
                Some(ink::env::block_timestamp::<ink::env::DefaultEnvironment>())
            }
            
            pub fn get_account_if_exists(&self, account: &AccountId) -> Result<Stake> {
                if let Some(lock) = self.staking_data.get(account) {
                    Ok(lock)
                } else {
                    Err(Error::AccountNotExist)
                }
            }
    
            pub fn check_sufficient_balance(&self, amount: Balance) -> Result<()> {
                if self.env().balance() < amount {
                    Err(Error::InsufficientContractBalance)
                } else {
                    Ok(())
                }
            }

            // #[ink(message, payable)]
            // fn send_funds(&mut self, to: AccountId)-> Result<(), ink_env::Error>{
            //     let balance = Self::env().transferred_value();
            //     Self::env().transfer(to, balance)?;
            //     Ok(())
            // }
            
            #[ink(message)]
            pub  fn transfer_balance(
                &mut self,
                account: AccountId,
                balance: Balance,
            ) -> Result<()> {
                self.check_sufficient_balance(balance)?;
                // contract => account 
                // account => contract
                // contract =>  PAN 
                // Self::env().transfer(to, amount)
                // if let Err(_) = self.env().transfer(account, balance) {
                //     Err(Error::TransferFailed)
                // } else {
                //     Ok(())
                // }
                match self.env().transfer(account, balance) {
                    Err(_) => return Err(Error::TransferFailed),
                    Ok(_) => return Ok(())
                }
                Ok(())
            }

            // account1 => account2 
            // account1 => contract => account2

             /// transfer_balance_to_contract
             #[ink(message)]
             #[ink(payable)]
             pub fn transfer_balance_to_contract(
                 &mut self,
             ) -> Result<()> {
                 let amount = Self::env().transferred_value();   
                 ink::env::debug_println!(" amount {:#?}",amount);
     
                 Ok(())      
             }
                  

             // someone =>  contravt 
             // reddem => contract 

             // add_stake => transfer_balance_to_contract => redeem_stake
             // on fe  => add_stake => get+staking+data => then transfer to admin owner or contract and while redeeming transfer back it to user  from the entity 
             // for that reasearch on how balance transfer happns in polka ja 
             ///add stake 
             /// 
             #[ink(message)]
             pub fn add_stake(
                 &mut self,
                 amount:Balance
             ) -> Result<()> {
                 let caller = self.env().caller();
     
                 if self.staking_data.contains(&caller){
                     return Err(Error::AlreadyStaked);
                 }     
                 if self
                 .dao_member_list
                 .get(&(caller))
                 == None 
                 {
                    //  if self
                    //  .dao_list
                    //  .get(&(caller))
                    //  == None
                    //  {
                    //  ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                    //  return Err(Error::NotAMember);
                    //  }
                 ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                 return Err(Error::NotAMember);
                 }
                 self.total_stake = self.total_stake+amount;
                 let time_now =  Self::env().block_timestamp();
                 let mut stake = Stake{
                     staked_amount:amount, 
                     deposit_time:time_now.to_string().into(),
                     release_time:None, 
                 };
                 
                 self.staking_data.insert(&caller, &stake);
                 Self::env().emit_event(DepositSuccessful {
                     staker: caller,
                     stake: stake,
                 });
                 Ok(())      
             }

             #[ink(message)]
             pub fn get_dao_by_id(&self,dao_id:DaoId) -> ResultTransaction<DaoInfo>{
                match self.dao_list_from_id.get(&(dao_id)) {
                    Some(_value) => return  Ok(_value),
                    None =>return Err(Error::DaoDoesNotExist)
                    ,
                }
             }
             
            #[ink(message)]
            pub fn get_staking_data_of_member(&self) -> Result<Stake> {
                let caller = self.env().caller();
     
                if self.staking_data.contains(&caller){
                    return Err(Error::NotStaked);
                }   

                match self.staking_data.get(&caller){
                    Some(_value) => return Ok(_value),
                    None => return Err(Error::NotStaked)
                }
            }

            /// transfer to contract from the dao's admin of the dao 
             #[ink(message)]
             pub fn add_stake_for_dao(
                &mut self,
                amount:Balance
             ) -> Result<()> {
                let caller = self.env().caller();
                let dao_info = match self.dao_list.get(&caller){
                    Some(data) => {
                        data
                    },
                    None =>{
                        return Err(Error::DaoNotAdded)
                    },
                };

                // if self.dao_list.contains(&caller){
                //     return Err(Error::DaoNotAdded)
                // }
                let dao_id = match dao_info.dao_id  {
                    Some(data) => {
                        data
                    },
                    None => {
                        return Err(Error::DaoDoesNotExist)
                    }
                };

                if self.dao_staking_data.contains(&dao_id){
                    return Err(Error::AlreadyStaked);
                }   

                self.total_stake = self.total_stake+amount;
                let time_now =  Self::env().block_timestamp();
                let mut stake = StakeForDao{
                    staked_amount:amount, 
                    deposit_time:time_now.to_string().into(),
                    release_time:None, 
                    dao_id:dao_id.clone()
                };
                
                self.dao_staking_data.insert(&dao_id, &stake);
                // Self::env().emit_event(DepositSuccessful {
                //     staker: caller,
                //     stake: stake,
                // });
                Ok(())   
             }

             /// get_daos who staked
             #[ink(message)]
             pub fn get_staked_dao_list(&self) -> Vec<StakeForDao> {
                let mut dao_list:Vec<StakeForDao> = Vec::new();
                for i in 0..self.next_dao_id {
                    let dao_info = match self.dao_staking_data.get(&i) {
                        Some(value) => value,
                        None => continue,
                    };
                    dao_list.push(dao_info.clone());
                }
                dao_list
             }
             
             /// reddem dao stake
             /// dao's  admin should call it 
             #[ink(message)]
             pub fn redeem_dao_stake(
                &mut self
             ) -> Result<()>{

                let admin = self.env().caller();
                let dao_info = match self.dao_list.get(&admin){
                    Some(data) => {
                        data
                    },
                    None =>{
                        return Err(Error::DaoNotAdded)
                    },
                };

                // if self.dao_list.contains(&caller){
                //     return Err(Error::DaoNotAdded)
                // }
                let dao_id = match dao_info.dao_id  {
                    Some(data) => {
                        data
                    },
                    None => {
                        return Err(Error::DaoDoesNotExist)
                    }
                };

                if self.dao_staking_data.contains(&dao_id) == false{
                    return Err(Error::DaoNotStaked);
                }  

                let mut your_stake = match self.dao_staking_data.get(&(dao_id)) {
                    Some(value) => value,
                    None => {
                        ink::env::debug_println!("MemberDoesNotExist Error.");
                        return Err(Error::DaoNotAdded);
                   },
                 };
                  let staked_amount = your_stake.staked_amount; 
                  self.transfer_balance(admin.clone(), staked_amount)?;
                  let time_now =  Self::env().block_timestamp();
                  your_stake.release_time=Some(time_now.to_string().into());
                  self.total_stake = self.total_stake-staked_amount ;
                  self.dao_staking_data.remove(&dao_id);


                Ok(())

             }


               ///reddem stake
               #[ink(message)]
               pub fn redeem_stake(
                   &mut self,
               ) -> Result<()> {
                   let caller = self.env().caller();
       
                   if self.staking_data.contains(&caller) ==false{
                       return Err(Error::NotStaked);
                   }     
                   if self
                   .dao_member_list
                   .get(&(caller))
                   == None
                   {
                    //  if self
                    //  .dao_list
                    //  .get(&(caller))
                    //  == None
                    //  {
                    //  ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                    //  return Err(Error::NotAMember);
                    //  }
                   ink::env::debug_println!("########## MEMBER DOES NOT EXISTS .");
                   return Err(Error::NotAMember);
                   }
                  
                   let mut your_stake = match self.staking_data.get(&(caller)) {
                     Some(value) => value,
                     None => {
                         ink::env::debug_println!("MemberDoesNotExist Error.");
                         return Err(Error::MemberDoesNotExist);
                    },
                  };
                   let staked_amount = your_stake.staked_amount; 
                   self.transfer_balance(caller.clone(), staked_amount)?;
                   let time_now =  Self::env().block_timestamp();
                   your_stake.release_time=Some(time_now.to_string().into());
                   self.total_stake = self.total_stake-staked_amount ;
                   self.staking_data.remove(&caller);
                   Self::env().emit_event(RedeemSuccessful {
                     staker: caller,
                     stake: your_stake,
                 });
                   Ok(())      
               }



               #[ink(message)]
               pub fn get_ownership(&self) -> Result<String>  {
                   let caller = self.env().caller();
                   let mut your_stake =  match self.staking_data.get(&(caller)) {
                       Some(_value) =>  _value,
                       None =>return Err(Error::NotStaked)
                       ,
                   };
                //    let staked_amount = your_stake.staked_amount/1000000000000;    // u128 
                //    let a = Fix::from_num(staked_amount as u8);  // 
                //    let b = Fix::from_num(self.total_stake as u8); 
                //    let res = a / b; 
                //    let res_str =res*100; 
                    let staked_amount = your_stake.staked_amount;
                    let a = Fix::from_num(staked_amount);
                    let b = Fix::from_num(self.total_stake as u128);
                    let res = a / b;
                    let res_str = res * Fix::from_num(100);
                   return Ok(res_str.to_string().into()) ;
               }
                 
               #[ink(message)]  
               pub fn get_dao_ownership(&self)-> Result<String> {
                let admin = self.env().caller();
                let dao_info = match self.dao_list.get(&admin){
                    Some(data) => {
                        data
                    },
                    None =>{
                        return Err(Error::DaoNotAdded)
                    },
                };

              

                let dao_id = match dao_info.dao_id  {
                    Some(data) => {
                        data
                    },
                    None => {
                        return Err(Error::DaoDoesNotExist)
                    }
                };


                let mut dao_stake =  match self.dao_staking_data.get(&(dao_id)) {
                    Some(_value) =>  _value,
                    None =>return Err(Error::NotStaked)
                    ,
                };

                let staked_amount = dao_stake.staked_amount;
                let a = Fix::from_num(staked_amount);
                let b = Fix::from_num(self.total_stake as u128);
                let res = a / b;
                let res_str = res * Fix::from_num(100);
               return Ok(res_str.to_string().into()) ;

               }
                
                 
            //    #[ink(message)]
            //    pub fn convert_accountid_to_string2(&self, account_id: AccountId) -> String {
            //        // convert to &[u8;32] first
            //        let account_id: &[u8;32] = account_id.as_ref();
            //        let account_id = H256::from(account_id);
       
            //        // Convert the AccountId to an SS58 encoded string
            //        let account_id_string = account_id.to_ss58check();
            //        account_id_string
               
            //    }


            #[ink(message)]
            pub fn calculate_ownership_test(&mut self,   amount_:u128,
                )  {
                    self.total_stake = self.total_stake+amount_ ;
                    ink::env::debug_println!("self.total_stake Error.{:?}", self.total_stake);
                    let a = Fix::from_num(amount_); 
                    let b = Fix::from_num(self.total_stake); 
                    let res = a / b; 
                    let res_str =res*100; 
                    // let s:String = res_str.to_string().into();
                    // ink::env::debug_println!("MemberDoesNotExist Error.{:?}",s);
            }

                   #[inline]
                    fn calculate_ownership(&self,   a:u8,
                       b:u8,) -> String{
                           let a = Fix::from_num(a); 
                           let b = Fix::from_num(b); 
                           let res = a / b; 
                           let res_str =res*100; 
                           let s = res_str.to_string();
                        //    ink::env::debug_println!("MemberDoesNotExist Error.{:?}",s);
                           s.into()
                   }

                   

                #[ink(message)]
                pub fn check_contract_balance(
                     &self,
                ) -> Result<(Balance)> {
                    
                    
                    Ok(self.env().balance() )   
                }


                //Default periodicity 4 
                #[ink(message)]
                pub fn efficiency_percent(&self, member_address:AccountId) -> Result<u8> {
                    let mut achieved_goals_count = 0;
                    // let mut adjusted_periodicity = periodicity;
        
                    // for goal in achieved_goals.iter() {
                    //     match *goal {
                    //         "Yes" => achieved_goals_count += 1,
                    //         "No" => continue,
                    //         "Nil" => adjusted_periodicity -= 1,
                    //         _ => ink_env::panic!("Invalid Input"),
                    //     }
                    // }

                // at the end of the week periodicty should increase by 1     
                let mut get_assigned_to_member_info =  match self.dao_member_list.get(&(member_address)) {
                    Some(_value) => _value,
                    None =>return Err(Error::MemberDoesNotExist)
                    ,
                };


                let mut periodicity_of_assigned_member= get_assigned_to_member_info.periodicity; 
                let mut achived_goals_of_assigned_member = get_assigned_to_member_info.achived_goals;

        
                    if periodicity_of_assigned_member == 0 {
                        return Err(Error::EfficiencyPeriodShouldNotBeZero); // Avoid division by zero
                    }
        
                   let efficiency =  (achived_goals_of_assigned_member * 100) / periodicity_of_assigned_member;
                   Ok(efficiency)

                }

                
                #[ink(message)]
                pub fn get_dao_info_by_admin(&self,admin:AccountId) ->ResultTransaction<DaoInfo> {
                    match self.dao_list.get(&(admin)) {
                        Some(_value) => return  Ok(_value),
                        None =>return Err(Error::DaoNotAdded)
                        ,
                    }
                }

                /// Modifies the code which is used to execute calls to this contract address (`AccountId`).
                ///
                /// We use this to upgrade the contract logic. We don't do any authorization here, any caller
                /// can execute this method. In a production contract you would do some authorization here.
                #[ink(message)]
                pub fn set_code(&mut self, code_hash: [u8; 32]) {
                    ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                        panic!(
                            "Failed to `set_code_hash` to {:?} due to {:?}",
                            code_hash, err
                        )
                    });
                    ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
                }

                #[ink(message)]
                pub fn get_contract_ownership(&self, ) -> Result<String> {
                    let contract = self.env().account_id() ;
                   let mut your_stake =  match self.staking_data.get(&(contract)) {
                       Some(_value) =>  _value,
                       None =>return Err(Error::NotStaked)
                       ,
                   };
                //    let staked_amount = your_stake.staked_amount/1000000000000;    // u128 
                //    let a = Fix::from_num(staked_amount as u8);  // 
                //    let b = Fix::from_num(self.total_stake as u8); 
                //    let res = a / b; 
                //    let res_str =res*100; 
                    let staked_amount = your_stake.staked_amount;
                    let a = Fix::from_num(staked_amount);
                    let b = Fix::from_num(self.total_stake as u128);
                    let res = a / b;
                    let res_str = res * Fix::from_num(100);
                   return Ok(res_str.to_string().into()) ;
                }  

                





    }
    
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        // We test if the default constructor does its job.
        // #[ink::test]
        // fn default_works() {
        //     let felideaDAO = FelideaDao::default();
        //     assert_eq!(felideaDAO.get(), false);
        // }

        // /// We test a simple use case of our contract.
        // #[ink::test]
        // fn it_works() {
        //     let mut felideaDAO = FelideaDao::new(false);
        //     assert_eq!(felideaDAO.get(), false);
        //     felideaDAO.flip();
        //     assert_eq!(felideaDAO.get(), true);
        // }
    }



    

    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FelideaDaoRef::default();

            // When
            let contract_account_id = client
                .instantiate("felideaDAO", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<FelideaDaoRef>(contract_account_id.clone())
                .call(|felideaDAO| felideaDAO.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = FelideaDaoRef::new(false);
            let contract_account_id = client
                .instantiate("felideaDAO", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<FelideaDaoRef>(contract_account_id.clone())
                .call(|felideaDAO| felideaDAO.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<FelideaDaoRef>(contract_account_id.clone())
                .call(|felideaDAO| felideaDAO.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<FelideaDaoRef>(contract_account_id.clone())
                .call(|felideaDAO| felideaDAO.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }



    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let rand_extension = RandExtension::new_default();
            assert_eq!(rand_extension.get(), [0; 32]);
        }

        #[ink::test]
        fn chain_extension_works() {
            // given
            struct MockedExtension;
            impl ink::env::test::ChainExtension for MockedExtension {
                /// The static function id of the chain extension.
                fn func_id(&self) -> u32 {
                    1101
                }

                /// The chain extension is called with the given input.
                ///
                /// Returns an error code and may fill the `output` buffer with a
                /// SCALE encoded result. The error code is taken from the
                /// `ink::env::chain_extension::FromStatusCode` implementation for
                /// `RandomReadErr`.
                fn call(&mut self, _input: &[u8], output: &mut Vec<u8>) -> u32 {
                    let ret: [u8; 32] = [1; 32];
                    scale::Encode::encode_to(&ret, output);
                    0
                }
            }
            ink::env::test::register_chain_extension(MockedExtension);
            let mut rand_extension = RandExtension::new_default();
            assert_eq!(rand_extension.get(), [0; 32]);

            // when
            rand_extension.update([0_u8; 32]).expect("update must work");

            // then
            assert_eq!(rand_extension.get(), [1; 32]);
        }
    }
    
}
