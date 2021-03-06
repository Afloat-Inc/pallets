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
	use frame_support::{
		pallet_prelude::*,
		traits::tokens::nonfungibles::{Inspect, InspectEnumerable},
		BoundedVec,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::string::String;
	use scale_info::prelude::vec::Vec;
	use sp_runtime::{traits::StaticLookup, Permill};
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// A frunique and asset class were succesfully created!
		FruniqueCreated(T::AccountId, T::AccountId, T::CollectionId, T::ItemId),
		// A frunique/unique was succesfully divided!
		FruniqueDivided(T::AccountId, T::AccountId, T::CollectionId, T::ItemId),
		// Counter should work?
		FruniqueCounter(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		NoPermission,
		StorageOverflow,
		NotYetImplemented,
		// Too many fruniques were minted
		FruniqueCntOverflow,
		// The asset_id is not linked to a frunique or it doesn't exists
		NotAFrunique,
	}

	#[pallet::storage]
	#[pallet::getter(fn frunique_cnt)]
	/// Keeps track of the number of Kitties in existence.
	pub(super) type FruniqueCnt<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> 
	where 
		T: pallet_uniques::Config<CollectionId = u32, ItemId = u32>,
	{
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
			class_id: T::CollectionId,
			instance_id: T::ItemId,
			numeric_value: Option<Permill>,
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;

			let new_cnt =
				Self::frunique_cnt().checked_add(1).ok_or(<Error<T>>::FruniqueCntOverflow)?;
			// create an NFT in the uniques pallet
			pallet_uniques::Pallet::<T>::create(origin.clone(), class_id.clone(), admin.clone())?;
			pallet_uniques::Pallet::<T>::mint(
				origin.clone(),
				class_id.clone(),
				instance_id.clone(),
				admin.clone(),
			)?;
			<FruniqueCnt<T>>::put(new_cnt);
			if let Some(n) = numeric_value {
				let num_value_key =
					BoundedVec::<u8, T::KeyLimit>::try_from(r#"num_value"#.encode())
						.expect("Error on encoding the numeric value key to BoundedVec");
				let num_value = BoundedVec::<u8, T::ValueLimit>::try_from(n.encode())
					.expect("Error on encoding the numeric value to BoundedVec");
				pallet_uniques::Pallet::<T>::set_attribute(
					origin.clone(),
					class_id,
					Some(instance_id),
					num_value_key,
					num_value,
				)?;
			}
			let admin = T::Lookup::lookup(admin)?;
			Self::deposit_event(Event::FruniqueCreated(owner, admin, class_id, instance_id));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn instance_exists(
			_origin: OriginFor<T>,
			_class_id: T::CollectionId,
			_instance_id: T::ItemId,
		) -> DispatchResult {
			// Always returns an empty iterator?
			//let instances = pallet_uniques::Pallet::<T>::;
			//println!("Instances found in class {:?}",instances.count());
			//log::info!("Instances found in class {:?}", instances.count());
			//println!("\tIterator? {}",instances.count());
			//Self::deposit_event(Event::FruniqueCounter(instances.count().try_into().unwrap()  ));
			//instances.into_iter().for_each(|f| println!("\tInstance:{:?}",f));
			Ok(())
		}

		/// ## NFT Division
		///
		/// PD: the Key/value length limits are ihnerited from the uniques pallet,
		/// so they're not explicitly declared on this pallet
		///
		///
		/// ### Boilerplate parameters:
		///
		/// - `admin`: The admin of this class of assets. The admin is the initial address of each
		/// member of the asset class's admin team.
		///
		/// ### Parameters needed in order to divide a unique:
		/// - `class_id`: The type of NFT that the function will create, categorized by numbers.
		/// - `instance_id`: The unique identifier of the instance to be fractioned/divided
		/// - `_inherit_attrs`: Doesn't do anything fow now. Intended to enable the attribute inheritance
		///
		#[pallet::weight(10_000 + T::DbWeight::get().writes(4))]
		pub fn spawn(
			origin: OriginFor<T>,
			class_id: T::CollectionId,
			instance_id: T::ItemId,
			inherit_attrs: bool,
			_p: Permill,
			admin: <T::Lookup as sp_runtime::traits::StaticLookup>::Source,
		) -> DispatchResult where <T as pallet_uniques::Config>::ItemId: From<u32>{
			// Boilerplate (setup, conversions, ensure_signed)
			let owner = ensure_signed(origin.clone())?;
			let enconded_id = instance_id.encode();
			let new_cnt = Self::frunique_cnt().checked_add(1)
				.ok_or(<Error<T>>::FruniqueCntOverflow)?;
			// TODO: Check if the instance_id exists?
			let parent_id_key = BoundedVec::<u8, T::KeyLimit>::try_from(r#"parent_id"#.encode())
				.expect("Error on encoding the parent_id key to BoundedVec");
			let mut parent_id_val: BoundedVec<u8, T::ValueLimit>;
			// Instance n number of nfts (with the respective parentId)
			let new_instance_id = Self::frunique_cnt().try_into().unwrap();
			// Mint a unique
			pallet_uniques::Pallet::<T>::mint(origin.clone(), class_id, new_instance_id, admin.clone())?;
			// Set the respective attributtes
			if inherit_attrs {
				// TODO: Check all the parent's instance attributes
				// Let's start with some static attributes check (does parent_id exist?)
				// Options:
				// 1.- Constant &str array containing the keys
				// 2.- Set a whole single attribute as bytes, containing all the fruniques metadata (parent_id, numerical_value, etc..)
				// 3.- Keep our own metadata (or whole nfts) storage on
				// 3.1.- Consider the 3 above but with interfaces/traits
				// I'm assuming doing it via scripts on the front-end isn't viable option
				let parent_id =
					Self::get_nft_attribute(&class_id, &instance_id, &"parent_id".encode());
				if parent_id.len() > 0 {
					parent_id_val = parent_id;
				} else {
					parent_id_val = BoundedVec::<u8, T::ValueLimit>::try_from(enconded_id.clone())
						.expect("Error on converting the parent_id to BoundedVec");
				}
				let num_value =
					Self::get_nft_attribute(&class_id, &instance_id, &"num_value".encode());
				if num_value.len() > 0 {
					let num_value_key =
						BoundedVec::<u8, T::KeyLimit>::try_from(r#"num_value"#.encode())
							.expect("Error on encoding the num_value key to BoundedVec");
					// TODO: Call bytes_to_u32 & divide the numeric value before setting it
					pallet_uniques::Pallet::<T>::set_attribute(
						origin.clone(), class_id,
						Some(Self::u32_to_instance_id(new_instance_id)),
						num_value_key ,num_value
					)?;
				}
			if let Some(parent_attr) = pallet_uniques::Pallet::<T>::attribute(&class_id, &instance_id,&"parent_id".encode() ){
				//println!(" Instance number {:?} parent_id (parent's parent): {:#?}", instance_id, Self::bytes_to_u32( parent_attr.clone() ));
				parent_id_val= BoundedVec::<u8,T::ValueLimit>::try_from(parent_attr)
					.expect("Error on converting the parent_id to BoundedVec");
			}else{
				//println!("The parent doesn't have a parent_id");
				parent_id_val= BoundedVec::<u8,T::ValueLimit>::try_from(enconded_id)
				.expect("Error on converting the parent_id to BoundedVec");
			}
			} else {
				parent_id_val = BoundedVec::<u8, T::ValueLimit>::try_from(enconded_id)
					.expect("Error on converting the parent_id to BoundedVec");
			}
			// (for encoding reasons the parentId is stored on hex format as a secondary side-effect, I hope it's not too much of a problem).
			pallet_uniques::Pallet::<T>::set_attribute(origin.clone(), class_id, Some(Self::u32_to_instance_id(new_instance_id)),
			parent_id_key ,parent_id_val)?;
			let _e = Self::instance_exists(origin, class_id, instance_id);
			//let final_test = pallet_uniques::Pallet::<T>::attribute(&class_id, &Self::u16_to_instance_id(new_instance_id ), &r#"parent_id"#.encode() );
			//println!("The parent_id of {} is now {:?}",new_instance_id, Self::bytes_to_u32(final_test.unwrap()) );
			<FruniqueCnt<T>>::put(new_cnt);
			// TODO: set the divided value attribute. Numbers, divisions and floating points are giving a lot of problems
			let admin = T::Lookup::lookup(admin)?;
			Self::deposit_event(Event::FruniqueDivided(owner, admin, class_id, instance_id));
			// Freeze the nft to prevent trading it? Burn it? Not clear, so nothing at the moment
			Ok(())
		}

	}

	impl<T: Config> Pallet<T> 
	where
	T: pallet_uniques::Config<CollectionId = u32, ItemId = u32>,
	{
		pub fn u32_to_instance_id(input: u32) -> T::ItemId where <T as pallet_uniques::Config>::ItemId: From<u32> {
		 	T::ItemId::from(input)
		}

		pub fn bytes_to_string(input: Vec<u8>) -> String {
			let mut s = String::default();
			for x in input {
				//let c: char = x.into();
				s.push(x as char);
			}
			s
		}
		/// Helper function for printing purposes
		pub fn bytes_to_u32(input: Vec<u8>) -> u32 {
			u32::from_ne_bytes(input.try_into().unwrap())
		}

		//get uniques attribute?
		pub fn get_nft_attribute(
			class_id: &T::CollectionId,
			instance_id: &T::ItemId,
			key: &Vec<u8>,
		) -> BoundedVec<u8, T::ValueLimit> {
			if let Some(a) = pallet_uniques::Pallet::<T>::attribute(class_id, instance_id, key) {
				return BoundedVec::<u8, T::ValueLimit>::try_from(a)
					.expect("Error on converting the attribute to BoundedVec");
			}
			BoundedVec::<u8, T::ValueLimit>::default()
		}
	}
}
