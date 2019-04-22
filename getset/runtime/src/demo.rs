use support::{decl_storage, decl_module, StorageValue, dispatch::Result};
use system::ensure_signed;

pub trait Trait: system::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as Counter {
        Count get(count): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn incrememnt_value(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            let Currentcount = Self::count();

            let newCount = Currentcount.wrapping_add(value);
            
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