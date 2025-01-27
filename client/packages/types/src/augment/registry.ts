// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type { CircuitStandaloneRuntimeOriginCaller, CircuitStandaloneRuntimeRuntime, EthbloomBloom, EthereumLog, FinalityGrandpaCommit, FinalityGrandpaEquivocationPrecommit, FinalityGrandpaEquivocationPrevote, FinalityGrandpaPrecommit, FinalityGrandpaPrevote, FinalityGrandpaSignedPrecommit, FrameSupportDispatchRawOrigin, FrameSupportPalletId, FrameSupportTokensMiscBalanceStatus, FrameSupportWeightsDispatchClass, FrameSupportWeightsDispatchInfo, FrameSupportWeightsPays, FrameSupportWeightsPerDispatchClassU32, FrameSupportWeightsPerDispatchClassU64, FrameSupportWeightsPerDispatchClassWeightsPerClass, FrameSupportWeightsRuntimeDbWeight, FrameSystemAccountInfo, FrameSystemCall, FrameSystemError, FrameSystemEvent, FrameSystemEventRecord, FrameSystemExtensionsCheckGenesis, FrameSystemExtensionsCheckNonZeroSender, FrameSystemExtensionsCheckNonce, FrameSystemExtensionsCheckSpecVersion, FrameSystemExtensionsCheckTxVersion, FrameSystemExtensionsCheckWeight, FrameSystemLastRuntimeUpgradeInfo, FrameSystemLimitsBlockLength, FrameSystemLimitsBlockWeights, FrameSystemLimitsWeightsPerClass, FrameSystemPhase, Pallet3vmCall, Pallet3vmError, Pallet3vmEvent, PalletAccountManagerCall, PalletAccountManagerError, PalletAccountManagerEvent, PalletAssetTxPaymentChargeAssetTxPayment, PalletAssetsApproval, PalletAssetsAssetAccount, PalletAssetsAssetDetails, PalletAssetsAssetMetadata, PalletAssetsCall, PalletAssetsDestroyWitness, PalletAssetsError, PalletAssetsEvent, PalletAssetsExistenceReason, PalletAttestersBatchMessage, PalletAttestersBatchStatus, PalletAttestersCall, PalletAttestersError, PalletAttestersEvent, PalletAuthorshipCall, PalletAuthorshipError, PalletAuthorshipUncleEntryItem, PalletBalancesAccountData, PalletBalancesBalanceLock, PalletBalancesCall, PalletBalancesError, PalletBalancesEvent, PalletBalancesReasons, PalletBalancesReleases, PalletBalancesReserveData, PalletCircuitCall, PalletCircuitError, PalletCircuitEvent, PalletCircuitVacuumCall, PalletCircuitVacuumError, PalletCircuitVacuumEvent, PalletCircuitVacuumOrderStatusRead, PalletClockError, PalletClockEvent, PalletContractsCall, PalletContractsError, PalletContractsEvent, PalletContractsRegistryCall, PalletContractsRegistryError, PalletContractsRegistryEvent, PalletContractsSchedule, PalletContractsScheduleHostFnWeights, PalletContractsScheduleInstructionWeights, PalletContractsScheduleLimits, PalletContractsStorageDeletedContract, PalletContractsStorageRawContractInfo, PalletContractsWasmOwnerInfo, PalletContractsWasmPrefabWasmModule, PalletEth2FinalityVerifierBeaconBlockHeader, PalletEth2FinalityVerifierBeaconCheckpoint, PalletEth2FinalityVerifierCall, PalletEth2FinalityVerifierCheckpoint, PalletEth2FinalityVerifierEpochSubmitted, PalletEth2FinalityVerifierError, PalletEth2FinalityVerifierEthereumEventInclusionProof, PalletEth2FinalityVerifierEthereumReceiptInclusionProof, PalletEth2FinalityVerifierEvent, PalletEth2FinalityVerifierExecutionCheckpoint, PalletEth2FinalityVerifierExecutionHeader, PalletEth2FinalityVerifierExecutionPayload, PalletEth2FinalityVerifierMerkleProof, PalletEth2FinalityVerifierSyncCommittee, PalletEvmCall, PalletEvmError, PalletEvmEvent, PalletEvmThreeVmInfo, PalletGrandpaCall, PalletGrandpaError, PalletGrandpaEvent, PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet, PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification, PalletGrandpaFinalityVerifierCall, PalletGrandpaFinalityVerifierError, PalletGrandpaFinalityVerifierEvent, PalletGrandpaFinalityVerifierParachainRegistrationData, PalletGrandpaStoredPendingChange, PalletGrandpaStoredState, PalletIdentityBitFlags, PalletIdentityCall, PalletIdentityError, PalletIdentityEvent, PalletIdentityIdentityField, PalletIdentityIdentityInfo, PalletIdentityJudgement, PalletIdentityRegistrarInfo, PalletIdentityRegistration, PalletMaintenanceModeCall, PalletMaintenanceModeError, PalletMaintenanceModeEvent, PalletPortalCall, PalletPortalError, PalletPortalEvent, PalletRewardsAssetType, PalletRewardsCall, PalletRewardsDistributionRecord, PalletRewardsError, PalletRewardsEvent, PalletRewardsTreasuryBalanceSheet, PalletSudoCall, PalletSudoError, PalletSudoEvent, PalletTimestampCall, PalletTransactionPaymentEvent, PalletTransactionPaymentReleases, PalletTreasuryCall, PalletTreasuryError, PalletTreasuryEvent, PalletTreasuryProposal, PalletUtilityCall, PalletUtilityError, PalletUtilityEvent, PalletXdnsCall, PalletXdnsError, PalletXdnsEvent, SpConsensusAuraSr25519AppSr25519Public, SpCoreEcdsaSignature, SpCoreEd25519Public, SpCoreEd25519Signature, SpCoreSr25519Public, SpCoreSr25519Signature, SpCoreVoid, SpFinalityGrandpaAppPublic, SpFinalityGrandpaAppSignature, SpFinalityGrandpaEquivocation, SpFinalityGrandpaEquivocationProof, SpRuntimeArithmeticError, SpRuntimeBlakeTwo256, SpRuntimeDigest, SpRuntimeDigestDigestItem, SpRuntimeDispatchError, SpRuntimeHeader, SpRuntimeModuleError, SpRuntimeMultiSignature, SpRuntimeTokenError, SpRuntimeTransactionalError, SpVersionRuntimeVersion, T3rnAbiRecodeCodec, T3rnAbiSfxAbi, T3rnAbiSfxAbiPerCodecAbiDescriptors, T3rnPrimitivesAccountManagerOutcome, T3rnPrimitivesAccountManagerRequestCharge, T3rnPrimitivesAccountManagerSettlement, T3rnPrimitivesAttestersAttesterInfo, T3rnPrimitivesAttestersLatencyStatus, T3rnPrimitivesCircuitTypesAdaptiveTimeout, T3rnPrimitivesCircuitTypesCause, T3rnPrimitivesCircuitTypesCircuitStatus, T3rnPrimitivesCircuitTypesOrderSFX, T3rnPrimitivesCircuitTypesSfxAction, T3rnPrimitivesCircuitTypesXExecSignal, T3rnPrimitivesClaimableBenefitSource, T3rnPrimitivesClaimableCircuitRole, T3rnPrimitivesClaimableClaimableArtifacts, T3rnPrimitivesCommonRoundInfo, T3rnPrimitivesContractMetadata, T3rnPrimitivesContractMetadataContractType, T3rnPrimitivesContractsRegistryAuthorInfo, T3rnPrimitivesContractsRegistryRegistryContract, T3rnPrimitivesEthereumToken, T3rnPrimitivesExecutionVendor, T3rnPrimitivesFinalityVerifierActivity, T3rnPrimitivesGatewayActivity, T3rnPrimitivesGatewayVendor, T3rnPrimitivesSpeedMode, T3rnPrimitivesStorageRawAliveContractInfo, T3rnPrimitivesSubstrateToken, T3rnPrimitivesTokenInfo, T3rnPrimitivesVolatileLocalState, T3rnPrimitivesXdnsEpochEstimate, T3rnPrimitivesXdnsGatewayRecord, T3rnPrimitivesXdnsTokenRecord, T3rnSdkPrimitivesSignalExecutionSignal, T3rnSdkPrimitivesSignalKillReason, T3rnSdkPrimitivesSignalSignalKind, T3rnTypesBidSfxBid, T3rnTypesFsxFullSideEffect, T3rnTypesGatewayContractActionDesc, T3rnTypesSfxConfirmationOutcome, T3rnTypesSfxConfirmedSideEffect, T3rnTypesSfxSecurityLvl, T3rnTypesSfxSideEffect, XpFormatStatus, XpFormatXbiResult } from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
  interface InterfaceTypes {
    CircuitStandaloneRuntimeOriginCaller: CircuitStandaloneRuntimeOriginCaller;
    CircuitStandaloneRuntimeRuntime: CircuitStandaloneRuntimeRuntime;
    EthbloomBloom: EthbloomBloom;
    EthereumLog: EthereumLog;
    FinalityGrandpaCommit: FinalityGrandpaCommit;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FinalityGrandpaSignedPrecommit: FinalityGrandpaSignedPrecommit;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSupportWeightsDispatchClass: FrameSupportWeightsDispatchClass;
    FrameSupportWeightsDispatchInfo: FrameSupportWeightsDispatchInfo;
    FrameSupportWeightsPays: FrameSupportWeightsPays;
    FrameSupportWeightsPerDispatchClassU32: FrameSupportWeightsPerDispatchClassU32;
    FrameSupportWeightsPerDispatchClassU64: FrameSupportWeightsPerDispatchClassU64;
    FrameSupportWeightsPerDispatchClassWeightsPerClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
    FrameSupportWeightsRuntimeDbWeight: FrameSupportWeightsRuntimeDbWeight;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    Pallet3vmCall: Pallet3vmCall;
    Pallet3vmError: Pallet3vmError;
    Pallet3vmEvent: Pallet3vmEvent;
    PalletAccountManagerCall: PalletAccountManagerCall;
    PalletAccountManagerError: PalletAccountManagerError;
    PalletAccountManagerEvent: PalletAccountManagerEvent;
    PalletAssetTxPaymentChargeAssetTxPayment: PalletAssetTxPaymentChargeAssetTxPayment;
    PalletAssetsApproval: PalletAssetsApproval;
    PalletAssetsAssetAccount: PalletAssetsAssetAccount;
    PalletAssetsAssetDetails: PalletAssetsAssetDetails;
    PalletAssetsAssetMetadata: PalletAssetsAssetMetadata;
    PalletAssetsCall: PalletAssetsCall;
    PalletAssetsDestroyWitness: PalletAssetsDestroyWitness;
    PalletAssetsError: PalletAssetsError;
    PalletAssetsEvent: PalletAssetsEvent;
    PalletAssetsExistenceReason: PalletAssetsExistenceReason;
    PalletAttestersBatchMessage: PalletAttestersBatchMessage;
    PalletAttestersBatchStatus: PalletAttestersBatchStatus;
    PalletAttestersCall: PalletAttestersCall;
    PalletAttestersError: PalletAttestersError;
    PalletAttestersEvent: PalletAttestersEvent;
    PalletAuthorshipCall: PalletAuthorshipCall;
    PalletAuthorshipError: PalletAuthorshipError;
    PalletAuthorshipUncleEntryItem: PalletAuthorshipUncleEntryItem;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReleases: PalletBalancesReleases;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletCircuitCall: PalletCircuitCall;
    PalletCircuitError: PalletCircuitError;
    PalletCircuitEvent: PalletCircuitEvent;
    PalletCircuitVacuumCall: PalletCircuitVacuumCall;
    PalletCircuitVacuumError: PalletCircuitVacuumError;
    PalletCircuitVacuumEvent: PalletCircuitVacuumEvent;
    PalletCircuitVacuumOrderStatusRead: PalletCircuitVacuumOrderStatusRead;
    PalletClockError: PalletClockError;
    PalletClockEvent: PalletClockEvent;
    PalletContractsCall: PalletContractsCall;
    PalletContractsError: PalletContractsError;
    PalletContractsEvent: PalletContractsEvent;
    PalletContractsRegistryCall: PalletContractsRegistryCall;
    PalletContractsRegistryError: PalletContractsRegistryError;
    PalletContractsRegistryEvent: PalletContractsRegistryEvent;
    PalletContractsSchedule: PalletContractsSchedule;
    PalletContractsScheduleHostFnWeights: PalletContractsScheduleHostFnWeights;
    PalletContractsScheduleInstructionWeights: PalletContractsScheduleInstructionWeights;
    PalletContractsScheduleLimits: PalletContractsScheduleLimits;
    PalletContractsStorageDeletedContract: PalletContractsStorageDeletedContract;
    PalletContractsStorageRawContractInfo: PalletContractsStorageRawContractInfo;
    PalletContractsWasmOwnerInfo: PalletContractsWasmOwnerInfo;
    PalletContractsWasmPrefabWasmModule: PalletContractsWasmPrefabWasmModule;
    PalletEth2FinalityVerifierBeaconBlockHeader: PalletEth2FinalityVerifierBeaconBlockHeader;
    PalletEth2FinalityVerifierBeaconCheckpoint: PalletEth2FinalityVerifierBeaconCheckpoint;
    PalletEth2FinalityVerifierCall: PalletEth2FinalityVerifierCall;
    PalletEth2FinalityVerifierCheckpoint: PalletEth2FinalityVerifierCheckpoint;
    PalletEth2FinalityVerifierEpochSubmitted: PalletEth2FinalityVerifierEpochSubmitted;
    PalletEth2FinalityVerifierError: PalletEth2FinalityVerifierError;
    PalletEth2FinalityVerifierEthereumEventInclusionProof: PalletEth2FinalityVerifierEthereumEventInclusionProof;
    PalletEth2FinalityVerifierEthereumReceiptInclusionProof: PalletEth2FinalityVerifierEthereumReceiptInclusionProof;
    PalletEth2FinalityVerifierEvent: PalletEth2FinalityVerifierEvent;
    PalletEth2FinalityVerifierExecutionCheckpoint: PalletEth2FinalityVerifierExecutionCheckpoint;
    PalletEth2FinalityVerifierExecutionHeader: PalletEth2FinalityVerifierExecutionHeader;
    PalletEth2FinalityVerifierExecutionPayload: PalletEth2FinalityVerifierExecutionPayload;
    PalletEth2FinalityVerifierMerkleProof: PalletEth2FinalityVerifierMerkleProof;
    PalletEth2FinalityVerifierSyncCommittee: PalletEth2FinalityVerifierSyncCommittee;
    PalletEvmCall: PalletEvmCall;
    PalletEvmError: PalletEvmError;
    PalletEvmEvent: PalletEvmEvent;
    PalletEvmThreeVmInfo: PalletEvmThreeVmInfo;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet: PalletGrandpaFinalityVerifierBridgesHeaderChainAuthoritySet;
    PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification: PalletGrandpaFinalityVerifierBridgesHeaderChainJustificationGrandpaJustification;
    PalletGrandpaFinalityVerifierCall: PalletGrandpaFinalityVerifierCall;
    PalletGrandpaFinalityVerifierError: PalletGrandpaFinalityVerifierError;
    PalletGrandpaFinalityVerifierEvent: PalletGrandpaFinalityVerifierEvent;
    PalletGrandpaFinalityVerifierParachainRegistrationData: PalletGrandpaFinalityVerifierParachainRegistrationData;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletIdentityBitFlags: PalletIdentityBitFlags;
    PalletIdentityCall: PalletIdentityCall;
    PalletIdentityError: PalletIdentityError;
    PalletIdentityEvent: PalletIdentityEvent;
    PalletIdentityIdentityField: PalletIdentityIdentityField;
    PalletIdentityIdentityInfo: PalletIdentityIdentityInfo;
    PalletIdentityJudgement: PalletIdentityJudgement;
    PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
    PalletIdentityRegistration: PalletIdentityRegistration;
    PalletMaintenanceModeCall: PalletMaintenanceModeCall;
    PalletMaintenanceModeError: PalletMaintenanceModeError;
    PalletMaintenanceModeEvent: PalletMaintenanceModeEvent;
    PalletPortalCall: PalletPortalCall;
    PalletPortalError: PalletPortalError;
    PalletPortalEvent: PalletPortalEvent;
    PalletRewardsAssetType: PalletRewardsAssetType;
    PalletRewardsCall: PalletRewardsCall;
    PalletRewardsDistributionRecord: PalletRewardsDistributionRecord;
    PalletRewardsError: PalletRewardsError;
    PalletRewardsEvent: PalletRewardsEvent;
    PalletRewardsTreasuryBalanceSheet: PalletRewardsTreasuryBalanceSheet;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTreasuryCall: PalletTreasuryCall;
    PalletTreasuryError: PalletTreasuryError;
    PalletTreasuryEvent: PalletTreasuryEvent;
    PalletTreasuryProposal: PalletTreasuryProposal;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletXdnsCall: PalletXdnsCall;
    PalletXdnsError: PalletXdnsError;
    PalletXdnsEvent: PalletXdnsEvent;
    SpConsensusAuraSr25519AppSr25519Public: SpConsensusAuraSr25519AppSr25519Public;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreVoid: SpCoreVoid;
    SpFinalityGrandpaAppPublic: SpFinalityGrandpaAppPublic;
    SpFinalityGrandpaAppSignature: SpFinalityGrandpaAppSignature;
    SpFinalityGrandpaEquivocation: SpFinalityGrandpaEquivocation;
    SpFinalityGrandpaEquivocationProof: SpFinalityGrandpaEquivocationProof;
    SpRuntimeArithmeticError: SpRuntimeArithmeticError;
    SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeHeader: SpRuntimeHeader;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    T3rnAbiRecodeCodec: T3rnAbiRecodeCodec;
    T3rnAbiSfxAbi: T3rnAbiSfxAbi;
    T3rnAbiSfxAbiPerCodecAbiDescriptors: T3rnAbiSfxAbiPerCodecAbiDescriptors;
    T3rnPrimitivesAccountManagerOutcome: T3rnPrimitivesAccountManagerOutcome;
    T3rnPrimitivesAccountManagerRequestCharge: T3rnPrimitivesAccountManagerRequestCharge;
    T3rnPrimitivesAccountManagerSettlement: T3rnPrimitivesAccountManagerSettlement;
    T3rnPrimitivesAttestersAttesterInfo: T3rnPrimitivesAttestersAttesterInfo;
    T3rnPrimitivesAttestersLatencyStatus: T3rnPrimitivesAttestersLatencyStatus;
    T3rnPrimitivesCircuitTypesAdaptiveTimeout: T3rnPrimitivesCircuitTypesAdaptiveTimeout;
    T3rnPrimitivesCircuitTypesCause: T3rnPrimitivesCircuitTypesCause;
    T3rnPrimitivesCircuitTypesCircuitStatus: T3rnPrimitivesCircuitTypesCircuitStatus;
    T3rnPrimitivesCircuitTypesOrderSFX: T3rnPrimitivesCircuitTypesOrderSFX;
    T3rnPrimitivesCircuitTypesSfxAction: T3rnPrimitivesCircuitTypesSfxAction;
    T3rnPrimitivesCircuitTypesXExecSignal: T3rnPrimitivesCircuitTypesXExecSignal;
    T3rnPrimitivesClaimableBenefitSource: T3rnPrimitivesClaimableBenefitSource;
    T3rnPrimitivesClaimableCircuitRole: T3rnPrimitivesClaimableCircuitRole;
    T3rnPrimitivesClaimableClaimableArtifacts: T3rnPrimitivesClaimableClaimableArtifacts;
    T3rnPrimitivesCommonRoundInfo: T3rnPrimitivesCommonRoundInfo;
    T3rnPrimitivesContractMetadata: T3rnPrimitivesContractMetadata;
    T3rnPrimitivesContractMetadataContractType: T3rnPrimitivesContractMetadataContractType;
    T3rnPrimitivesContractsRegistryAuthorInfo: T3rnPrimitivesContractsRegistryAuthorInfo;
    T3rnPrimitivesContractsRegistryRegistryContract: T3rnPrimitivesContractsRegistryRegistryContract;
    T3rnPrimitivesEthereumToken: T3rnPrimitivesEthereumToken;
    T3rnPrimitivesExecutionVendor: T3rnPrimitivesExecutionVendor;
    T3rnPrimitivesFinalityVerifierActivity: T3rnPrimitivesFinalityVerifierActivity;
    T3rnPrimitivesGatewayActivity: T3rnPrimitivesGatewayActivity;
    T3rnPrimitivesGatewayVendor: T3rnPrimitivesGatewayVendor;
    T3rnPrimitivesSpeedMode: T3rnPrimitivesSpeedMode;
    T3rnPrimitivesStorageRawAliveContractInfo: T3rnPrimitivesStorageRawAliveContractInfo;
    T3rnPrimitivesSubstrateToken: T3rnPrimitivesSubstrateToken;
    T3rnPrimitivesTokenInfo: T3rnPrimitivesTokenInfo;
    T3rnPrimitivesVolatileLocalState: T3rnPrimitivesVolatileLocalState;
    T3rnPrimitivesXdnsEpochEstimate: T3rnPrimitivesXdnsEpochEstimate;
    T3rnPrimitivesXdnsGatewayRecord: T3rnPrimitivesXdnsGatewayRecord;
    T3rnPrimitivesXdnsTokenRecord: T3rnPrimitivesXdnsTokenRecord;
    T3rnSdkPrimitivesSignalExecutionSignal: T3rnSdkPrimitivesSignalExecutionSignal;
    T3rnSdkPrimitivesSignalKillReason: T3rnSdkPrimitivesSignalKillReason;
    T3rnSdkPrimitivesSignalSignalKind: T3rnSdkPrimitivesSignalSignalKind;
    T3rnTypesBidSfxBid: T3rnTypesBidSfxBid;
    T3rnTypesFsxFullSideEffect: T3rnTypesFsxFullSideEffect;
    T3rnTypesGatewayContractActionDesc: T3rnTypesGatewayContractActionDesc;
    T3rnTypesSfxConfirmationOutcome: T3rnTypesSfxConfirmationOutcome;
    T3rnTypesSfxConfirmedSideEffect: T3rnTypesSfxConfirmedSideEffect;
    T3rnTypesSfxSecurityLvl: T3rnTypesSfxSecurityLvl;
    T3rnTypesSfxSideEffect: T3rnTypesSfxSideEffect;
    XpFormatStatus: XpFormatStatus;
    XpFormatXbiResult: XpFormatXbiResult;
  } // InterfaceTypes
} // declare module
