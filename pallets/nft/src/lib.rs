#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

use codec::HasCompact;
use frame_support::{
	dispatch::{DispatchResult, DispatchResultWithPostInfo},
	ensure,
	traits::{tokens::nonfungibles::*, Currency, Get, NamedReservableCurrency},
	transactional, BoundedVec,
};
use frame_system::ensure_signed;

use primitives::{nft::NftPermission, ReserveIdentifier};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, One, StaticLookup, Zero},
	DispatchError,
};
pub use types::*;
use weights::WeightInfo;

mod benchmarking;
pub mod types;
pub mod weights;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type BoundedVecOfUnq<T> = BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>;
type ClassInfoOf<T> = ClassInfo<<T as Config>::ClassType, BoundedVecOfUnq<T>>;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::{pallet_prelude::*, traits::EnsureOrigin};
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_uniques::Config {
		/// Currency type for reserve balance.
		type Currency: NamedReservableCurrency<Self::AccountId, ReserveIdentifier = ReserveIdentifier>;
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Amount that must be reserved for each minted NFT
		#[pallet::constant]
		type TokenDeposit: Get<BalanceOf<Self>>;
		type WeightInfo: WeightInfo;
		type ProtocolOrigin: EnsureOrigin<Self::Origin>;

		type NftClassId: Member + Parameter + Default + Copy + HasCompact + AtLeast32BitUnsigned + Into<Self::ClassId>;
		type NftInstanceId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ AtLeast32BitUnsigned
			+ Into<Self::InstanceId>
			+ From<Self::InstanceId>;
		type ClassType: Member + Parameter + Default + Copy;
		type Permissions: NftPermission<Self::ClassType>;
	}

	/// Next available class ID.
	#[pallet::storage]
	#[pallet::getter(fn next_class_id)]
	pub(super) type NextClassId<T: Config> = StorageValue<_, T::NftClassId, ValueQuery>;

	/// Next available token ID.
	#[pallet::storage]
	#[pallet::getter(fn next_instance_id)]
	pub(super) type NextInstanceId<T: Config> =
		StorageMap<_, Twox64Concat, T::NftClassId, T::NftInstanceId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn classes)]
	/// Stores class info
	pub type Classes<T: Config> = StorageMap<_, Twox64Concat, T::NftClassId, ClassInfoOf<T>>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates an NFT class and sets its metadata
		///
		/// Parameters:
		/// - `class_id`: The identifier of the new asset class. This must not be currently in use.
		/// - `class_type`: The class type determines its purpose and usage
		/// - `metadata`: Arbitrary data about a class, e.g. IPFS hash
		#[pallet::weight(<T as Config>::WeightInfo::create_class())]
		#[transactional]
		pub fn create_class(
			origin: OriginFor<T>,
			class_type: T::ClassType,
			metadata: BoundedVecOfUnq<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(T::Permissions::can_create(&class_type), Error::<T>::NotPermitted);

			let (class_id, class_type) = Self::do_create_class(sender.clone(), Default::default(), metadata)?;

			Self::deposit_event(Event::ClassCreated(sender, class_id, class_type));

			Ok(())
		}

		/// Mints an NFT in the specified class
		/// Sets metadata and the royalty attribute
		///
		/// Parameters:
		/// - `class_id`: The class of the asset to be minted.
		#[pallet::weight(<T as Config>::WeightInfo::mint())]
		#[transactional]
		pub fn mint(origin: OriginFor<T>, class_id: T::NftClassId) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let class_type = Self::classes(class_id)
				.map(|c| c.class_type)
				.ok_or(Error::<T>::ClassUnknown)?;

			ensure!(T::Permissions::can_mint(&class_type), Error::<T>::NotPermitted);

			let instance_id = Self::do_mint(sender.clone(), class_id)?;

			Self::deposit_event(Event::InstanceMinted(sender, class_id, instance_id));

			Ok(())
		}

		/// Transfers NFT from account A to account B
		/// Only the ProtocolOrigin can send NFT to another account
		/// This is to prevent creating deposit burden for others
		///
		/// Parameters:
		/// - `class_id`: The class of the asset to be transferred.
		/// - `instance_id`: The instance of the asset to be transferred.
		/// - `dest`: The account to receive ownership of the asset.
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		#[transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			class_id: T::NftClassId,
			instance_id: T::NftInstanceId,
			dest: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let dest = T::Lookup::lookup(dest)?;

			let class_type = Self::classes(class_id)
				.map(|c| c.class_type)
				.ok_or(Error::<T>::ClassUnknown)?;

			ensure!(T::Permissions::can_transfer(&class_type), Error::<T>::NotPermitted);

			Self::do_transfer(class_id, instance_id, sender.clone(), dest.clone())?;

			Self::deposit_event(Event::InstanceTransferred(sender, dest, class_id, instance_id));

			Ok(())
		}

		/// Removes a token from existence
		///
		/// Parameters:
		/// - `class_id`: The class of the asset to be burned.
		/// - `instance_id`: The instance of the asset to be burned.
		#[pallet::weight(<T as Config>::WeightInfo::burn())]
		#[transactional]
		pub fn burn(origin: OriginFor<T>, class_id: T::NftClassId, instance_id: T::NftInstanceId) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let class_type = Self::classes(class_id)
				.map(|c| c.class_type)
				.ok_or(Error::<T>::ClassUnknown)?;

			ensure!(T::Permissions::can_burn(&class_type), Error::<T>::NotPermitted);

			Self::do_burn(sender.clone(), class_id, instance_id)?;

			Self::deposit_event(Event::InstanceBurned(sender, class_id, instance_id));

			Ok(())
		}

		/// Removes a class from existence
		///
		/// Parameters:
		/// - `class_id`: The identifier of the asset class to be destroyed.
		#[pallet::weight(<T as Config>::WeightInfo::destroy_class())]
		#[transactional]
		pub fn destroy_class(origin: OriginFor<T>, class_id: T::NftClassId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let class_type = Self::classes(class_id)
				.map(|c| c.class_type)
				.ok_or(Error::<T>::ClassUnknown)?;

			ensure!(T::Permissions::can_destroy(&class_type), Error::<T>::NotPermitted);

			Self::do_destroy_class(class_id)?;

			Self::deposit_event(Event::ClassDestroyed(sender, class_id));

			Ok(().into())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A class was created \[sender, class_id, class_type\]
		ClassCreated(T::AccountId, T::NftClassId, T::ClassType),
		/// An instance was minted \[class_type, sender, class_id, instance_id\]
		InstanceMinted(T::AccountId, T::NftClassId, T::NftInstanceId),
		/// An instance was transferred \[from, to, class_id, instance_id\]
		InstanceTransferred(T::AccountId, T::AccountId, T::NftClassId, T::NftInstanceId),
		/// An instance was burned \[sender, class_id, instance_id\]
		InstanceBurned(T::AccountId, T::NftClassId, T::NftInstanceId),
		/// A class was destroyed \[sender, class_id\]
		ClassDestroyed(T::AccountId, T::NftClassId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Count of instances overflown
		NoAvailableInstanceId,
		/// Count of classes overflown
		NoAvailableClassId,
		/// Class still contains minted tokens
		TokenClassNotEmpty,
		/// Class does not exist
		ClassUnknown,
		/// Shares has to be set for liquidity mining
		SharesNotSet,
		/// Operation not permitted
		NotPermitted,
	}
}

