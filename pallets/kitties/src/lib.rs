#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, traits::Randomness, ensure};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
use sp_io::hashing::blake2_128;
use sp_runtime::DispatchError;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type KittyIndex = u32;

#[derive(Encode, Decode, Clone)]
pub struct Kitty {
    pub dna: [u8; 16],
    pub parents: Option<[KittyIndex; 2]>,
    pub mates: Vec<KittyIndex>,
    pub brothers: Vec<KittyIndex>,
    pub children: Vec<KittyIndex>
}
// pub struct Kitty([u8; 16]);

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as KittiesModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count): KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>;
        pub WhosKitties get(fn whos_kitties): map hasher(blake2_128_concat) T::AccountId => Option<Kitty>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
        Created(AccountId, KittyIndex),
        Transfered(AccountId, AccountId, KittyIndex),
        Breeded(AccountId, KittyIndex),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
        ParentsTheSame,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;		
        
        #[weight = 0]
		pub fn create(origin) {
            let sender = ensure_signed(origin)?;
            let kitty_id = Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);
            let kitty = Kitty{
                dna: dna,
                parents: None,
                mates: Vec::new(),
                brothers: Vec::new(),
                children: Vec::new(),
            };
            Self::insert_kitties(&sender, kitty_id, &kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
        }
        
        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_id: KittyIndex) {
            let sender = ensure_signed(origin)?;
            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transfered(sender, to, kitty_id));
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) {
            let sender = ensure_signed(origin)?;
            let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;   // ??what is fn get, can I use Kitties
            let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::ParentsTheSame);

            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2);
            Self::deposit_event(RawEvent::Breeded(sender, new_kitty_id.ok().unwrap()));
        }
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
    (selector & dna1) | (!selector & dna2)
}

impl<T: Trait> Module<T> {
    fn next_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }
        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );
        payload.using_encoded(blake2_128)
    }

    fn insert_kitties(owner: &T::AccountId, kitty_id: KittyIndex, kitty: &Kitty) {
        Kitties::insert(kitty_id, kitty);
        KittiesCount::put(kitty_id + 1);
        KittyOwners::<T>::insert(kitty_id, owner);
        <WhosKitties<T>>::insert(owner, kitty);
    }

    fn do_breed(sender: &T::AccountId, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> sp_std::result::Result<KittyIndex, DispatchError> {
        let mut kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;   // ??what is fn get, can I use Kitties
        let mut kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        let kitty_id = Self::next_kitty_id()?;
        
        let kitty1_dna = kitty1.dna;
        let kitty2_dna = kitty2.dna;
        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }
        // get brothers
        let mut brothers = kitty1.children.clone();

        for child in &kitty2.children {
            let mut exist = false;
            for child2 in &brothers {
                if *child2 == *child {
                    exist = true;
                }
            }
            if !exist {
                brothers.push(*child);
            }
        }

        let kitty = Kitty{
            dna: new_dna,
            parents: Some([kitty_id_1, kitty_id_2]),
            mates: Vec::new(),
            brothers: brothers.clone(),
            children: Vec::new(),
        };
        Self::insert_kitties(sender, kitty_id, &kitty);

        // update brothers
        for brother in &brothers {
            let mut brother_kitty = Self::kitties(brother).ok_or(Error::<T>::InvalidKittyId)?;
            brother_kitty.brothers.push(kitty_id);
            Kitties::insert(brother, brother_kitty);
        }

        // update parent
        let mut exist = false;
        for mate in &kitty1.mates {
            if (*mate == kitty_id_2) {
                exist = true;
            }
        }
        if !exist {
            kitty1.mates.push(kitty_id_2);
            kitty2.mates.push(kitty_id_1);
        }

        kitty1.children.push(kitty_id);
        kitty2.children.push(kitty_id);

        Kitties::insert(kitty_id_1, kitty1);
        Kitties::insert(kitty_id_2, kitty2);

        Ok(kitty_id)
    }
}
