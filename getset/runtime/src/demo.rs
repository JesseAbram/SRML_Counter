// ACTION: Update StorageValue to StorageMap to support mappings
use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

// ACTION: Update this to use `balances::Trait` to access T::AccountId
pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Counter {
        // ACTION: Update this storage item to be a `map` from `T::AccountId` to `u64`
        Count get(count): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn incrememnt_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            let Currentcount = Self::count();

            let newCount = Currentcount.wrapping_add(value);
            
            // ACTION: Update this to `insert()` the key/value pair (sender, value)
            <Count<T>>::put(newCount);

            Ok(())
        }

      fn decrement_value(origin, value: u64) -> Result {
          let sender = ensure_signed(origin)?;

          let Currentcount = Self::count();

          let newCount = Currentcount.wrapping_add(value);

          <Count<T>>::put(newCount);

            Ok(())
      }

    }
}