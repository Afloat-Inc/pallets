#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
use super::*;
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + pallet_uniques::Config {
		type Event: From<Event<Self, I>> + IsType<<Self as frame_system::Config>::Event>;

		// The maximum length of an attribute key.
		/*
		#[pallet::constant]
		type KeyLimit: Get<u32>;

		/// The maximum length of an attribute value.
		#[pallet::constant]
		type ValueLimit: Get<u32>;*/

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		FruniqueCreated(T::AccountId, T::AccountId, T::ClassId, T::InstanceId),
		FruniqueDivided(T::AccountId, T::AccountId, T::ClassId, T::InstanceId),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		NoneValue,
		NoPermission,
		StorageOverflow,
		NotYetImplemented,
	}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Issue a new frunique from a public origin.
		///
		/// A new NFT (unique) is created and reserved,
		/// a fungible token (asset) is created and minted to the owner.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// `AssetDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `asset_id`: The identifier of the new asset. This must not be currently in use to identify
		/// an existing asset.
		/// - `class`: The identifier of the new asset class. This must not be currently in use.
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		///
		/// Emits `FruniqueCreated` event when successful.
		///
		/// Weight: `O(1)`
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(
			origin: OriginFor<T>,
			#[pallet::compact] class_id: T::ClassId,
			instance_id: T::InstanceId,
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;

			// create an NFT in the uniques pallet
			pallet_uniques::Pallet::<T>::create(origin.clone(), class_id.clone(), admin.clone())?;
			pallet_uniques::Pallet::<T>::mint(
				origin.clone(),
				class_id.clone(),
				instance_id.clone(),
				admin.clone(),
			)?;

			let admin = T::Lookup::lookup(admin)?;
			Self::deposit_event(Event::FruniqueCreated(owner, admin, class_id, instance_id));

			Ok(())
		}

		/// Create a new frunique that is a child or subset of the parent frunique
		///
		/// A new NFT (unique) is created and reserved,
		/// a fungible token (asset) is created and minted to the owner.
		///
		/// The origin must be Signed and the sender must have sufficient funds free.
		///
		/// `AssetDeposit` funds of sender are reserved.
		///
		/// Parameters:
		/// - `class_id`: the class for the item that is spawning
		/// - `instance_id`: the identifier of the asset that is spawning. This must not be currently in use to identify
		/// an existing asset.
		/// - `new_instance_id`: The identifier of the new asset being created. This must not be currently in use to identify
		/// an existing asset.
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		///
		/// Emits `FruniqueDivided` event when successful.
		///
		/// Weight: `O(1)`		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]

		#[pallet::weight(10_000 + T::DbWeight::get().writes(3))]
		pub fn divide(
			_origin: OriginFor<T>,
			#[pallet::compact] _class_id: T::ClassId,
			_instance_id: T::InstanceId,
			_new_instance_id: T::InstanceId,
			_amount: u64,
		) -> DispatchResult {
			ensure!(false, Error::<T, I>::NotYetImplemented);
			//let owner = ensure_signed(_origin.clone())?;
			//let instance = Asset::<T, I>::insert(&_class_id, &_instance_id, details);
			//let instance = pallet_uniques::Pallet::<T>::
			// Get the members from `special-pallet` pallet
			//let who = special_pallet::Pallet::<T>::get();
			// retrieve the instance being divided
			// let instance: T::InstanceId = Self.Asset::<T, _>::get(class_id.clone(), instance_id.clone())
			// 	.ok_or(Error::<T, _>::Unknown)?;
			// pallet_uniques::Pallet::<T>::mint(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	instance.admin.clone(),
			// )?;

			// set the parent instance_id to the metadata
			// // (probably will need to record this in the fruniques pallet storage)
			// pallet_uniques::Pallet::<T>::set_attribute(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	"parent".into(),            // key: BoundedVec<u8, T::KeyLimit>,
			// 	instance_id.clone().into(), // value: BoundedVec<u8, T::ValueLimit>,
			// )?;

			// pallet_uniques::Pallet::<T>::set_attribute(
			// 	origin.clone(),
			// 	class_id.clone(),
			// 	new_instance_id.clone(),
			// 	"amount".into(), // key: BoundedVec<u8, T::KeyLimit>,
			// 	amount,          // value: BoundedVec<u8, T::ValueLimit>,
			// )?;

			// let admin = T::Lookup::lookup(instance.admin)?;
			// Self::deposit_event(Event::FruniqueDivided(
			// 	owner,
			// 	admin,
			// 	class_id,
			// 	new_instance_id, // non-fungible token parameters
			// 	0,
			// 	0,
			// 	0,
			// )); // fungible token parameters
			Ok(())
		}

		/// ## NFT Division
		/// 
		/// PD: the Key/value length limits are ihnerited from the uniques pallet,
		/// so they're not explicitly declared on this pallet 
		/// 
		/// (por ahora dejar de lado la division del valor numerico)
		/// 
		/// Boilerplate parameters:
		/// origin, admin
		/// Para dividir un nft se necesita :
		/// class_id, instance_id (el que será padre), num_fractions
		/// 
		/// 1.- Boilerplate (setup, conversions, ensure_signed)
		/// 2.- Instance n number of nfts (with the respective parentId)
		/// 3.- Freeze the nft? Not clear, so not at the moment
		/// 4.- Ok(()) ??
		/// 
		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn divide2(
			origin: OriginFor<T>, 
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
			class_id: T::ClassId, 
			instance_id: T::InstanceId,
			num_fractions:u32,
		)->DispatchResult{
			// 1.- Boilerplate (setup, conversions, ensure_signed)
			let _owner = ensure_signed( origin.clone())?;
			// create the fractions in a loop
			let b = instance_id.encode();
			let vector = BoundedVec::<u8,T::ValueLimit>::try_from(b).unwrap();
			for i in 0..num_fractions{
				//get the tentaitve id?
				//let mut parent_id = BoundedVec::<u8,T::KeyLimit>::default();
				//let mut value = BoundedVec::<u8,T::ValueLimit>::default();
				let parent_id = BoundedVec::<u8,T::KeyLimit>::try_from("parent_id".encode()).unwrap();
				//parent_id.try_push('i' as u8 ).map_err(|_| Error::<T, I>::StorageOverflow)?;
				//value.try_push('f' as u8 ).map_err(|_| Error::<T, I>::StorageOverflow)?;
				pallet_uniques::Pallet::<T>::mint(origin.clone(), class_id, Self::u16_to_instance_id(i as u16) , admin.clone())?;
				//set the respective attributtes
				pallet_uniques::Pallet::<T>::set_attribute(origin.clone(), class_id, Some(Self::u16_to_instance_id(i as u16)),
				parent_id ,vector.clone())?;

				//emit event: fruniques created?
			}
			
			Ok(())
		}

		
		
	}

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		pub fn u16_to_instance_id(input: u16) -> T::InstanceId where <T as pallet_uniques::Config>::InstanceId: From<u16> {
			input.into()
		}

	}

}