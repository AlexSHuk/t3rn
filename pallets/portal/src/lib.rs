// #![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
//
// mod types;
// mod crypto;

// use crate::types::{Header, ValidatorSet, H256, Proof, Receipt};
use t3rn_primitives::portal::ErrorMsg;
// use codec::Decode;
// use sp_std::vec::Vec;


#[frame_support::pallet]
pub mod pallet {
    use scale_info::prelude::string::String;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use core::convert::TryInto;
    use sp_std::vec::Vec;
    use t3rn_primitives::{xdns::Xdns};
    use codec::Decode;
    use t3rn_primitives::{
        side_effect::interface::SideEffectInterface,
        xdns::{AllowedSideEffect, XdnsRecord},
        abi::{GatewayABIConfig},
        portal::{ErrorMsg, RococoBridge},
        ChainId, EscrowTrait, GatewaySysProps, GatewayType, GatewayVendor, GatewayGenesisConfig,
    };
    use t3rn_primitives::bridges::runtime::Chain;
    use t3rn_primitives::portal::RegistrationData;


    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config:
    frame_system::Config
    + pallet_grandpa_finality_verifier::Config<RococoBridge>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Xdns: Xdns<Self>;


    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // #[pallet::storage]
    // #[pallet::getter(fn validators)]
    // pub type Validators<T> = StorageValue<_, ValidatorSet>;
    //
    // #[pallet::storage]
    // #[pallet::getter(fn headers)]
    // pub type Headers<T> = StorageMap<
    //     _,
    //     Identity,
    //     H256,
    //     Header,
    // >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// Gateway was registered successsfully. [ChainId]
        GatewayRegistered(ChainId),
        /// Gateway owner was set successfully. [ChainId, Vec<u8>]
        SetOwner(ChainId, Vec<u8>),
        /// Gateway was set operational. [ChainId, bool]
        SetOperational(ChainId, bool),
        // Emits error msg of function call
        // Error(ErrorMsg),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// The creation of the XDNS record was not successful
        XdnsRecordCreationFailed,
        /// Gateway registration failed
        RegistrationError,
        /// The gateways vendor is not available, which is a result of a missing XDNS record.
        GatewayVendorNotFound,
        /// Finality Verifier owner can't be set.
        SetOwnerError,
        /// Finality Verifiers operational status can't be updated
        SetOperationalError
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn register_gateway(
            origin: OriginFor<T>,
            registration_data: RegistrationData
        ) -> DispatchResultWithPostInfo {
            <T as Config>::Xdns::add_new_xdns_record(
                origin.clone(),
                registration_data.url,
                registration_data.gateway_id,
                None,
                registration_data.gateway_abi.clone(),
                registration_data.gateway_vendor.clone(),
                registration_data.gateway_type.clone(),
                registration_data.gateway_genesis,
                registration_data.gateway_sys_props.clone(),
                registration_data.allowed_side_effects.clone(),
            )?;

            let res = match registration_data.gateway_vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::initialize(origin, registration_data.gateway_id, registration_data.encoded_registration_data),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::GatewayRegistered(registration_data.gateway_id));
                     return Ok(().into())
                },
                Err(_err) => {
                    return Err(Error::<T>::RegistrationError.into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_owner(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            encoded_new_owner: Vec<u8>
        ) -> DispatchResultWithPostInfo {
            let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

            let vendor = match vendor_result {
                Ok(vendor) => vendor,
                Err(_msg) => {
                    return Err(Error::<T>::GatewayVendorNotFound.into())
                }
            };

            let res = match vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::set_owner(origin, gateway_id, encoded_new_owner.clone()),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::SetOwner(gateway_id, encoded_new_owner));
                     return Ok(().into())
                },
                Err(_msg) => {
                    return Err(Error::<T>::SetOwnerError.into())
                }
            }
        }

        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_operational(
            origin: OriginFor<T>,
            gateway_id: ChainId,
            operational: bool
        ) -> DispatchResultWithPostInfo {
            let vendor_result = <T as Config>::Xdns::get_gateway_vendor(&gateway_id);

            let vendor = match vendor_result {
                Ok(vendor) => vendor,
                Err(_msg) => {
                    return Err(Error::<T>::GatewayVendorNotFound.into())
                }
            };

            let res = match vendor {
                GatewayVendor::Rococo =>  pallet_grandpa_finality_verifier::Pallet::<T, RococoBridge>::set_operational(origin, operational, gateway_id),
                _ => unimplemented!()
            };

            match res {
                Ok(_) => {
                     Self::deposit_event(Event::SetOperational(gateway_id, operational));
                     return Ok(().into())
                },
                Err(_msg) => {
                    return Err(Error::<T>::SetOperationalError.into())
                }
            }
        }
    }
}

// impl<T: Config> Portal<T> for Pallet<T> {
// fn init_bridge_instance (
// 	encoded_header: Vec<u8>
// ) -> Result<(), &'static str> {
//
// 	let header: Header = match Decode::decode(&mut &*encoded_header) {
// 		Ok(header) => header,
// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
// 	};
//
// 	if header.number % 200 != 0 {
// 		return Err(Error::<T>::NonEpochBlock.into())
// 	}
//
// 	// header is invalid. returning error
// 	if let Err(None) = header.signature_valid() {
// 		return Err(Error::<T>::InvalidHeader.into())
// 	}
//
// 	let validators = ValidatorSet {
// 		last_update: header.number,
// 		validators: header.validators.unwrap()
// 	};
//
// 	<Validators<T>>::put(validators);
// 	<Headers<T>>::insert(
// 		header.hash(),
// 		header,
// 	);
//
// 	Ok(())
// }

// fn check_inclusion(
// 	enc_receipt: Vec<u8>,
// 	enc_proof: Option<Vec<u8>>,
// 	enc_block_hash: Vec<u8>
// ) -> Result<(), &'static str> {
// 	// ToDo: remove for release
// 	if let None = enc_proof {
// 		return Ok(())
// 	}
// 	let block_hash: H256 = match Decode::decode(&mut &*enc_block_hash) {
// 		Ok(res) => res,
// 		Err(_) => return Err(Error::<T>::InvalidHash.into())
// 	};
//
// 	let receipt_root: H256 = match <Headers<T>>::try_get(block_hash) {
// 		Ok(res) => res.receipts_root,
// 		Err(_) => return Err(Error::<T>::HeaderNotFound.into())
// 	};
//
// 	let receipt: Receipt = match Decode::decode(&mut &*enc_receipt) {
// 		Ok(res) => res,
// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
// 	};
//
// 	let proof: Proof = match Decode::decode(&mut &*enc_proof.unwrap()) {
// 		Ok(res) => res,
// 		Err(_) => return Err(Error::<T>::InvalidEncoding.into())
// 	};
//
// 	match receipt.in_block(receipt_root.as_fixed_bytes(), proof) {
// 		Ok(_) => return Ok(()),
// 		Err(_) => return Err(Error::<T>::InvalidInclusionProof.into())
// 	}
// }
// }