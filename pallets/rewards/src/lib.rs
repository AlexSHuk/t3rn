#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    t3rn_primitives::reexport_currency_types!();
    use codec::Encode;
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Currency};
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{CheckedAdd, CheckedDiv, Saturating, Zero},
        PerThing, Perbill, Percent,
    };
    use sp_std::{collections::btree_map::BTreeMap, convert::TryInto, prelude::*};
    use t3rn_primitives::{
        account_manager::{AccountManager, Settlement},
        attesters::AttestersReadApi,
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        clock::Clock as ClockTrait,
        TreasuryAccount, TreasuryAccountProvider,
    };

    #[derive(Clone, Encode, Decode, PartialEq, Eq, Debug, TypeInfo)]
    pub struct DistributionRecord<BlockNumber, Balance> {
        pub block_number: BlockNumber,
        pub attester_rewards: Balance,
        pub collator_rewards: Balance,
        pub executor_rewards: Balance,
        pub treasury_rewards: Balance,
        pub available: Balance,
        pub distributed: Balance,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;

        type TreasuryAccounts: TreasuryAccountProvider<Self::AccountId>;
        /// The total inflation per year, expressed as a Perbill.
        ///
        /// Default: 4.4% (44_000_000 / 1_000_000_000)
        #[pallet::constant]
        type TotalInflation: Get<Perbill>;

        /// The attester's portion of the total inflation, expressed as a Perbill.
        ///
        /// Default: 1.1% (11 / 100)
        #[pallet::constant]
        type AttesterInflation: Get<Perbill>;

        /// The executor's portion of the total inflation, expressed as a Perbill.
        ///
        /// Default: 0.8% (8 / 100)
        #[pallet::constant]
        type ExecutorInflation: Get<Perbill>;

        /// The collator's portion of the total inflation, expressed as a Perbill.
        ///
        /// Default: 0.5% (5 / 100)
        #[pallet::constant]
        type CollatorInflation: Get<Perbill>;

        /// The treasury's portion of the total inflation, expressed as a Perbill.
        ///
        /// Default: 2% (20 / 100)
        #[pallet::constant]
        type TreasuryInflation: Get<Perbill>;

        /// The number of blocks in one year.
        ///
        /// Default: 2_628_000 (assuming 12s block time)
        #[pallet::constant]
        type OneYear: Get<Self::BlockNumber>;

        /// The number of blocks between inflation distribution.
        ///
        /// Default: 100_800 (assuming one distribution per two weeks)
        #[pallet::constant]
        type InflationDistributionPeriod: Get<Self::BlockNumber>;

        type AvailableBootstrapSpenditure: Get<BalanceOf<Self>>;

        type AttesterBootstrapRewards: Get<Percent>;

        type CollatorBootstrapRewards: Get<Percent>;

        type ExecutorBootstrapRewards: Get<Percent>;

        type Clock: ClockTrait<Self>;

        type AccountManager: AccountManager<
            Self::AccountId,
            BalanceOf<Self>,
            Self::Hash,
            Self::BlockNumber,
            u32,
        >;

        type Attesters: AttestersReadApi<Self::AccountId, BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Attesters<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    pub type Collators<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32>;

    #[pallet::storage]
    #[pallet::getter(fn get_accumulated_settlements)]
    pub type AccumulatedSettlements<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

    #[pallet::storage]
    pub type DistributionBlock<T: Config> = StorageValue<_, T::BlockNumber>;

    #[pallet::storage]
    pub type DistributionHistory<T: Config> =
        StorageValue<_, Vec<DistributionRecord<T::BlockNumber, BalanceOf<T>>>, ValueQuery>;

    #[pallet::storage]
    pub type IsDistributionHalted<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub type IsSettlementAccumulationHalted<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub type IsClaimingHalted<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_pending_claims)]
    pub type PendingClaims<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        Vec<ClaimableArtifacts<T::AccountId, BalanceOf<T>>>,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AttesterRewarded(T::AccountId, BalanceOf<T>),
        CollatorRewarded(T::AccountId, BalanceOf<T>),
        ExecutorRewarded(T::AccountId, BalanceOf<T>),
        Claimed(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        DistributionPeriodNotElapsed,
        NoPendingClaims,
        ArithmeticOverflow,
        AttesterNotFound,
        TryIntoConversionU128ToBalanceFailed,
        Halted,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn trigger_distribution(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            Self::distribute_inflation();
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn turn_on_off_distribution(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            let is_distribution_on = IsDistributionHalted::<T>::get();
            IsDistributionHalted::<T>::put(!is_distribution_on);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn turn_on_off_claims(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            let is_claiming_on = IsClaimingHalted::<T>::get();
            IsClaimingHalted::<T>::put(!is_claiming_on);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn turn_on_off_settlement_accumulation(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            let is_settlement_accumulation_on = IsSettlementAccumulationHalted::<T>::get();
            IsSettlementAccumulationHalted::<T>::put(!is_settlement_accumulation_on);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn claim(
            origin: OriginFor<T>,
            role_to_claim: Option<CircuitRole>, // Add this parameter
        ) -> DispatchResultWithPostInfo {
            // ensure claiming is not halted
            ensure!(!IsClaimingHalted::<T>::get(), Error::<T>::Halted);

            let who = ensure_signed(origin)?;

            PendingClaims::<T>::try_mutate(who.clone(), |maybe_pending_claims| {
                let pending_claims = maybe_pending_claims
                    .take()
                    .ok_or(Error::<T>::NoPendingClaims)?;

                // Filter by the specified role if provided
                let claims_to_process = match role_to_claim {
                    Some(role) => pending_claims
                        .clone()
                        .into_iter()
                        .filter(|claim| claim.role == role)
                        .collect::<Vec<_>>(),
                    None => pending_claims.clone(),
                };

                let mut total_claim = BalanceOf::<T>::zero();
                for claim in &claims_to_process {
                    total_claim = total_claim
                        .checked_add(&claim.total_round_claim)
                        .ok_or(Error::<T>::ArithmeticOverflow)?;
                }

                ensure!(
                    total_claim > BalanceOf::<T>::zero(),
                    Error::<T>::NoPendingClaims
                );

                T::Currency::deposit_into_existing(&who, total_claim)?;

                *maybe_pending_claims = Some(
                    pending_claims
                        .into_iter()
                        .filter(|claim| !claims_to_process.contains(claim))
                        .collect(),
                );

                Self::deposit_event(Event::Claimed(who, total_claim));
                Ok(().into())
            })
        }
    }

    impl<T: Config> Pallet<T> {
        fn distribute_inflation() {
            // Ensure distribution is not halted
            if IsDistributionHalted::<T>::get() {
                return
            }
            // Calculate the available balance for distribution in the current period
            let total_issuance = T::Currency::total_issuance();
            let distribution_period = T::InflationDistributionPeriod::get();
            let one_year_blocks = T::OneYear::get();
            // Include TotalInflation in the calculation
            let total_inflation = T::TotalInflation::get();
            let inflated_total_issuance: BalanceOf<T> = total_inflation.mul_ceil(total_issuance);

            let balance_for_distribution =
                Perbill::from_rational(distribution_period, one_year_blocks)
                    .mul_ceil(inflated_total_issuance);

            log::debug!("inflated_total_issuance: {:?}", inflated_total_issuance);
            log::debug!("balance_for_distribution: {:?}", balance_for_distribution);
            log::debug!("total_issuance: {:?}", total_issuance);

            // Calculate each portion per percentages
            let attester_rewards = T::AttesterInflation::get().mul_ceil(balance_for_distribution);
            let executor_rewards = T::ExecutorInflation::get().mul_ceil(balance_for_distribution);
            let collator_rewards = T::CollatorInflation::get().mul_ceil(balance_for_distribution);
            let treasury_rewards = T::TreasuryInflation::get().mul_ceil(balance_for_distribution);

            log::debug!("attester_rewards: {:?}", attester_rewards);
            log::debug!("executor_rewards: {:?}", executor_rewards);
            log::debug!("collator_rewards: {:?}", collator_rewards);
            log::debug!("treasury_rewards: {:?}", treasury_rewards);
            // Distribute rewards to attesters
            let attester_rewards_distributed = Self::distribute_attester_rewards(attester_rewards);

            // Distribute rewards to collators
            let collator_rewards_distributed = Self::distribute_collator_rewards(collator_rewards);

            // Distribute rewards to executors
            let executor_rewards_distributed = Self::distribute_executor_rewards(executor_rewards);

            // Transfer the treasury rewards to the treasury account
            T::Currency::deposit_creating(
                &T::TreasuryAccounts::get_treasury_account(TreasuryAccount::Parachain),
                treasury_rewards,
            );

            // Distribute bootstrap rewards from the treasury account
            // todo: uncomment this when bootstrap rewards are implemented
            // Self::distribute_bootstrap_rewards()?;

            // Update the distribution block
            DistributionBlock::<T>::put(frame_system::Pallet::<T>::block_number());

            // Update the distribution history
            let current_block = frame_system::Pallet::<T>::block_number();
            let distribution_record = DistributionRecord {
                block_number: current_block,
                attester_rewards: attester_rewards_distributed,
                collator_rewards: collator_rewards_distributed,
                executor_rewards: executor_rewards_distributed,
                treasury_rewards,
                distributed: attester_rewards_distributed
                    + collator_rewards_distributed
                    + executor_rewards_distributed
                    + treasury_rewards,
                available: balance_for_distribution,
            };
            let mut history = DistributionHistory::<T>::get();
            history.push(distribution_record);
            DistributionHistory::<T>::put(history);
        }

        pub fn distribute_attester_rewards(current_distribution: BalanceOf<T>) -> BalanceOf<T> {
            let honest_active_set = T::Attesters::honest_active_set();
            let active_set_size: usize = T::Attesters::active_set().len();
            let total_attesters = honest_active_set.len() as u32;

            if total_attesters == 0 {
                return Zero::zero()
            }

            let reward_per_attester =
                current_distribution / BalanceOf::<T>::from(active_set_size as u32);

            for attester in honest_active_set {
                let attester_info =
                    if let Some(attester_info) = T::Attesters::read_attester_info(&attester) {
                        attester_info
                    } else {
                        log::warn!(
                            "No attester info found for during rewards distribution {:?}",
                            attester
                        );
                        continue
                    };

                let commission_reward = attester_info.commission.mul_ceil(reward_per_attester);

                // Update the pending claims for the attester
                Self::update_pending_claims(
                    &attester,
                    CircuitRole::Attester,
                    commission_reward,
                    BenefitSource::Inflation,
                );
                let remaining_reward = reward_per_attester.saturating_sub(commission_reward);

                // Get the attester's nominators
                let nominators = T::Attesters::read_nominations(&attester);

                let total_nomination: BalanceOf<T> = nominators
                    .iter()
                    .map(|(_, balance)| *balance)
                    .fold(BalanceOf::<T>::zero(), |acc, x| acc.saturating_add(x));

                // Distribute the remaining reward to the nominators
                for (nominator, nomination_balance) in nominators {
                    let check_nominator_reward = remaining_reward
                        .saturating_mul(nomination_balance)
                        .checked_div(&total_nomination);
                    match check_nominator_reward {
                        Some(nominator_reward) => {
                            Self::update_pending_claims(
                                &nominator,
                                CircuitRole::Staker,
                                nominator_reward,
                                BenefitSource::Inflation,
                            );
                        },
                        None => {
                            // If the nominator reward is zero, then we don't need to do anything
                            // because the nominator's balance is zero
                            log::error!(
                                "Nominator reward is zero for nominator {:?} and attester {:?}",
                                nominator,
                                attester
                            );
                        },
                    }
                }
            }

            BalanceOf::<T>::from(total_attesters).saturating_mul(reward_per_attester)
        }

        pub fn distribute_collator_rewards(current_distribution: BalanceOf<T>) -> BalanceOf<T> {
            let total_collators = Collators::<T>::iter().count() as u32;

            if total_collators == 0 {
                return Zero::zero()
            }

            let reward_per_collator = current_distribution / BalanceOf::<T>::from(total_collators);

            for (collator, _) in Collators::<T>::iter() {
                Self::update_pending_claims(
                    &collator,
                    CircuitRole::Collator,
                    reward_per_collator,
                    BenefitSource::Inflation,
                );
            }

            reward_per_collator.saturating_mul(BalanceOf::<T>::from(total_collators))
        }

        pub fn distribute_executor_rewards(current_distribution: BalanceOf<T>) -> BalanceOf<T> {
            let accumulated_settlements = AccumulatedSettlements::<T>::iter().collect::<Vec<_>>();

            // Get the total settled executions this round
            let total_settled_executions_this_round =
                match Self::total_settled_executions_this_round(accumulated_settlements.clone()) {
                    Ok(total_settled_executions_this_round) => total_settled_executions_this_round,
                    Err(e) => {
                        log::error!(
                            "Arithmetic Overflow when calculating settled executor rewards: {:?}",
                            e
                        );
                        return Zero::zero()
                    },
                };

            // Calculate the proportions of the total settled executions for each executor
            let executions_proportionally_of_total_this_round =
                Self::executions_proportionally_of_total_this_round(
                    accumulated_settlements,
                    total_settled_executions_this_round,
                );

            let mut distibuted_rewards = Zero::zero();

            // Distribute the executor rewards proportionally
            for (executor, accumulated_settlement, proportion) in
                executions_proportionally_of_total_this_round
            {
                let reward = proportion.mul_ceil(current_distribution);

                // Ensure the reward does not exceed 90% of the accumulated settlement amount
                let max_reward = Perbill::from_percent(90).mul_ceil(accumulated_settlement);
                let capped_reward = reward.min(max_reward);

                // Update the pending claims for the executor
                Self::update_pending_claims(
                    &executor,
                    CircuitRole::Executor,
                    capped_reward,
                    BenefitSource::Inflation,
                );

                // Remove the accumulated settlement from the storage
                AccumulatedSettlements::<T>::remove(&executor);

                distibuted_rewards += capped_reward;
            }

            distibuted_rewards
        }

        pub fn process_accumulated_settlements() -> Weight {
            // Ensure settlements accumulation is not halted
            if IsSettlementAccumulationHalted::<T>::get() {
                return Zero::zero()
            }
            // Get the total accumulated settlements
            let executions_this_round = Self::executions_this_round();
            let mut weight: Weight = 0;
            let mut temp_accumulated_settlements: BTreeMap<T::AccountId, BalanceOf<T>> =
                BTreeMap::new();

            for (executor, settlement) in executions_this_round {
                // Add the settlement amount to the existing accumulated settlement
                let accumulated_settlement = AccumulatedSettlements::<T>::get(&executor)
                    .unwrap_or_else(Zero::zero)
                    .saturating_add(settlement.settlement_amount);

                // Update the AccumulatedSettlements storage
                AccumulatedSettlements::<T>::insert(&executor, accumulated_settlement);

                // Accumulate settlements per executor in the temporary map
                let temp_accumulated_settlement = temp_accumulated_settlements
                    .entry(executor.clone())
                    .or_insert(Zero::zero());
                *temp_accumulated_settlement =
                    temp_accumulated_settlement.saturating_add(settlement.settlement_amount);

                weight += T::DbWeight::get().reads_writes(1, 1);
            }

            for (executor, total_accumulated_settlement) in temp_accumulated_settlements {
                // Update the pending claims for the executor
                Self::update_pending_claims(
                    &executor,
                    CircuitRole::Executor,
                    total_accumulated_settlement,
                    BenefitSource::TrafficRewards,
                );

                weight += T::DbWeight::get().reads_writes(1, 1);
            }

            weight
        }

        pub fn executions_this_round() -> Vec<(T::AccountId, Settlement<T::AccountId, BalanceOf<T>>)>
        {
            T::AccountManager::get_settlements_by_role(CircuitRole::Executor)
        }

        pub fn executions_proportionally_of_total_this_round(
            executions_this_round: Vec<(T::AccountId, BalanceOf<T>)>,
            total_settled_executions_this_round: BalanceOf<T>,
        ) -> Vec<(T::AccountId, BalanceOf<T>, Perbill)> {
            let mut executions_proportionally_of_total_this_round = Vec::new();
            for (executor, settlement) in executions_this_round {
                let proportion =
                    Perbill::from_rational(settlement, total_settled_executions_this_round);
                executions_proportionally_of_total_this_round
                    .push((executor, settlement, proportion));
            }
            executions_proportionally_of_total_this_round
        }

        fn total_settled_executions_this_round(
            executions_this_round: Vec<(T::AccountId, BalanceOf<T>)>,
        ) -> Result<BalanceOf<T>, DispatchError> {
            executions_this_round
                .into_iter()
                .map(|(_, settlement)| settlement)
                .try_fold(
                    Zero::zero(),
                    |acc: BalanceOf<T>, settlement: BalanceOf<T>| {
                        acc.checked_add(&settlement)
                            .ok_or(Error::<T>::ArithmeticOverflow.into())
                    },
                )
        }

        fn update_pending_claims(
            account: &T::AccountId,
            role: CircuitRole,
            reward: BalanceOf<T>,
            benefit_source: BenefitSource,
        ) {
            let claim = ClaimableArtifacts {
                beneficiary: account.clone(),
                role,
                total_round_claim: reward,
                benefit_source,
            };

            let mut pending_claims = PendingClaims::<T>::get(account).unwrap_or_default();
            pending_claims.push(claim);
            PendingClaims::<T>::insert(account, pending_claims);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(n: T::BlockNumber) -> Weight {
            let mut weight: Weight = 0;
            if n % T::Clock::round_duration() == Zero::zero() {
                weight += Self::process_accumulated_settlements();
            }
            if n % T::InflationDistributionPeriod::get() == Zero::zero() {
                Self::distribute_inflation();
                weight += T::DbWeight::get().reads_writes(8, 8)
            }

            weight
        }
    }

    // The genesis config type.
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        phantom: PhantomData<T>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                phantom: Default::default(),
            }
        }
    }

    // The build of genesis for the pallet.
    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            IsClaimingHalted::<T>::put(false);
            IsDistributionHalted::<T>::put(false);
            IsSettlementAccumulationHalted::<T>::put(false);
        }
    }
}