impl<T: Config> Pallet<T> {
	pub fn get_next_class_id() -> Result<T::NftClassId, Error<T>> {
		NextClassId::<T>::try_mutate(|id| {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableClassId)?;
			Ok(current_id)
		})
	}

	pub fn get_next_instance_id(class_id: T::NftClassId) -> Result<T::NftInstanceId, Error<T>> {
		NextInstanceId::<T>::try_mutate(class_id, |id| {
			let current_id = *id;
			*id = id.checked_add(&One::one()).ok_or(Error::<T>::NoAvailableInstanceId)?;
			Ok(current_id)
		})
	}

	pub fn class_owner(class_id: T::NftClassId) -> Option<T::AccountId> {
		pallet_uniques::Pallet::<T>::class_owner(&class_id.into())
	}

	pub fn instance_owner(class_id: T::NftClassId, instance_id: T::NftInstanceId) -> Option<T::AccountId> {
		pallet_uniques::Pallet::<T>::owner(class_id.into(), instance_id.into())
	}

	pub fn do_create_class(
		owner: T::AccountId,
		class_type: T::ClassType,
		metadata: BoundedVecOfUnq<T>,
	) -> Result<(T::NftClassId, T::ClassType), DispatchError> {
		let class_id = Self::get_next_class_id()?;

		let deposit_info = match T::Permissions::has_deposit(&class_type) {
			false => (Zero::zero(), true),
			true => (T::ClassDeposit::get(), false),
		};

		pallet_uniques::Pallet::<T>::do_create_class(
			class_id.into(),
			owner.clone(),
			owner.clone(),
			deposit_info.0,
			deposit_info.1,
			pallet_uniques::Event::Created(class_id.into(), owner.clone(), owner),
		)?;

		Classes::<T>::insert(class_id, ClassInfo { class_type, metadata });

		Ok((class_id, class_type))
	}

	pub fn do_mint(owner: T::AccountId, class_id: T::NftClassId) -> Result<T::NftInstanceId, DispatchError> {
		let instance_id = Self::get_next_instance_id(class_id)?;

		pallet_uniques::Pallet::<T>::do_mint(class_id.into(), instance_id.into(), owner, |_details| Ok(()))?;

		Ok(instance_id)
	}

	pub fn do_transfer(
		class_id: T::NftClassId,
		instance_id: T::NftInstanceId,
		from: T::AccountId,
		to: T::AccountId,
	) -> DispatchResult {
		if from == to {
			return Ok(());
		}

		pallet_uniques::Pallet::<T>::do_transfer(
			class_id.into(),
			instance_id.into(),
			to,
			|_class_details, instance_details| {
				let is_permitted = instance_details.owner == from;
				ensure!(is_permitted, Error::<T>::NotPermitted);
				Ok(())
			},
		)
	}

	pub fn do_burn(owner: T::AccountId, class_id: T::NftClassId, instance_id: T::NftInstanceId) -> DispatchResult {
		pallet_uniques::Pallet::<T>::do_burn(
			class_id.into(),
			instance_id.into(),
			|_class_details, instance_details| {
				let is_permitted = instance_details.owner == owner;
				ensure!(is_permitted, Error::<T>::NotPermitted);
				Ok(())
			},
		)
	}

	pub fn do_destroy_class(class_id: T::NftClassId) -> DispatchResultWithPostInfo {
		let witness =
			pallet_uniques::Pallet::<T>::get_destroy_witness(&class_id.into()).ok_or(Error::<T>::ClassUnknown)?;

		ensure!(witness.instances == 0u32, Error::<T>::TokenClassNotEmpty);
		pallet_uniques::Pallet::<T>::do_destroy_class(class_id.into(), witness, None)?;
		Classes::<T>::remove(class_id);
		Ok(().into())
	}
}