#[cfg(test)]
pub mod test {

    use frame_support::{
        assert_err, assert_ok,
        traits::{Currency, Hooks, Len},
    };
    use sp_core::H256;

    use t3rn_mini_mock_runtime::{
        AccountId, Balance, Balances, Clock, ConfigRewards, DistributionHistory, ExtBuilder,
        MiniRuntime, Origin, PendingClaims, Rewards, RewardsError, SettlementsPerRound, System,
    };
    use t3rn_primitives::{
        account_manager::{Outcome, Settlement},
        claimable::{BenefitSource, CircuitRole, ClaimableArtifacts},
        TreasuryAccount, TreasuryAccountProvider,
    };

    #[test]
    fn test_available_distribution_totals_to_max_4_4_percent_after_almost_1_year() {
        let _total_supply_account = AccountId::from([99u8; 32]);

        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Setup
            let distribution_period =
                <MiniRuntime as ConfigRewards>::InflationDistributionPeriod::get();

            pub const TRN: Balance = 1_000_000_000_000;

            let mut available_total_rewards = 0 as Balance;
            let mut actual_total_rewards = 0 as Balance;

            // Weeks per year: 52.1429
            // Weeks per period: 2
            // Test period: 26 periods (52 weeks) - almost 1 year
            let expected_top_yearly_rewards_available: Balance = 4_400_000 * TRN;

            for cnt in 1..27u32 {
                // Simulate the passage of time (two weeks per period)
                System::set_block_number(distribution_period * cnt);

                // Call the distribute_inflation function
                Rewards::on_initialize(distribution_period * cnt);

                // Retrieve the last distribution record
                let history = DistributionHistory::<MiniRuntime>::get();

                assert_eq!(history.len(), cnt as usize);
                let last_record = history.last().unwrap();

                // Add this round's rewards to the total
                actual_total_rewards += last_record.distributed;
                available_total_rewards += last_record.available;
            }

            // Check if the total rewards distributed equal the expected total rewards
            assert_eq!(available_total_rewards, 4389797013799501253);
            assert!(available_total_rewards < expected_top_yearly_rewards_available);
        });
    }

    #[test]
    fn test_inflation_benefits_parachain_treasury_account() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // Setup
            let distribution_period =
                <MiniRuntime as ConfigRewards>::InflationDistributionPeriod::get();

            // Simulate the passage of time (two weeks per period)
            System::set_block_number(distribution_period);

            // Call the distribute_inflation function
            Rewards::on_initialize(distribution_period);

            // Retrieve the last distribution record
            let history = DistributionHistory::<MiniRuntime>::get();
            let last_record = history.last().unwrap();

            let treasury_account =
                <MiniRuntime as TreasuryAccountProvider<AccountId>>::get_treasury_account(
                    TreasuryAccount::Parachain,
                );
            // Check if the total rewards distributed equal the expected total rewards
            assert_eq!(
                last_record.treasury_rewards,
                Balances::total_balance(&treasury_account)
            );
        });
    }

    #[test]
    fn test_distribution_to_executors_subsidies_settlement_proportionally_with_others() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // create 10 Settlements to 10 different executors in AccountManager
            for counter in 1..11u8 {
                let requester = AccountId::from([counter + 100u8; 32]);
                let executor = AccountId::from([counter; 32]);
                let sfx_id = H256::from([counter; 32]);
                let settlement_amount = 100 as Balance;
                SettlementsPerRound::<MiniRuntime>::insert(
                    Clock::current_round(),
                    sfx_id,
                    Settlement {
                        requester,
                        recipient: executor,
                        settlement_amount,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor,
                    },
                );
            }

            Rewards::process_accumulated_settlements();

            let available_rewards_10x_less_of_total_settlements = 100 as Balance;

            let rewards_res = Rewards::distribute_executor_rewards(
                available_rewards_10x_less_of_total_settlements,
            );

            assert_eq!(rewards_res, available_rewards_10x_less_of_total_settlements); // All rewards are distributed to executors

            for counter in 1..11u8 {
                let executor = AccountId::from([counter; 32]);
                let _settlement_amount_plus_rewards = 110 as Balance;
                let pending_claim = Rewards::get_pending_claims(executor.clone());
                assert_eq!(
                    pending_claim,
                    Some(vec![
                        ClaimableArtifacts {
                            beneficiary: executor.clone(),
                            role: CircuitRole::Executor,
                            total_round_claim: 100 as Balance, // Settlement amount
                            benefit_source: BenefitSource::TrafficRewards,
                        },
                        ClaimableArtifacts {
                            beneficiary: executor,
                            role: CircuitRole::Executor,
                            total_round_claim: 10 as Balance, // 10% of 100 as Settlement amount
                            benefit_source: BenefitSource::Inflation,
                        },
                    ])
                );
            }
        });
    }

    #[test]
    fn test_distribution_to_executors_does_not_exceed_90_percent_rewards_subsidy() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // create 10 Settlements to 10 different executors in AccountManager
            for counter in 1..11u8 {
                let requester = AccountId::from([counter + 100u8; 32]);
                let executor = AccountId::from([counter; 32]);
                let sfx_id = H256::from([counter; 32]);
                let settlement_amount = 100 as Balance;
                SettlementsPerRound::<MiniRuntime>::insert(
                    Clock::current_round(),
                    sfx_id,
                    Settlement {
                        requester,
                        recipient: executor,
                        settlement_amount,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor,
                    },
                );
            }
            Rewards::process_accumulated_settlements();

            let available_rewards_more_than_total_settlements = 100 as Balance * 100 as Balance;

            let rewards_res =
                Rewards::distribute_executor_rewards(available_rewards_more_than_total_settlements);

            assert_eq!(rewards_res, 10 as Balance * 90 as Balance); // 90% of 100 TRN times 10 settlements

            for counter in 1..11u8 {
                let executor = AccountId::from([counter; 32]);
                let pending_claim = Rewards::get_pending_claims(executor.clone());
                assert_eq!(pending_claim.len(), 1);
                assert_eq!(
                    pending_claim,
                    Some(vec![
                        ClaimableArtifacts {
                            beneficiary: executor.clone(),
                            role: CircuitRole::Executor,
                            total_round_claim: 100 as Balance, // Settlement amount
                            benefit_source: BenefitSource::TrafficRewards,
                        },
                        ClaimableArtifacts {
                            beneficiary: executor,
                            role: CircuitRole::Executor,
                            total_round_claim: 90 as Balance, // 90% of 100 as Settlement amount
                            benefit_source: BenefitSource::Inflation,
                        },
                    ])
                );
            }
        });
    }

    #[test]
    fn test_cannot_claim_twice_for_the_same_period() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let beneficiary = AccountId::from([99u8; 32]);
            const INITIAL_BALANCE: Balance = 1;
            Balances::deposit_creating(&beneficiary, INITIAL_BALANCE);
            PendingClaims::<MiniRuntime>::insert(
                beneficiary.clone(),
                vec![ClaimableArtifacts {
                    beneficiary: AccountId::from([99u8; 32]),
                    role: CircuitRole::Executor,
                    total_round_claim: 100 as Balance,
                    benefit_source: BenefitSource::Inflation,
                }],
            );

            // Claim the rewards
            let claim_res = Rewards::claim(
                Origin::signed(beneficiary.clone()),
                Some(CircuitRole::Executor),
            );

            assert_ok!(claim_res);
            assert_eq!(
                Rewards::get_pending_claims(beneficiary.clone()),
                Some(vec![])
            );
            assert_eq!(
                Balances::free_balance(beneficiary.clone()),
                100 as Balance + INITIAL_BALANCE
            );

            // Claim the rewards again
            let claim_res = Rewards::claim(
                Origin::signed(beneficiary.clone()),
                Some(CircuitRole::Executor),
            );
            assert_err!(claim_res, RewardsError::<MiniRuntime>::NoPendingClaims);
            assert_eq!(
                Rewards::get_pending_claims(beneficiary.clone()),
                Some(vec![])
            );
            assert_eq!(
                Balances::free_balance(beneficiary),
                100 as Balance + INITIAL_BALANCE
            );
        });
    }

    #[test]
    fn test_claim_executor_rewards_without_inflation_extras() {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            // create 10 Settlements to 10 different executors in AccountManager
            for counter in 1..11u8 {
                let requester = AccountId::from([counter + 100u8; 32]);
                let executor = AccountId::from([counter; 32]);
                let sfx_id = H256::from([counter; 32]);
                let settlement_amount = 100 as Balance;
                SettlementsPerRound::<MiniRuntime>::insert(
                    Clock::current_round(),
                    sfx_id,
                    Settlement {
                        requester,
                        recipient: executor,
                        settlement_amount,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor,
                    },
                );
            }

            Rewards::process_accumulated_settlements();

            let _available_rewards_more_than_total_settlements = 100 as Balance * 100 as Balance;

            for counter in 1..11u8 {
                let executor = AccountId::from([counter; 32]);
                let settlement_amount_without_rewards = 100 as Balance;
                let pending_claim = Rewards::get_pending_claims(executor.clone());

                assert_eq!(pending_claim.len(), 1);
                assert_eq!(
                    pending_claim,
                    Some(vec![ClaimableArtifacts {
                        beneficiary: executor,
                        role: CircuitRole::Executor,
                        total_round_claim: settlement_amount_without_rewards,
                        benefit_source: BenefitSource::TrafficRewards,
                    }])
                );
            }
        });
    }

    #[test]
    fn test_distribution_to_executors_does_not_exceed_90_percent_rewards_subsidy_for_single_executor_and_is_claimable(
    ) {
        let mut ext = ExtBuilder::default().build();
        ext.execute_with(|| {
            let single_executor = AccountId::from([99u8; 32]);
            const INITIAL_BALANCE: Balance = 1;
            Balances::deposit_creating(&single_executor, INITIAL_BALANCE);

            // create 10 Settlements to 10 different executors in AccountManager
            for counter in 1..11u8 {
                let requester = AccountId::from([counter + 100u8; 32]);
                let sfx_id = H256::from([counter; 32]);
                let settlement_amount = 100 as Balance;
                SettlementsPerRound::<MiniRuntime>::insert(
                    Clock::current_round(),
                    sfx_id,
                    Settlement {
                        requester,
                        recipient: single_executor.clone(),
                        settlement_amount,
                        outcome: Outcome::Commit,
                        source: BenefitSource::TrafficRewards,
                        role: CircuitRole::Executor,
                    },
                );
            }
            Rewards::process_accumulated_settlements();

            let available_rewards_more_than_total_settlements = 100 as Balance * 100 as Balance;

            let rewards_res =
                Rewards::distribute_executor_rewards(available_rewards_more_than_total_settlements);

            assert_eq!(rewards_res, 10 as Balance * 90 as Balance); // 90% of 100 TRN times 10 settlements

            let pending_claims = Rewards::get_pending_claims(single_executor.clone()).unwrap();
            assert_eq!(pending_claims.len(), 2); // 1 for the settlement and 1 for the inflation rewards

            // Check the settlement claim
            assert_eq!(
                pending_claims,
                vec![
                    ClaimableArtifacts {
                        beneficiary: single_executor.clone(),
                        role: CircuitRole::Executor,
                        total_round_claim: 10 * 100 as Balance, // 10 settlements times 100 TRN
                        benefit_source: BenefitSource::TrafficRewards,
                    },
                    ClaimableArtifacts {
                        beneficiary: single_executor.clone(),
                        role: CircuitRole::Executor,
                        total_round_claim: 10 * 90 as Balance, // 10 settlements times 90% of 100 TRN
                        benefit_source: BenefitSource::Inflation,
                    }
                ]
            );

            // Claim the rewards
            let claim_res = Rewards::claim(
                Origin::signed(single_executor.clone()),
                Some(CircuitRole::Executor),
            );
            assert_ok!(claim_res);
            assert_eq!(
                Rewards::get_pending_claims(single_executor.clone()),
                Some(vec![])
            );
            assert_eq!(
                Balances::free_balance(single_executor),
                1900 as Balance + INITIAL_BALANCE
            );
        });
    }
}